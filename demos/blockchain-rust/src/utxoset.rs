#![allow(non_snake_case)]
#![allow(unused)]
//! unspend transaction output set

use super::*;
use crate::block::*;
use crate::blockchain::*;
use crate::transaction::*;
use bincode::{deserialize, serialize};
use sled;
use std::collections::HashMap;

/// UTXOSet represents UTXO set
/// 设计 UTXO 的意义: 使代码的语义更加明确，提高代码的模块化，并可能提供一些特定于 UTXO 集的操作和数据
pub struct UTXOSet {
    pub blockchain: Blockchain,
}

impl UTXOSet {
    /// FindUnspentTransactions returns a list of transactions containing unspent outputs
    /// 在 UTXO 集合中查找可用于此交易的 TXOutputs(struct TXOutputs{outputs: Vec<TXOutput>,}) , 
    /// 返回的是一个 accumulated 和 unspent_outputs 的元组。
    /// 此方法会遍历所有存储在数据库中的未花费交易输出，找到一个满足以下条件的输出集合：
    ///     它们被指定的公钥(pub_key_hash) 锁定，且它们的总价值 > amount 参数。这些输出可以用于创建一个新的交易。
    /// 当 accumulated 总价值 > amount 时，即停止查找, 直接返回当前找到的 unspent_outputs 进行后续的花费
    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32,
    ) -> Result<(i32, HashMap<String, Vec<i32>>)> {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated = 0;

        let db = sled::open("data/utxos")?;
        for kv in db.into_iter() { // db 是个可迭代 obj
            let (k, v) = kv?;
            let txid = String::from_utf8(k.to_vec())?;
            let outs: TXOutputs = deserialize(&v.to_vec())?;

            for out_idx in 0..outs.outputs.len() {
                // 当 accumulated 总价值 > amount 时，停止查找, 直接利用 unspent_outputs 进行后续的花费
                if outs.outputs[out_idx].is_locked_with_key(pub_key_hash) && accumulated < amount {
                    accumulated += outs.outputs[out_idx].value;
                    match unspent_outputs.get_mut(&txid) {
                        Some(v) => v.push(out_idx as i32),
                        None => {
                            unspent_outputs.insert(txid.clone(), vec![out_idx as i32]);
                        }
                    }
                }
            }
        }

        Ok((accumulated, unspent_outputs))
    }

    /// FindUTXO finds UTXO for a public key hash
    /// 找到所有被指定公钥哈希锁定的未花费交易输出。这些输出代表了 pub_key_hash 这个地址的 "余额"
    /// ( Tips: find_UTXO 和上面的代码有部分重合, 但是如果想复用 find_UTXO 函数, 
    ///      需要对其进行一些修改，比如添加一个累计值参数，并在函数中进行价值的计算和比较。
    ///      但这样可能会破坏 find_UTXO 函数的原有功能，所以我个人不建议这样做。)
    pub fn find_UTXO(&self, pub_key_hash: &[u8]) -> Result<TXOutputs> {
        let mut utxos = TXOutputs {
            outputs: Vec::new(),
        };
        let db = sled::open("data/utxos")?;

        for kv in db.iter() {
            let (_, v) = kv?;
            let outs: TXOutputs = deserialize(&v.to_vec())?;

            for out in outs.outputs {
                if out.is_locked_with_key(pub_key_hash) {
                    utxos.outputs.push(out.clone())
                }
            }
        }

        Ok(utxos)
    }

    /// CountTransactions returns the number of transactions in the UTXO set
    /// 返回数据库中存储的所有地址的所有未花费交易输出的数量
    pub fn count_transactions(&self) -> Result<i32> {
        let mut counter = 0;
        let db = sled::open("data/utxos")?;
        for kv in db.iter() {
            kv?;
            counter += 1;
        }
        Ok(counter)
    }

    /// Reindex rebuilds the UTXO set
    /// 重建UTXO集合。它首先删除数据库中的所有数据，然后通过查找区块链中的所有未花费交易输出来重新填充数据库。
    pub fn reindex(&self) -> Result<()> {
        std::fs::remove_dir_all("data/utxos").ok();
        let db = sled::open("data/utxos")?;

        // 这个 find_UTXO() 是定义在 Blockchain 上的, 和上面不同
        let utxos = self.blockchain.find_UTXO();

        for (txid, outs) in utxos {
            db.insert(txid.as_bytes(), serialize(&outs)?)?;
        }

        Ok(())
    }

    /// Update updates the UTXO set with transactions from the Block
    ///
    /// The Block is considered to be the tip of a blockchain
    /// 此方法会用区块中的交易来更新UTXO集合。对于每个交易，它会从数据库中移除已经被花费的输出，并添加新的未花费输出。
    pub fn update(&self, block: &Block) -> Result<()> {
        let db = sled::open("data/utxos")?;

        for tx in block.get_transaction() {
            if !tx.is_coinbase() {
                for vin in &tx.vin {
                    let mut update_outputs = TXOutputs {
                        outputs: Vec::new(),
                    };
                    let outs: TXOutputs = deserialize(&db.get(&vin.txid)?.unwrap().to_vec())?;
                    for out_idx in 0..outs.outputs.len() {
                        if out_idx != vin.vout as usize {
                            update_outputs.outputs.push(outs.outputs[out_idx].clone());
                        }
                    }

                    if update_outputs.outputs.is_empty() {
                        db.remove(&vin.txid)?;
                    } else {
                        db.insert(vin.txid.as_bytes(), serialize(&update_outputs)?)?;
                    }
                }
            }

            let mut new_outputs = TXOutputs {
                outputs: Vec::new(),
            };
            for out in &tx.vout {
                new_outputs.outputs.push(out.clone());
            }

            db.insert(tx.id.as_bytes(), serialize(&new_outputs)?)?;
        }
        Ok(())
    }
}
