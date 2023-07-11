#![allow(non_snake_case)]
#![allow(unused)]
//! Blockchain

use super::*;
use crate::block::*;
use crate::transaction::*;
use bincode::{deserialize, serialize};
use failure::format_err;
use serde_json::map::IterMut;
use sled;
use std::collections::HashMap;
use log::{debug, info};

// 这是当时《泰晤士报》的一条新闻标题，用来证明这个区块是在2009年1月3日之后创建的
// 在比特币网络中，创世区块在2009年由中本聪(Satoshi)创建，并且从那时起就固定在比特币区块链的开始处.
const GENESIS_COINBASE_DATA: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

/// Blockchain implements interactions with a DB
#[derive(Debug)]
pub struct Blockchain {
    // 字段前的 pub: 即使 Blockchain 已经声明 pub, 如果字段 tip/db  不声明, 也仍是默认私有的..
    pub tip: String,  // 存放 lasthash 即最后一个区块的哈希, 
    pub db: sled::Db, // 区块链 db 中的每一个区块都是通过其哈希来索引的，同时 db 上还有一个特殊的 key "LAST" 用来存储最后一个区块的哈希
}

/// BlockchainIterator is used to iterate over blockchain blocks   遍历区块链
pub struct BlockchainIterator<'a> {
    current_hash: String,
    bc: &'a Blockchain,  // bc 是指向 Blockchain 的引用, lifetime 标注告诉编译器: 引用的有效期至少和 'a 一样长
}

impl Blockchain {
    /// NewBlockchain creates a new Blockchain db
    pub fn new() -> Result<Blockchain> {
        info!("open blockchain");

        let db = sled::open("data/blocks")?;
        let hash = match db.get("LAST")? {
            Some(l) => l.to_vec(),
            None => Vec::new(),
        };
        
        let lasthash = if hash.is_empty() {
            info!("Create block database");  // no key "LAST"
            String::new()
        } else {
            info!("Found block database"); 
            String::from_utf8(hash.to_vec())?
        };
        Ok(Blockchain { tip: lasthash, db })
    }

    /// CreateBlockchain creates a new blockchain DB
    /// 化身中本聪, 创建比特币区块链! 第一步: 创建创始区块!
    /// 先清空本地数据库，创建一个新的 db 文件，并添加一个创世区块，该区块包含一个 coinbase 交易 (cbtx)
    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Creating new blockchain");

        std::fs::remove_dir_all("data/blocks").ok();
        let db = sled::open("data/blocks")?;
        debug!("Creating new block database");
        // GENESIS_COINBASE_DATA: 这是当时《泰晤士报》的一条新闻标题，用来证明这个区块是在2009年1月3日之后创建的
        // 在比特币网络中，创世区块在2009年由中本聪(Satoshi)创建，并且从那时起就固定在比特币区块链的开始处.
        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        let genesis: Block = Block::new_genesis_block(cbtx);  //化身中本聪, 创建创始区块!
        db.insert(genesis.get_hash(), serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;
        let bc = Blockchain {
            tip: genesis.get_hash(),
            db,
        };
        bc.db.flush()?;
        Ok(bc)
    }

    /// MineBlock mines a new block with the provided transactions
    /// 挖掘一个新的区块 —— 调用 new_block 调用 run_proof_of_work 不断循环, 直到挖到合适的 nonce
    pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> Result<Block> {
        info!("mine a new block");

        for tx in &transactions {  // 循环验证所有交易的有效性
            if !self.verify_transacton(tx)? {
                return Err(format_err!("ERROR: Invalid transaction"));
            }
        }

        let lasthash = self.db.get("LAST")?.unwrap();

        /// while 不断循环, 直到挖到合适的 nonce 为止
        let newblock = Block::new_block(
            transactions,
            String::from_utf8(lasthash.to_vec())?, // 将 'LAST' Block 作为 Prev
            self.get_best_height()? + 1,
        )?;
        self.db.insert(newblock.get_hash(), serialize(&newblock)?)?;
        self.db.insert("LAST", newblock.get_hash().as_bytes())?;
        self.db.flush()?;

        self.tip = newblock.get_hash(); // tip 字段: 最后一个区块的哈希，让它指向新的区块
        Ok(newblock)
    }

    /// 为 Blockchain 定义 iter() 方法, 辅助后面 Iterator 的实现.
    /// ps: Blockchain 没实现 Iterator trait，但它通过实现这个 iter() 方法 returns BlockchainIterator 迭代器对象。
    /// BlockchainIterator 迭代器对象实现了Iterator trait，因此可以在 for 循环中使用。
    pub fn iter(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.tip.clone(),
            bc: &self,
        }
    }

    /// FindUTXO finds and returns all unspent transaction outputs
    pub fn find_UTXO(&self) -> HashMap<String, TXOutputs> {
        let mut utxos: HashMap<String, TXOutputs> = HashMap::new(); // 存储了所有未花费的交易输出
        let mut spend_txos: HashMap<String, Vec<i32>> = HashMap::new(); // 存储了所有已经花费的输出

        // 迭代 整个区块链 db 的每个区块 & 每个区块中的每个交易
        for block in self.iter() {  // Blockchain 没实现 Iterator, 但是其 iter() 方法返回了 BlockchainIterator
            for tx in block.get_transaction() {
                // 对于交易的每个输出，我们检查它是否在 spend_txos 中，如果在，则表示这个输出已经被花费，
                // 我们跳过这个输出。否则，我们将这个输出添加到 utxos 中。
                // 这样，当我们处理完所有区块和所有交易后，utxos 中就存储了所有的 UTXOs。
                for index in 0..tx.vout.len() {
                    if let Some(ids) = spend_txos.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }

                    match utxos.get_mut(&tx.id) {
                        Some(v) => {
                            v.outputs.push(tx.vout[index].clone());
                        }
                        None => {
                            utxos.insert(
                                tx.id.clone(),
                                TXOutputs {
                                    outputs: vec![tx.vout[index].clone()],
                                },
                            );
                        }
                    }
                }

                if !tx.is_coinbase() {
                    for i in &tx.vin {
                        match spend_txos.get_mut(&i.txid) {
                            Some(v) => {
                                v.push(i.vout);
                            }
                            None => {
                                spend_txos.insert(i.txid.clone(), vec![i.vout]);
                            }
                        }
                    }
                }
            }
        }

        utxos
    }

    /// FindTransaction finds a transaction by its ID
    pub fn find_transacton(&self, id: &str) -> Result<Transaction> {
        for b in self.iter() {
            for tx in b.get_transaction() {
                if tx.id == id {
                    return Ok(tx.clone());
                }
            }
        }
        Err(format_err!("Transaction is not found"))
    }

    fn get_prev_TXs(&self, tx: &Transaction) -> Result<HashMap<String, Transaction>> {
        let mut prev_TXs = HashMap::new();
        for vin in &tx.vin {
            let prev_TX = self.find_transacton(&vin.txid)?;
            prev_TXs.insert(prev_TX.id.clone(), prev_TX);
        }
        Ok(prev_TXs)
    }

    /// SignTransaction signs inputs of a Transaction
    pub fn sign_transacton(&self, tx: &mut Transaction, private_key: &[u8]) -> Result<()> {
        let prev_TXs = self.get_prev_TXs(tx)?;
        tx.sign(private_key, prev_TXs)?;
        Ok(())
    }

    /// VerifyTransaction verifies transaction input signatures
    pub fn verify_transacton(&self, tx: &Transaction) -> Result<bool> {
        if tx.is_coinbase() {
            return Ok(true);
        }
        let prev_TXs = self.get_prev_TXs(tx)?;
        tx.verify(prev_TXs)
    }

    /// AddBlock saves the block into the blockchain
    /// mine_block 对应的是挖矿过程, 这个 add_block 接收一个 block 并将其添加到 Blockchain
    pub fn add_block(&mut self, block: Block) -> Result<()> {
        let data = serialize(&block)?;
        if let Some(_) = self.db.get(block.get_hash())? {
            return Ok(());
        }
        self.db.insert(block.get_hash(), data)?;

        let lastheight = self.get_best_height()?;
        if block.get_height() > lastheight {
            self.db.insert("LAST", block.get_hash().as_bytes())?;
            self.tip = block.get_hash();
            self.db.flush()?;
        }
        Ok(())
    }

    // GetBlock finds a block by its hash and returns it
    pub fn get_block(&self, block_hash: &str) -> Result<Block> {
        let data = self.db.get(block_hash)?.unwrap();
        let block = deserialize(&data.to_vec())?;
        Ok(block)
    }

    /// GetBestHeight returns the height of the latest block
    pub fn get_best_height(&self) -> Result<i32> {
        let lasthash = if let Some(h) = self.db.get("LAST")? {
            h
        } else {
            return Ok(-1);
        };
        let last_data = self.db.get(lasthash)?.unwrap();
        let last_block: Block = deserialize(&last_data.to_vec())?;
        Ok(last_block.get_height())
    }

    /// GetBlockHashes returns a list of hashes of all the blocks in the chain
    pub fn get_block_hashs(&self) -> Vec<String> {
        let mut list = Vec::new();
        for b in self.iter() {
            list.push(b.get_hash());
        }
        list
    }
}

/// 为 BlockchainIterator 实现了 Iterator trait！
/// if let: 如果获取成功（即返回 Ok(encoded_block)），那么 encoded_block 就会被解包并赋值
/// return match: 返回匹配到的分支
/// Option<Self::Item>:  next() 方法需要返回一个 Option 类型, Option 里包裹着 Block
impl<'a> Iterator for BlockchainIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encoded_block) = self.bc.db.get(&self.current_hash) {
            return match encoded_block {
                Some(b) => {
                    if let Ok(block) = deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}

use super::*;

#[test]
fn test_add_block(){

    let mut b = Blockchain::new().unwrap();
    // b.add_block("data1".to_string());
    // b.add_block("data2".to_string());
    // b.add_block("data3".to_string());
    
    // for item in b.iter(){
    //     println!("item: {:?}", item);
    // }
}