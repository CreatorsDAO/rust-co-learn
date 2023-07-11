#![allow(non_snake_case)]
#![allow(unused)]
//! Block implement of blockchain

use super::*;
use crate::transaction::Transaction;
use bincode::serialize;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use merkle_cbt::merkle_tree::Merge;
use merkle_cbt::merkle_tree::CBMT;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime};
use log::info;

const TARGET_HEXS: usize = 4;

/// Block keeps block headers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    timestamp: u128,
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    hash: String,  // hash Merkle Root
    nonce: i32,
    height: i32,
}

impl Block {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }

    pub fn get_transaction(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    /// NewBlock creates and returns Block
    pub fn new_block(
        transactions: Vec<Transaction>,
        prev_block_hash: String,
        height: i32,
    ) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)? // UNIX_EPOCH 获取当前系统时间, 计算当前时间距离UNIX纪元的Duration
            .as_millis(); // 转化为毫秒数
        let mut block = Block {
            timestamp,
            transactions,
            prev_block_hash,
            hash: String::new(),
            nonce: 0,
            height,
        };
        block.run_proof_of_work()?;  // 不断循环, 直到挖到为止
        Ok(block)
    }

    /// NewGenesisBlock creates and returns genesis Block
    pub fn new_genesis_block(coinbase: Transaction) -> Block {
        // coinbase = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        Block::new_block(vec![coinbase], String::new(), 0).unwrap()
    }

    /// Run performs a proof-of-work
    /// 矿工可以通过不断改变区块的 nonce 值 (随机数), 来寻找满足条件的哈希值, 这是实现工作量证明算法的关键步骤
    /// 挖矿: 不断地改变 block.nonce 的值(+= 1)，直到找到一个使 validate 方法返回 true 的 nonce
    /// 只要调用这个函数, 就会不断循环, 直到挖到合适的 nonce 才停止.
    fn run_proof_of_work(&mut self) -> Result<()> {
        info!("Mining the block");
        while !self.validate()? {
            self.nonce += 1; // 不断改变 Block 的 nonce
        }
        let data = self.prepare_hash_data()?;  // 运行到这里, 说明找到了符合条件的 nonce.
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    /// HashTransactions returns a hash of the transactions in the block
    /// 将区块中的所有交易哈希化，然后用 Merkle 树组合这些哈希值，得到一个树根，这个树根是一个对区块中所有交易的摘要
    fn hash_transactions(&self) -> Result<Vec<u8>> {
        let mut transactions = Vec::new();
        for tx in &self.transactions {
            transactions.push(tx.hash()?.as_bytes().to_owned());
        }
        let tree = CBMT::<Vec<u8>, MergeVu8>::build_merkle_tree(&transactions);

        Ok(tree.root())
    }

    /// 将  { 上一个区块 hash, 挖到的符合条件的 nonce, 时间戳, 交易数据} 等结构化数据序列化成字节流 Vec<u8> 返回.
    /// 这一步是为了将目前的 Block 的数据整体做 hash() ,作为未来下一个区块中的 "Previous Block hash" 字段的内容.
    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.hash_transactions()?,  // tree.root's hash
            self.timestamp,
            TARGET_HEXS, // 4
            self.nonce,
        );
        let bytes = serialize(&content)?;
        Ok(bytes)
    }

    /// Validate validates block's PoW
    /// 本 Block 的 nonce 不断变化, 不断调用该函数验证该 nonce 是否能让区块的哈希值满足某种条件 (hash 完后 TARGET_HEXS 位为 0)
    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?; // 将上一个区块的 hash, nonce.. 等结构化数据拿来做哈希, 为了放在下一个区块里面
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = Vec::new();
        // println!("{:?}", vec1):  [48, 48, 48, 48] , 48 是 '0' 的编码结果(utf-8), 这里挖矿难度是固定的, 最后 4 位 0
        vec1.resize(TARGET_HEXS, '0' as u8);
        // 比较哈希值的前 TARGET_HEXS (4) 位和全零向量是否相等，如果相等，就说明 nonce 有效。
        Ok(&hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec1)?) 
    }
}

struct MergeVu8 {}  // MergeVu8 struct 只是一个实现特定功能的载体

/// 接收两个 Vec<u8> 类型的引用，将它们合并后，用 SHA256 生成 hash value
impl Merge for MergeVu8 {
    type Item = Vec<u8>;  // 本实现中, 我们处理的数据类型是 Vec<u8>
    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut re: [u8; 32] = [0; 32];

        let mut hasher = Sha256::new();
        let mut data: Vec<u8> = left.clone();
        data.append(&mut right.clone());
        hasher.input(&data);
        hasher.result(&mut re);  // 将 hash 结果吐到 re 中
        re.to_vec()   // 是否可以不用辅助变量 re, 直接使用 into_bytes 将  String 转化为 Vec<u8> bytes ?
        // hasher.result_str().into_bytes()
    }
}