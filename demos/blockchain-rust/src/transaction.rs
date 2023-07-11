#![allow(non_snake_case)]
#![allow(unused)]
//! transaction implement

use super::*;
use crate::utxoset::*;
use crate::wallets::*;
use bincode::serialize;
use bitcoincash_addr::Address;
use crypto::digest::Digest;
use crypto::ed25519;
use crypto::sha2::Sha256;
use failure::format_err;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::{debug, error, info};
use rand::rngs::OsRng;

const SUBSIDY: i32 = 10;

/// TXInput represents a transaction input
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    pub txid: String, 
    pub vout: i32,  // index 序号, -1 表示 coinbase tx, cbtx
    pub signature: Vec<u8>,  // 签名, 其他人可以通过 Alice 的 pub_key 和 整个消息体, 来验证 Alice 的 Signature 是否有效.
    pub pub_key: Vec<u8>,  // 消费者即 Alice 的公钥
}

/// TXOutput represents a transaction output
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,            // 金额 amount
    pub pub_key_hash: Vec<u8>,  // 标识了谁能花费这笔 TXOutput
}

// TXOutputs collects TXOutput
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutputs {
    pub outputs: Vec<TXOutput>,  // UTXOs 集合
}

/// Transaction represents a Bitcoin transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl Transaction {
    /// NewUTXOTransaction creates a new transaction
    /// 它从一个给定的钱包创建一个新的 UTXO（未消费的交易输出）交易，发送指定数量的资金到指定的地址，并返回创建的交易
    pub fn new_UTXO(wallet: &Wallet, to: &str, amount: i32, utxo: &UTXOSet) -> Result<Transaction> {
        info!(
            "new UTXO Transaction from: {} to: {}",
            wallet.get_address(), to
        );
        let mut vin = Vec::new();

        // 复制钱包的公钥后, 创建公钥哈希
        let mut pub_key_hash = wallet.public_key.clone();
        hash_pub_key(&mut pub_key_hash);

        // 在该公钥地址的 UTXO 集合中迭代累计查找可用于此交易的 TXOutput 。返回一个包含总额和刚刚好覆盖该笔交易的映射的元组。
        // ↑ 上面说的不太好懂, 举个例子: 某笔交易需要 10 BTC
        // 遍历 msg.sender 的 UTXO outputs, 第一笔 3 BTC,第二笔 5 BTC, 第三笔 4 BTC,此时 3+5+4=12 刚刚好也是第一次 < 10
        // 这 3 笔 TXOutputs 就会用来全部花费掉以执行本次交易: 转给 receiver 10 BTC ; 转给 msg.sender 2 BTC.
        let acc_v = utxo.find_spendable_outputs(&pub_key_hash, amount)?;

        // 如果交易金额 > 找到所有总额，则记录 Not Enough balance 错误并返回错误。
        if acc_v.0 < amount { 
            error!("Not Enough balance");
            return Err(format_err!(
                "Not Enough balance: current balance {}",
                acc_v.0
            ));
        }
 
        // 对于每个待被花费的 TXOutput 和其对应的输出索引，创建一个新的 TXInput(交易输入) 并将其添加到交易输入列表中。
        for tx in acc_v.1 {
            for out in tx.1 {  // 迭代 Vec<TXOutput>
                let input = TXInput {
                    txid: tx.0.clone(),
                    vout: out,
                    signature: Vec::new(),
                    pub_key: wallet.public_key.clone(),
                };
                vin.push(input);
            }
        }
        
        // 创建交易输出: 
        // 首先，创建一个新的交易输出，其金额等于交易金额，收款人是目标地址。
        // 如果找到的总额大于交易金额（意味着需要找零），则创建一个新的交易输出，其金额等于找到的总额减去交易金额，收款人是发送者的地址。
        let mut tx_outs = vec![TXOutput::new(amount, to.to_string())?];
        if acc_v.0 > amount {  // 需要找零给自己
            tx_outs.push(TXOutput::new(acc_v.0 - amount, wallet.get_address())?)
        }
        
        // 创建新的交易，其中包括已创建的交易输入和输出，然后计算并设置交易ID，
        let mut tx = Transaction {
            id: String::new(),
            vin,
            vout: tx_outs,
        };
        tx.id = tx.hash()?;
        utxo.blockchain
            .sign_transacton(&mut tx, &wallet.secret_key)?; // 最后对交易进行签名。
        Ok(tx)
    }

    /// NewCoinbaseTX creates a new coinbase transaction
    /// CoinbaseTX 不需要通过挖矿来创建。实际上，coinbase 交易就是矿工因成功挖掘出新的区块而得到的奖励，这个奖励直接发给矿工，而不是从其他交易中取得
    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction> {
        info!("new coinbase Transaction to: {}", to);
        let mut key: [u8; 32] = [0; 32];
        if data.is_empty() {
            let mut rand = OsRng::default();
            rand.fill_bytes(&mut key);
            data = format!("Reward to '{}'", to);  // 挖矿奖励
        }
        let mut pub_key = Vec::from(data.as_bytes());
        pub_key.append(&mut Vec::from(key));

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![TXInput {
                txid: String::new(),
                vout: -1,
                signature: Vec::new(),
                pub_key,
            }],
            vout: vec![TXOutput::new(SUBSIDY, to)?], // 10 个 BTC 津贴作为 mine Reward.
        };
        tx.id = tx.hash()?;
        Ok(tx)
    }

    /// IsCoinbase checks whether the transaction is coinbase
    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].txid.is_empty() && self.vin[0].vout == -1
    }

    /// Verify verifies signatures of Transaction inputs
    /// 验证交易输入的签名, 它将当前交易中的每个输入和先前交易相关联，并确认 `输入的签名与先前交易的输出一致`
    pub fn verify(&self, prev_TXs: HashMap<String, Transaction>) -> Result<bool> {
        if self.is_coinbase() {
            return Ok(true); // 它没有先前的输入，因此可以立即确认签名是有效的。
        }

        // 检查该输入是否有一个有效的相关联的先前交易
        for vin in &self.vin { 
            if prev_TXs.get(&vin.txid).unwrap().id.is_empty() {
                return Err(format_err!("ERROR: Previous transaction is not correct"));
            }
        }

        // 创建当前交易的一个“修剪版”副本，它移除了输入的签名和公钥，以便我们可以重新计算它的哈希值
        let mut tx_copy = self.trim_copy();

        for in_id in 0..self.vin.len() {
            let prev_Tx = prev_TXs.get(&self.vin[in_id].txid).unwrap();  // 找到该输入对应的先前的交易
            tx_copy.vin[in_id].signature.clear(); // 清除修剪版交易副本中此输入的签名
            tx_copy.vin[in_id].pub_key = prev_Tx.vout[self.vin[in_id].vout as usize]
                .pub_key_hash
                .clone();  // 将此输入在修剪版交易副本中的公钥替换为先前交易对应输出的公钥哈希。
            tx_copy.id = tx_copy.hash()?; // 计算修剪版交易副本的新哈希值
            tx_copy.vin[in_id].pub_key = Vec::new();  // 清除修剪版交易副本中此输入的公钥

            // 使用Ed25519签名方案验证原始交易中此输入的签名。如果签名无效，返回Ok(false)，表示交易验证失败。
            // fn sign() 中提到: &tx_copy.id = tx_copy.hash()?, 是消息体的 hash 值.
            // 利用 Alice 的公钥 和 整个消息体, 我们可以验证 Alice 的 Signature 是否有效.
            if !ed25519::verify(  
                &tx_copy.id.as_bytes(),
                &self.vin[in_id].pub_key,
                &self.vin[in_id].signature,
            ) {
                return Ok(false);
            }
        }
        // 检查了所有输入并且它们的签名都有效，那么我们返回Ok(true)，表示交易验证成功。
        Ok(true)
    }

    /// Sign signs each input of a Transaction
    /// 使用私钥签名交易 (在 new_UTXO() 里调用 sign_transacton 调用 本函数. (sign_transacton 在 blockchain.rs 中))
    /// 为什么要创建 copy? 计算整个交易的哈希, 如果我们直接在原始交易上操作，就需要临时清空或修改输入中的 signature 和 pub_key 字段，然后进行哈希运算。这样做会修改原始交易，而我们并不希望这样。
    /// 交易中的重要信息都需要被包含进去做 hash, 这样，如果交易中的任何部分发生改变，哈希值就会改变，导致签名无效。
    pub fn sign(
        &mut self,
        private_key: &[u8],
        prev_TXs: HashMap<String, Transaction>,
    ) -> Result<()> {
        if self.is_coinbase() {
            return Ok(());
        }

        for vin in &self.vin {
            if prev_TXs.get(&vin.txid).unwrap().id.is_empty() {
                return Err(format_err!("ERROR: Previous transaction is not correct"));
            }
        }

        let mut tx_copy = self.trim_copy();

        for in_id in 0..tx_copy.vin.len() {
            let prev_Tx = prev_TXs.get(&tx_copy.vin[in_id].txid).unwrap();
            tx_copy.vin[in_id].signature.clear();
            tx_copy.vin[in_id].pub_key = prev_Tx.vout[tx_copy.vin[in_id].vout as usize]
                .pub_key_hash
                .clone();
            tx_copy.id = tx_copy.hash()?;  // 将 struct 的 hash 结果存入其 id
            tx_copy.vin[in_id].pub_key = Vec::new();
            // 对消息用私钥签名, 后面的任何人都可以拿着 其公钥和消息进行验证.
            // 每个节点在接收到一个交易后，都会用发送者的公钥去验证交易的签名，以确保交易是真实有效的
            let signature = ed25519::signature(tx_copy.id.as_bytes(), private_key);
            self.vin[in_id].signature = signature.to_vec();
        }

        Ok(())
    }

    /// Hash returns the hash of the Transaction
    /// 为什么不要 id 字段:
    ///   当我们要计算交易的哈希值时，不应该包含原有的 id 字段，因为这个 id 字段本身就是由其他字段通过哈希计算得出的。
    ///   如果包含原有的 id 字段，就会引入不必要的复杂性，因为在我们重新计算哈希值（例如在签名或验证过程中）时，
    ///   id 字段本身是未知的或者变化的。
    ///   所以，通过创建一个新的对象副本并清除 id 字段，我们可以确保对其他所有字段进行哈希计算，从而获得一个反映这些字段内容的哈希值。在后续的签名和验证过程中，这个哈希值会被用作交易的唯一标识。
    pub fn hash(&self) -> Result<String> {
        let mut copy = self.clone();
        copy.id = String::new();
        let data = serialize(&copy)?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);  // 将 `Vec<u8>` 转为 `&[u8]` 切片
        Ok(hasher.result_str())
    }

    /// TrimmedCopy creates a trimmed copy of Transaction to be used in signing
    /// 先创建一个不包含签名(signature) 和 pub_key 的"修剪版"副本，计算它的哈希值，然后用这个哈希值生成签名
    fn trim_copy(&self) -> Transaction {
        let mut vin = Vec::new();
        let mut vout = Vec::new();

        for v in &self.vin {
            vin.push(TXInput {
                txid: v.txid.clone(),
                vout: v.vout.clone(),
                signature: Vec::new(),
                pub_key: Vec::new(),
            })
        }

        for v in &self.vout {
            vout.push(TXOutput {
                value: v.value,
                pub_key_hash: v.pub_key_hash.clone(),
            })
        }

        Transaction {
            id: self.id.clone(),
            vin,
            vout,
        }
    }
}

impl TXOutput {
    /// IsLockedWithKey checks if the output can be used by the owner of the pubkey
    /// 定义 TXOutput（交易输出）的方法 : 
    /// 在区块链交易中，锁定交易输出实际上就是将交易输出与一个特定的公钥哈希（也就是地址）相关联。
    /// 这个步骤确保了只有拥有相应私钥的用户才能在将来的交易中使用这个输出

    //检查当前输出是否被给定的公钥哈希锁定。如果当前输出的公钥哈希与给定的公钥哈希相等，那么返回true，否则返回false。
    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash == pub_key_hash
    }

    /// Lock signs the output
    /// 锁定当前的输出到一个特定的地址。它将给定地址解码为公钥哈希，然后将该公钥哈希设置为当前输出的公钥哈希。
    fn lock(&mut self, address: &str) -> Result<()> {
        let pub_key_hash = Address::decode(address).unwrap().body;
        debug!("lock: {}", address);
        self.pub_key_hash = pub_key_hash;
        Ok(())
    }

    /// 创建新的交易输出 - address 是 value将 要被锁定到的地址
    pub fn new(value: i32, address: String) -> Result<Self> {
        let mut txo = TXOutput {
            value,
            pub_key_hash: Vec::new(),  
        };
        txo.lock(&address)?;
        Ok(txo)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_signature() {
        let mut ws = Wallets::new().unwrap();
        let wa1 = ws.create_wallet();
        let w = ws.get_wallet(&wa1).unwrap().clone();
        ws.save_all().unwrap();
        drop(ws);

        let data = String::from("test");
        let tx = Transaction::new_coinbase(wa1, data).unwrap();
        assert!(tx.is_coinbase());

        let signature = ed25519::signature(tx.id.as_bytes(), &w.secret_key);
        assert!(ed25519::verify(tx.id.as_bytes(), &w.public_key, &signature));
    }
}