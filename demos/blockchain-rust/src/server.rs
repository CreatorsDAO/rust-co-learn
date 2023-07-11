//! server of Blockchain

use super::*;
use crate::block::*;
use crate::transaction::*;
use crate::utxoset::*;
use bincode::{deserialize, serialize};
use failure::format_err;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::*;
use std::thread;
use std::time::Duration;
use log::{debug, info};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Message { // 各种可能的消息类型
    Addr(Vec<String>),
    Version(Versionmsg),
    Tx(Txmsg),
    GetData(GetDatamsg),
    GetBlock(GetBlocksmsg),
    Inv(Invmsg),
    Block(Blockmsg),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Blockmsg {
    addr_from: String,
    block: Block,
}

/* 当一个节点收到 "getblocks" 消息后，它会将请求的区块数据通过 "block" 消息发回。
这样的机制使得节点可以维持更新自己的区块链，保持和网络中其他节点的同步。*/
#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetBlocksmsg {
    addr_from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetDatamsg {
    addr_from: String,
    kind: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Invmsg {
    addr_from: String,
    kind: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Txmsg {
    addr_from: String,
    transaction: Transaction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Versionmsg {
    addr_from: String,
    version: i32,
    best_height: i32,
}

/* Mutex（互斥锁）是一个并发原语，它保证了同一时间只能有一个线程访问数据。当一个线程想要访问被 Mutex 包裹的数据时，
  它需要首先获取锁（lock），如果锁已经被另一个线程持有，那么这个线程就会被阻塞直到锁被释放 
  Arc<Mutex<ServerInner>>：Arc（Atomic Reference Counting，原子引用计数）是一个智能指针，
  它使得数据能在多个线程之间共享。当 Arc 的引用计数为 0 时，数据会被安全地清理
  所以, ServerInner 类型的数据被一个 Mutex 包裹着以确保在同一时间只有一个线程可以访问和修改它，
  并且这个 Mutex 又被一个 Arc 包裹以使得这个 Mutex 可以在多个线程之间安全地共享
*/
pub struct Server {
    node_address: String,
    mining_address: String,
    inner: Arc<Mutex<ServerInner>>,
}

struct ServerInner {
    known_nodes: HashSet<String>,  // store 已知节点的地址
    utxo: UTXOSet,  // 区块链中所有未花费交易输出的集合 —— 记录了所有人的余额的账本
    blocks_in_transit: Vec<String>, // 没看懂: 这是一个字符串向量，存储了正在传输的区块的哈希值。当我们从其他节点请求区块时，我们将请求的区块哈希值存入这个向量，当我们接收到区块并将其添加到我们的区块链时，我们从这个向量中移除对应的区块哈希值。这就像我们的待办事项列表，记录了我们需要完成的任务。
    mempool: HashMap<String, Transaction> //内存池，存储"待处理交易", 当新的交易被创建时，我们将其添加到内存池，当交易被打包进区块时，我们将其从内存池中移除
}

const KNOWN_NODE1: &str = "localhost:3000";
const CMD_LEN: usize = 12;
const VERSION: i32 = 1;

impl Server {
    // 只会在服务器启动时调用一次，初始化服务器的状态
    pub fn new(port: &str, miner_address: &str, utxo: UTXOSet) -> Result<Server> {
        let mut node_set = HashSet::new();
        node_set.insert(String::from(KNOWN_NODE1));
        Ok(Server {
            node_address: String::from("localhost:") + port, // 正式部署时应该是个 IP
            mining_address: miner_address.to_string(),  // 负责挖矿（即创建新区块）的节点地址
            inner: Arc::new(Mutex::new(ServerInner {
                known_nodes: node_set,
                utxo,
                blocks_in_transit: Vec::new(),
                mempool: HashMap::new(),
            })),
        })
    }

    /* Starts the server and begins to listen for incoming connections. 
    A separate thread is started for each incoming connection. 
      每个传入连接都会启动一个单独的线程。 */
    pub fn start_server(&self) -> Result<()> {
        let server1 = Server {
            node_address: self.node_address.clone(),
            mining_address: self.mining_address.clone(),
            inner: Arc::clone(&self.inner),
        };
        info!(
            "Start server at {}, minning address: {}",
            &self.node_address, &self.mining_address
        );

        thread::spawn(move || {
            // 如果当前区块链的最高高度为-1（即区块链为空），服务器会向已知的节点（KNOWN_NODE1）发送请求，获取区块信息；
            // 否则，服务器会向已知节点发送当前版本信息。
            thread::sleep(Duration::from_millis(1000));
            if server1.get_best_height()? == -1 {
                server1.request_blocks()
            } else {
                server1.send_version(KNOWN_NODE1)
            }
        });
        
        // 监听传入的 TCP 连接
        let listener = TcpListener::bind(&self.node_address).unwrap();
        info!("Server listen...");

        // 在 TCP 协议中，一次通信开始时，发送端和接收端会进行一次 "握手" 来建立连接。
        // listener.incoming() 会返回一个迭代器，这个迭代器会为每个传入的连接请求生成一个新的 TcpStream。
        // 每个 TcpStream 代表了一个到客户端的 TCP 连接，服务器可以通过这个 TcpStream 来与客户端进行通信。
        // 服务器为每一个新的 TcpStream 启动一个新的线程，并在这个线程中处理该连接的所有请求
        for stream in listener.incoming() {
            let stream = stream?;
            let server1 = Server {
                node_address: self.node_address.clone(),
                mining_address: self.mining_address.clone(),
                inner: Arc::clone(&self.inner),
            };
            thread::spawn(move || server1.handle_connection(stream));
        }

        Ok(())
    }

    /* 每当需要发送新的交易时，就会调用这个函数。在这个函数中，每次都会创建一个新的Server实例来发送交易。这是一个比较简单的实现方式，但可能并不是最有效的 
       实际上，通常会希望有一个长期运行的服务器实例，它可以处理多个请求，而不是为每个请求都创建一个新的实例。 
       因为每次都创建新的服务器实例，所以每个请求都有自己的服务器状态（包括已知的节点列表，UTXO集合，正在传输的区块和内存池等），这些状态在处理请求结束后会被丢弃。如果有多个请求同时进行，它们各自的服务器实例之间不会互相影响。
       然而，这种方式可能会导致一些问题。例如，如果在多个请求之间共享一些状态（如UTXO集合）可能会更有效。否则，如果每个请求都有自己的UTXO集合，那么可能需要多次从磁盘或网络加载相同的数据，这会降低性能。另一方面，如果有大量的请求，那么为每个请求创建新的服务器实例可能会消耗大量的内存。 
       这是一个设计决策，取决于具体的需求和约束。在某些情况下，为每个请求创建新的服务器实例可能是合适的。在其他情况下，可能需要寻找其他的方法，例如使用线程池或异步IO来处理多个请求，同时共享一些状态。       
       */
    pub fn send_transaction(tx: &Transaction, utxoset: UTXOSet) -> Result<()> {
        let server = Server::new("7000", "", utxoset)?;
        server.send_tx(KNOWN_NODE1, tx)?;
        Ok(())
    }

    /* ------------------- inner halp functions ----------------------------------*/

    // 移除一个指定的节点 (如节点离线/不可达/行为异常/网络拓扑优化等需求)
    // lock() 获取这个互斥锁的所有权: 锁住互斥锁
    // 
    fn remove_node(&self, addr: &str) {
        self.inner.lock().unwrap().known_nodes.remove(addr);
    }

    fn add_nodes(&self, addr: &str) {
        self.inner
            .lock()
            .unwrap()
            .known_nodes
            .insert(String::from(addr));
    }

    fn get_known_nodes(&self) -> HashSet<String> {
        self.inner.lock().unwrap().known_nodes.clone()
    }

    fn node_is_known(&self, addr: &str) -> bool {
        self.inner.lock().unwrap().known_nodes.get(addr).is_some()
    }

    fn replace_in_transit(&self, hashs: Vec<String>) {
        let bit = &mut self.inner.lock().unwrap().blocks_in_transit;
        bit.clone_from(&hashs);
    }

    fn get_in_transit(&self) -> Vec<String> {
        self.inner.lock().unwrap().blocks_in_transit.clone()
    }

    /// 这些函数用于管理内存池，内存池类似于邮局的储存室，存储待处理的邮件（即交易）
    // get_mempool 用于获取 mempool 中对应地址的交易。mempool 是一个内存池，里面存储了待打包进区块的交易。
    // 如果 mempool 中存在对应地址的交易，则返回这个交易，否则返回 None
    fn get_mempool_tx(&self, addr: &str) -> Option<Transaction> {
        match self.inner.lock().unwrap().mempool.get(addr) {
            Some(tx) => Some(tx.clone()),
            None => None,
        }
    }

    fn get_mempool(&self) -> HashMap<String, Transaction> { // 当前的 mempool
        self.inner.lock().unwrap().mempool.clone()
    }

    fn insert_mempool(&self, tx: Transaction) { // 将一笔交易添加到 mempool
        self.inner.lock().unwrap().mempool.insert(tx.id.clone(), tx);
    }

    fn clear_mempool(&self) { // 清空 mempool
        self.inner.lock().unwrap().mempool.clear()
    }

    fn get_best_height(&self) -> Result<i32> {
        self.inner.lock().unwrap().utxo.blockchain.get_best_height()
    }

    fn get_block_hashs(&self) -> Vec<String> {
        self.inner.lock().unwrap().utxo.blockchain.get_block_hashs()
    }

    fn get_block(&self, block_hash: &str) -> Result<Block> {
        self.inner
            .lock()
            .unwrap()
            .utxo
            .blockchain
            .get_block(block_hash)
    }

    // 验证一个交易是否有效
    fn verify_tx(&self, tx: &Transaction) -> Result<bool> {
        self.inner
            .lock()
            .unwrap()
            .utxo
            .blockchain
            .verify_transacton(tx)
    }

    fn add_block(&self, block: Block) -> Result<()> {
        self.inner.lock().unwrap().utxo.blockchain.add_block(block)
    }

    // 挖掘一个新的区块
    fn mine_block(&self, txs: Vec<Transaction>) -> Result<Block> {
        self.inner.lock().unwrap().utxo.blockchain.mine_block(txs)
    }

    // 对 UTXO 集进行重新索引，它通常在区块链发生变更后被调用，以便保持 UTXO 集的最新状态
    fn utxo_reindex(&self) -> Result<()> {
        self.inner.lock().unwrap().utxo.reindex()
    }

    /* -----------------------------------------------------*/

    fn send_data(&self, addr: &str, data: &[u8]) -> Result<()> {
        if addr == &self.node_address { 
            return Ok(());  // 节点不需要向自己发送数据。
        }
        let mut stream = match TcpStream::connect(addr) {
            Ok(s) => s,
            Err(_) => {
                self.remove_node(addr);  //从已知节点集合中移除该地址, 一个无法连接的节点对我们来说没有用处。
                return Ok(());
            }
        };

        stream.write(data)?; // 这一行将数据写入到 TCP 连接中，这样数据就会被发送到目标地址。

        info!("data send successfully");
        Ok(())
    }

    fn request_blocks(&self) -> Result<()> {
        // 遍历所有已知节点。对每一个节点，我们都会做一次区块请求。
        for node in self.get_known_nodes() {
            self.send_get_blocks(&node)? // 这一行向当前遍历到的节点发送一个请求区块的消息
        }
        Ok(())
    }

    fn send_block(&self, addr: &str, b: &Block) -> Result<()> {
        info!("send block data to: {} block hash: {}", addr, b.get_hash());
        let data = Blockmsg {
            addr_from: self.node_address.clone(),
            block: b.clone(),
        };
        let data = serialize(&(cmd_to_bytes("block"), data))?;
        self.send_data(addr, &data)
    }

    // 向特定的地址发送节点信息
    fn send_addr(&self, addr: &str) -> Result<()> {
        info!("send address info to: {}", addr);
        let nodes = self.get_known_nodes();
        //序列化我们的消息，消息类型为 "addr"，消息内容为我们已知的所有节点的地址。
        let data = serialize(&(cmd_to_bytes("addr"), nodes))?;
        self.send_data(addr, &data)
    }

    // 向特定的地址发送 "inv" 类型的消息，消息内容是一些项目的 ID。
    fn send_inv(&self, addr: &str, kind: &str, items: Vec<String>) -> Result<()> {
        info!(
            "send inv message to: {} kind: {} data: {:?}",
            addr, kind, items
        );
        let data = Invmsg {
            addr_from: self.node_address.clone(),
            kind: kind.to_string(),
            items,
        };
        let data = serialize(&(cmd_to_bytes("inv"), data))?;
        self.send_data(addr, &data)
    }

    // 向特定的地址发送 "getblocks" 类型的消息
    fn send_get_blocks(&self, addr: &str) -> Result<()> {
        info!("send get blocks message to: {}", addr);
        let data = GetBlocksmsg {
            addr_from: self.node_address.clone(),
        };
        let data = serialize(&(cmd_to_bytes("getblocks"), data))?;
        self.send_data(addr, &data)
    }

    // 向特定的地址发送 "getdata" 类型的消息
    fn send_get_data(&self, addr: &str, kind: &str, id: &str) -> Result<()> {
        info!(
            "send get data message to: {} kind: {} id: {}",
            addr, kind, id
        );
        let data = GetDatamsg {
            addr_from: self.node_address.clone(),
            kind: kind.to_string(),
            id: id.to_string(),
        };
        let data = serialize(&(cmd_to_bytes("getdata"), data))?;
        self.send_data(addr, &data)
    }

    // 向特定的地址发送 "tx" 类型的消息
    pub fn send_tx(&self, addr: &str, tx: &Transaction) -> Result<()> {
        info!("send tx to: {} txid: {}", addr, &tx.id);
        let data = Txmsg {
            addr_from: self.node_address.clone(),
            transaction: tx.clone(),
        };
        let data = serialize(&(cmd_to_bytes("tx"), data))?;
        self.send_data(addr, &data)
    }

    // 向特定的地址发送 "version" 类型的消息
    fn send_version(&self, addr: &str) -> Result<()> {
        info!("send version info to: {}", addr);
        let data = Versionmsg {
            addr_from: self.node_address.clone(),
            best_height: self.get_best_height()?,
            version: VERSION,
        };
        let data = serialize(&(cmd_to_bytes("version"), data))?;
        self.send_data(addr, &data)
    }

    /// 处理接收到的 "version" 类型的消息 ("version" 类型的消息包含了发送者的区块链高度和发送者的地址)
    fn handle_version(&self, msg: Versionmsg) -> Result<()> {
        info!("receive version msg: {:#?}", msg);
        let my_best_height = self.get_best_height()?;
        if my_best_height < msg.best_height {
            // 如果当前节点的区块链高度低于发送者的区块链高度，向发送者发送 "getblocks" 消息，请求区块数据
            self.send_get_blocks(&msg.addr_from)?;
        } else if my_best_height > msg.best_height {
            // 若当前节点的区块链高度高于发送者的区块链高度, 向发送者发送 "version" 消息，让发送者知道当前节点的区块链高度
            self.send_version(&msg.addr_from)?;
        }
        // 向发送者发送 "addr" 消息，包含当前节点已知的所有节点地址。
        self.send_addr(&msg.addr_from)?;

        if !self.node_is_known(&msg.addr_from) {
            self.add_nodes(&msg.addr_from);
        }
        Ok(())
    }

    // 处理接收到的 "addr" 类型的消息。"addr" 类型的消息包含了发送者已知的所有节点地址。
    fn handle_addr(&self, msg: Vec<String>) -> Result<()> {
        info!("receive address msg: {:#?}", msg);
        for node in msg {
            self.add_nodes(&node);
        }
        //self.request_blocks()?;
        Ok(())
    }

    // 处理收到的 "block" 类型的消息。该消息包含一个区块。
    fn handle_block(&self, msg: Blockmsg) -> Result<()> {
        info!(
            "receive block msg: {}, {}",
            msg.addr_from,
            msg.block.get_hash()
        );
        self.add_block(msg.block)?; // 将收到的区块添加到本地的区块链中(为了保证区块链网络中的所有节点都能持有一份相同的、最新的区块链数据)。

        let mut in_transit = self.get_in_transit(); // 当前正在接收的区块序列
        // 如果正在接收的区块列表非空，则从发送者那里请求第一个区块，并将其从列表中移除。
        // 如果列表为空，则对 UTXO 集进行重新索引。
        if in_transit.len() > 0 {
            let block_hash = &in_transit[0];
            self.send_get_data(&msg.addr_from, "block", block_hash)?;
            in_transit.remove(0);
            self.replace_in_transit(in_transit);
        } else {
            self.utxo_reindex()?;
        }

        Ok(())
    }

    /// 处理收到的 "inv" 类型的消息
    /// - 对于 "block" 类型的数据，会向发送者请求第一个区块，并更新正在接收的区块列表。
    /// - 对于 "tx" 类型的数据，会检查本地交易池中是否已经有这个交易，如果没有则向发送者请求。
    fn handle_inv(&self, msg: Invmsg) -> Result<()> {
        info!("receive inv msg: {:#?}", msg);
        if msg.kind == "block" {
            let block_hash = &msg.items[0];
            self.send_get_data(&msg.addr_from, "block", block_hash)?;

            let mut new_in_transit = Vec::new();
            for b in &msg.items {
                if b != block_hash {
                    new_in_transit.push(b.clone());
                }
            }
            self.replace_in_transit(new_in_transit);
        } else if msg.kind == "tx" {
            let txid = &msg.items[0];
            match self.get_mempool_tx(txid) {
                Some(tx) => {
                    if tx.id.is_empty() {
                        self.send_get_data(&msg.addr_from, "tx", txid)?
                    }
                }
                None => self.send_get_data(&msg.addr_from, "tx", txid)?,
            }
        }
        Ok(())
    }

    /// 处理收到的 "getblocks" 类型的消息
    fn handle_get_blocks(&self, msg: GetBlocksmsg) -> Result<()> {
        info!("receive get blocks msg: {:#?}", msg);
        let block_hashs = self.get_block_hashs();
        self.send_inv(&msg.addr_from, "block", block_hashs)?;
        Ok(())
    }

    /// 处理收到的 "getdata" 类型的消息
    fn handle_get_data(&self, msg: GetDatamsg) -> Result<()> {
        info!("receive get data msg: {:#?}", msg);
        if msg.kind == "block" {
            let block = self.get_block(&msg.id)?;
            self.send_block(&msg.addr_from, &block)?;
        } else if msg.kind == "tx" {
            let tx = self.get_mempool_tx(&msg.id).unwrap();
            self.send_tx(&msg.addr_from, &tx)?;
        }
        Ok(())
    }

    /// 处理收到的 "tx" 类型的消息。该消息包含一个交易。
    /// 首先将交易添加到交易池中，然后根据节点类型（矿工节点还是普通节点）进行不同的处理。
    /// 如果当前节点是矿工节点，并且交易池中已经有足够的交易，则会进行挖矿并广播新区块。
    fn handle_tx(&self, msg: Txmsg) -> Result<()> {
        info!("receive tx msg: {} {}", msg.addr_from, &msg.transaction.id);
        self.insert_mempool(msg.transaction.clone());

        let known_nodes = self.get_known_nodes();
        if self.node_address == KNOWN_NODE1 { // Deploy 时, node_address/KNOWN_NODE1 应该是一个 IP?
            for node in known_nodes {
                if node != self.node_address && node != msg.addr_from {
                    self.send_inv(&node, "tx", vec![msg.transaction.id.clone()])?;
                }
            }
        } else {
            let mut mempool = self.get_mempool();
            debug!("Current mempool: {:#?}", &mempool);
            if mempool.len() >= 1 && !self.mining_address.is_empty() {
                loop {
                    let mut txs = Vec::new();

                    for (_, tx) in &mempool {
                        if self.verify_tx(tx)? {
                            txs.push(tx.clone());
                        }
                    }

                    if txs.is_empty() {
                        return Ok(());
                    }

                    let cbtx =
                        Transaction::new_coinbase(self.mining_address.clone(), String::new())?;
                    txs.push(cbtx);

                    for tx in &txs {
                        mempool.remove(&tx.id);
                    }

                    let new_block = self.mine_block(txs)?;
                    self.utxo_reindex()?;

                    for node in self.get_known_nodes() {
                        if node != self.node_address {
                            self.send_inv(&node, "block", vec![new_block.get_hash()])?;
                        }
                    }

                    if mempool.len() == 0 {
                        break;
                    }
                }
                self.clear_mempool();
            }
        }

        Ok(())
    }

    /// 处理 TCP 连接。首先读取从客户端发送过来的数据，然后根据消息类型调用相应的处理函数
    fn handle_connection(&self, mut stream: TcpStream) -> Result<()> {
        let mut buffer = Vec::new();
        let count = stream.read_to_end(&mut buffer)?;
        info!("Accept request: length {}", count);

        let cmd = bytes_to_cmd(&buffer)?;

        match cmd {
            Message::Addr(data) => self.handle_addr(data)?,
            Message::Block(data) => self.handle_block(data)?,
            Message::Inv(data) => self.handle_inv(data)?,
            Message::GetBlock(data) => self.handle_get_blocks(data)?,
            Message::GetData(data) => self.handle_get_data(data)?,
            Message::Tx(data) => self.handle_tx(data)?,
            Message::Version(data) => self.handle_version(data)?,
        }

        Ok(())
    }
}

/// 接受一个命令字符串，并返回它的二进制表示
fn cmd_to_bytes(cmd: &str) -> [u8; CMD_LEN] {
    let mut data = [0; CMD_LEN];
    for (i, d) in cmd.as_bytes().iter().enumerate() {
        data[i] = *d;
    }
    data
}

/// 接受一个字节数组，并试图将其解析为一个命令和相关数据
fn bytes_to_cmd(bytes: &[u8]) -> Result<Message> {
    let mut cmd = Vec::new();
    let cmd_bytes = &bytes[..CMD_LEN];
    let data = &bytes[CMD_LEN..];
    for b in cmd_bytes {
        if 0 as u8 != *b {
            cmd.push(*b);
        }
    }
    info!("cmd: {}", String::from_utf8(cmd.clone())?);

    if cmd == "addr".as_bytes() {
        let data: Vec<String> = deserialize(data)?;
        Ok(Message::Addr(data))
    } else if cmd == "block".as_bytes() {
        let data: Blockmsg = deserialize(data)?;
        Ok(Message::Block(data))
    } else if cmd == "inv".as_bytes() {
        let data: Invmsg = deserialize(data)?;
        Ok(Message::Inv(data))
    } else if cmd == "getblocks".as_bytes() {
        let data: GetBlocksmsg = deserialize(data)?;
        Ok(Message::GetBlock(data))
    } else if cmd == "getdata".as_bytes() {
        let data: GetDatamsg = deserialize(data)?;
        Ok(Message::GetData(data))
    } else if cmd == "tx".as_bytes() {
        let data: Txmsg = deserialize(data)?;
        Ok(Message::Tx(data))
    } else if cmd == "version".as_bytes() {
        let data: Versionmsg = deserialize(data)?;
        Ok(Message::Version(data))
    } else {
        Err(format_err!("Unknown command in the server"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::blockchain::*;
    use crate::wallets::*;

    #[test]
    fn test_cmd() {
        let mut ws = Wallets::new().unwrap();
        let wa1 = ws.create_wallet();
        let bc = Blockchain::create_blockchain(wa1).unwrap();
        let utxo_set = UTXOSet { blockchain: bc };
        let server = Server::new("7878", "localhost:3001", utxo_set).unwrap();

        let vmsg = Versionmsg {
            addr_from: server.node_address.clone(),
            best_height: server.get_best_height().unwrap(),
            version: VERSION,
        };
        let data = serialize(&(cmd_to_bytes("version"), vmsg.clone())).unwrap();
        if let Message::Version(v) = bytes_to_cmd(&data).unwrap() {
            assert_eq!(v, vmsg);
        } else {
            panic!("wrong!");
        }
    }
}
