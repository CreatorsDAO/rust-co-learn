ref code: 
- https://github.com/behrouz-rfa/blockchain-rust
- https://github.com/yunwei37/blockchain-rust



### Bitcoin / Blockchain Basics

[Bitcoin paper](https://bitcoin.org/bitcoin.pdf)
Mac 本地 Bitcoin paper

 - `open /System/Library/Image\ Capture/Devices/VirtualScanner.app/Contents/Resources/simpledoc.pdf`
 - [Bitcoin’s White Paper Has Been in Every MacOS Version Since 2018](https://cryptopotato.com/bitcoins-white-paper-has-been-in-every-macos-version-since-2018/)
 - https://www.coindesk.com/tech/2023/04/06/hidden-inside-macos-the-bitcoin-whitepaper/


区块链 : 

<img src="http://imagesoda.oss-cn-beijing.aliyuncs.com/Sodaoo/2023-07-04-110900.jpg" width="90%" />

- **The Bitcoin blockchain converges to a single-file chain** because each block references exactly one predecessor.  (单链表结构)
- Transactions: 在一个区块大约可以包含 2000 到 4000 笔交易

```rust
pub struct Block {
    timestamp: u128,
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    hash: String,
    nonce: i32,
    height: i32,
}
```



#### 挖矿难度

为了确保全网平均大约每 10 分钟就会产生一个新的区块。难度值是一个动态的参数，如果算力大幅增长，难度值也会对应增加。这种机制确保了比特币网络的安全性，同时也保持了区块生成的稳定性。

> 那么，在一个完全去中心化的网络中，这样的调整是如何做到的呢？难度的调整是在每个完整节点中独立自动发生的。 每2,016个区块中的所有节点都会调整难度。难度的调整公式是由最新2,016个区块的花费时长与20,160分钟（即 这些区块以10分钟一个速率所期望花费的时长）比较得出的。难度是根据实际时长与期望时长的比值进行相应调整的 （或变难或变易）。简单来说，如果网络发现区块产生速率比10分钟要快时会增加难度。如果发现比10分钟慢时则降低 难度。 —— 《精通比特币》

在本 Rust 代码实现中, 使用了这样一个算法: 
 - 为了挖矿, `run_proof_of_work()`  会不断改变 Block 的 nonce 值, 来验证该 `nonce` 是否能让区块的哈希值满足某种条件 (hash 完后 TARGET_HEXS 位为 0)

```rust
    /// Run performs a proof-of-work
    /// 矿工可以通过不断改变区块的 nonce 值 (随机数), 来寻找满足条件的哈希值, 这是实现工作量证明算法的关键
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
        self.hash = hasher.result_str();  // 新区块哈希值.
        Ok(())
    }

    /// Validate validates block's PoW
    /// 本 Block 的 nonce 不断变化, 不断调用该函数验证该 nonce 是否能让区块的哈希值满足某种条件 (hash 完后 TARGET_HEXS 位为 0)
    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?; // hash(Block) 
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = Vec::new();
        // println!("{:?}", vec1):  [48, 48, 48, 48] , 48 是 '0' 的编码结果(utf-8), 这里挖矿难度是固定的, 最后 4 位 0
        vec1.resize(TARGET_HEXS, '0' as u8);
        // 比较哈希值的前 TARGET_HEXS (4) 位和全零向量是否相等，如果相等，就说明 nonce 有效。
        Ok(&hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec1)?)  
    }
```



当把这个结果放进 区块头时，nonce 4,215,469,401 就会产生一个区块哈希值：nonce `4,215,469,401` 就会产生一个区块哈希值：
`0000000000000002a7bbd25a417c0374cc55261021e8a9ca74442b01284f0569`
而这个值小于难度目标值：
`00000000000000000000000003A30C00000000000000000000000000000000000000000000`



Jing 的挖矿节点立刻将这个区块发给它的所有相邻节点。这些节点在接收并验证这个新区块后，也会继续传播此区块。当这个新区块在网络中扩散时，每个节点都会将它作为区块277,316加到自身节点的区块链副本中”





#### 打包交易:

比特币网络中，"打包交易"是指矿工**从内存池（mempool）中选择一些交易，验证这些交易，然后将它们包含在他们正在工作的新区块中**的过程。这个过程的主要步骤包括以下几点：

1. **选择交易**：矿工首先从内存池中选择一些交易。内存池是网络中所有未确认交易的集合。矿工通常会优先选择交易费用（Transaction fee, 也就是他们可以获得的收益）较高的交易。
2. **验证交易**：矿工需要验证他们选择的每一笔交易。这包括检查交易的输入是否足够，签名是否正确，以及交易是否满足比特币协议的其他规则。
3. **创建新区块**：一旦选择并验证了一组交易，矿工就会将这些交易包含在他们正在创建的新区块中。他们还会添加一个特殊的交易，称为 coinbase 交易，作为他们挖矿的奖励。（矿工们在挖矿过程中会得到两种类型的奖励：创建新区块的新币奖励，以及区块中所含交易的交易费）
4. **挖矿**：在交易被打包到新区块后，矿工就开始尝试解决一个计算密集型的问题，也就是寻找一个满足特定条件的哈希值。这个过程被称为挖矿。
5. **发布新区块**：如果矿工成功找到了满足条件的哈希值，他们就会发布这个新区块到比特币网络。其他的节点如果验证这个新区块是有效的，就会将它添加到他们的区块链上，并从他们的内存池中移除这个区块中的交易。
6. **新的一轮**：当其他矿工们收到新区块的广播并验证后， 这个"计算密集型的问题" 随之改变 (因为如下代码的 `self.prepare_hash_data()` 变化了 )，所有矿工都需要放弃之前的努力，重新开始新一轮计算

```rust
    /// Validate validates block's PoW
    /// 本 Block 的 nonce 不断变化, 不断调用该函数验证该 nonce 是否能让区块的哈希值满足某种条件 (hash 完后 TARGET_HEXS 位为 0)
    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = Vec::new();
        // println!("{:?}", vec1):  [48, 48, 48, 48] , 48 是 '0' 的编码结果(utf-8), 这里挖矿难度是固定的, 最后 4 位 0
        vec1.resize(TARGET_HEXS, '0' as u8);
        // 比较哈希值的前 TARGET_HEXS (4) 位和全零向量是否相等，如果相等，就说明 nonce 有效。
        Ok(&hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec1)?) 
    }
```


所以，矿工只能将交易打包到他们自己正在工作的新区块中，他们不能将交易打包到别人的区块中。一旦一个区块被发布到网络并被其他节点接受，这个区块就不能再被修改，包括其中的交易。

比特币网络中，一个区块的大小上限是 1 MB，每笔交易的大小不同，一般在 200 到 1000 字节之间。因此，一个区块大约可以包含 2000 到 4000 笔交易，但这个数字会根据实际的交易数据有所不同。

一旦矿工成功地“挖出”（也就是找到了满足特定难度条件的哈希值）一个新区块，他们就会将这个新区块广播到比特币网络。每个接收到这个新区块的节点都会验证这个区块的有效性，包括区块头的哈希值是否满足难度条件，区块中的交易是否都是有效的等等。如果这个新区块被验证为有效，那么节点就会将它添加到他们自己的区块链中，这个新区块就成为了区块链的新的一部分，或者说新的一个节点。

需要注意的是，当一个新区块被添加到区块链后，它包含的所有交易都被认为是已确认的，而且不能再被改变。这是因为如果要修改一个已经被添加到区块链的区块，需要重新计算这个区块以及所有后续区块的哈希值，这在计算上是几乎不可能的，这也是区块链能提供高度安全性的主要原因。



<img src="http://imagesoda.oss-cn-beijing.aliyuncs.com/Sodaoo/2023-07-04-Blockchain-05.jpg" width="70%" />


#### Coinbase Transaction

A coinbase transaction is **the first transaction in a block**. It is a unique type of bitcoin transaction that can be created by a miner. The miners use it to collect the block reward for their work and any other transaction fees collected by the miner are also sent in this transaction.

`Coinbase` 交易只能由矿工创建，并且矿工收取的任何其他交易费用也会在此交易中发送。

`Coinbase` 交易是每个区块中的第一笔交易，这种交易存在的原因是作为对挖矿的奖励，创造出全新的 UTXO 用来支付给“赢家”矿工。这也就是为什么比特币可以在挖矿过程中被创造出来。



#### UTXO

UTXO代表未花费的交易输出（Unspent Transaction Output）。在基于UTXO的区块链如比特币中，账户余额并不直接被记录，而是通过计算所有未花费的交易输出得到的。

首先，我们需要理解的是交易（Transaction）的基本结构。每笔交易都有输入（Input）和输出（Output）。

**交易输入（Input）**：它是指向一个早前的交易输出，并包含一个解锁脚本（解密或验证脚本）以证明这个UTXO可以被花费。

**交易输出（Output）**：每个输出包含一个新的锁定脚本（通常包含接收者的公钥哈希）和一定的数量的货币（如比特币）。

例如，Alice有50个比特币，她想要发送 30 个比特币给Bob。

1. Alice 创建一个交易，输入是她的50个比特币的UTXO，输出有两个，一个是发送给Bob的30个比特币，另一个是找零的20个比特币（返回给Alice自己）。
2. Alice 会用她的私钥对这个交易进行签名，这个签名会被包含在输入部分，以证明她有权花费这个UTXO。
3. 这个交易会被广播到比特币网络，挖矿节点会验证这个交易的合法性（包括验证签名，确保UTXO未被花费等）。
4. 一旦交易被网络中的节点验证并接受，它将被包含在新的区块中并添加到区块链上。这样，Alice的50个比特币的 UTXO 就被标记为已花费，而两个新的 UTXO（30个比特币给 Bob，20个比特币给Alice）就被创建。

注意，一个UTXO只能被完整的花费，也就是说，不能部分使用一个UTXO。如果交易金额小于UTXO的值，那么必须生成找零。

因此，用户的钱包余额是通过扫描区块链并累加所有属于用户的 UTXO 来计算的。



#### SPV/轻节点钱包

轻节点钱包（Lightweight Wallet）或SPV（Simple Payment Verification）钱包是一种专门为移动设备和低功率设备设计的比特币钱包类型。相对于完整节点钱包，轻节点钱包不需要下载整个比特币区块链，而只需要下载区块头信息，因此需要的存储空间和计算资源更少。

比特币轻节点钱包的主要 **功能特性** 如下：
1. 发送和接收比特币：就像全节点钱包一样，轻节点钱包允许用户发送和接收比特币。
2. 验证交易：轻节点钱包使用SPV模式验证交易。它只下载区块头信息，然后查询完整节点以验证其交易。然而，轻节点钱包无法验证交易是否是最新的或是否有双花情况，它们必须依赖完全节点的诚实性。
3. 提供用户隐私：由于轻节点钱包只需要查询它们感兴趣的特定交易，因此可以在一定程度上保护用户隐私。

**工作原理**
1. 同步区块头：轻节点钱包从网络上的完全节点下载所有的区块头，这通常大约需要 **$\color{red}40MB$** 的空间，远远小于整个比特币区块链的大小（到 2021 年 9 月已经超过 350 GB）。
2. 查询交易：当用户想要查看他们的交易记录或余额时，轻节点钱包会向完全节点查询他们感兴趣的交易。
3. 验证交易：轻节点钱包通过比对交易的区块头与本地存储的区块头，可以验证这个交易是否包含在区块链中。但由于它没有完整的区块链数据，因此无法验证交易是否是最新的或是否存在双花情况。

轻节点钱包可以向全节点请求该交易的 Merkle 证明（Merkle proof）。Merkle证明是一种数据结构，它包含了从某个特定的交易哈希到区块的 Merkle 根的路径，这个路径上包含了所有需要的节点的哈希值。

轻节点钱包可以通过这个 Merkle 证明验证该交易是否包含在区块链中。**因为如果该交易是真实存在的，那么通过 Merkle 证明计算出来的 Merkle 根应该与区块头中的 Merkle 根完全一致**。通过这种方式，轻节点钱包可以在不需要下载全部区块链数据的情况下验证交易的有效性。

综上，轻节点钱包虽然无法像完全节点一样对所有的交易进行完全验证，但对于大部分用户来说，它提供了一个方便、易用、且需要更少资源的方法来发送和接收比特币。

**举例理解** :  假设我们有一个简单的区块，它包含 4 个交易：TxA, TxB, TxC 和 TxD。我们首先为每个交易生成一个哈希值：HA, HB, HC 和 HD。然后，我们创建一个Merkle树，将这四个哈希值作为叶节点：

```rust
		HABCD
	   /     \
	 HAB    (HCD)
	/  \    /   \
  HA  (HB) HC   HD
  |    |    |    | 
 TxA  TxB  TxC  TxD
  ↑
待验证
```

现在假设你的轻节点钱包想要验证 TxA 是否在这个区块中。全节点会向你的钱包提供一个 Merkle 证明，该证明包含以下数据：HB（TxA的兄弟节点）、HCD（TxA的叔父节点）。

你的钱包现在可以自行计算 Merkle 根，步骤如下：
1. 计算 HA 和 HB 的哈希值组合，得到 HAB。
2. 然后计算 HAB 和 HCD 的哈希值组合，得到 HABCD。

如果计算出来的HABCD与区块头中的Merkle根一致，那么你就可以确定TxA确实存在于这个区块中，而不需要下载整个区块的所有交易。

这就是如何使用Merkle证明和Merkle树来验证一个交易。只需几个哈希计算，就可以验证一个交易，而无需下载整个区块链。



#### 信息传递

```rust
enum Message { // 各种可能的消息类型
    Addr(Vec<String>), // 节点信息，使得新加入的节点可以通过获取已经存在的节点信息快速接入网络。
    Version(Versionmsg), // 首次建立连接时，会互相发送版本信息来交换彼此的协议版本
    Tx(Txmsg),  // 当一个节点产生新的交易时，它会将这个交易广播到整个比特币网络。每个节点在接收到交易信息后，都会检查该交易是否有效，如果有效则会添加到自己的内存池中，并将其进一步广播给其他节点。
    GetData(GetDatamsg),  // 当节点收到 `INV` 消息并确认需要其中的交易或区块时，节点会向信息的源节点发送 GETDATA 消息请求特定的信息
    GetBlock(GetBlocksmsg),
    Inv(Invmsg),
    Block(Blockmsg),
}
```

在比特币的 P2P 网络中，节点之间需要相互传递哪些信息？请根据论文详细说明

- **ADDR消息**：用于交换节点信息，使得新加入的节点可以通过获取已经存在的节点信息快速接入网络。

- **版本信息(Version Message)**：在节点之间首次建立连接时，会互相发送版本信息来交换彼此的协议版本和网络地址等信息，以确保彼此之间能够顺利地交换信息。

- **交易信息(Transaction Information)**：这是节点之间最常见的信息交换类型。当一个节点产生新的交易时，它会将这个交易广播到整个比特币网络。每个节点在接收到交易信息后，都会检查该交易是否有效，如果有效则会添加到自己的内存池中，并将其进一步广播给其他节点。

- **INV 消息 (Inventory 库存清单 Message)**：用于广播一个节点有哪些交易、区块等信息，让其他节点知道自己有哪些信息是新的，可以让其他节点选择是否请求这些信息。

- **GETDATA 和 BLOCK/TRANSACTION 消息**：当节点收到 `INV` 消息并确认需要其中的交易或区块时，节点会向信息的源节点发送 GETDATA 消息请求特定的信息，源节点在收到 GETDATA 消息后，会用 BLOCK 或 TRANSACTION 消息将请求的信息发送回去。

- **区块信息(Block Information)**：当一个节点成功地挖出一个新区块时，它会将新区块的信息广播到整个网络。其他节点在收到新的区块信息后，会进行验证，如果验证通过，那么就将新的区块添加到自己的区块链中。

- **Ping/Pong 消息(Ping/Pong Messages)**：为了检测节点之间的连接状态，节点之间会定期发送 Ping/Pong 消息。如果一段时间内没有收到某个节点的 Pong 消息，那么就可以认为与该节点的连接已经断开。




----

##### INV 消息

**INV(Inventory Message)**: INV消息用于通知其他节点，发送者有一个未见过的交易或区块。这样，其他节点可以决定是否请求更多的信息。比特币网络中的节点会接收并处理它们接触到的每一笔交易和每一个区块，但是，如果每个节点都立即将每一个新的交易或区块广播给它们连接的所有其他节点，那么将会产生大量的冗余数据流通，因此 INV 消息的设计就是为了解决这个问题，即通过宣告新的交易或区块的存在，而不是立即广播所有详细信息，从而减少网络的数据传输压力。

什么时候需要发送 INV 消息？ 

1. **新交易产生**：当节点产生或接收到新的交易，并且经过验证是有效的，那么该节点会将这笔交易的信息（如交易的哈希值）打包进INV消息，并广播给与它连接的所有节点。
2. **新区块生成**：当节点挖出新的区块或接收并验证通过别的节点挖出的新区块，该节点会将新区块的信息（如区块的哈希值）打包进INV消息，并广播给与它连接的所有节点。

什么时候需要对 INV 消息作出响应？

1. **收到新交易或新区块的INV消息**：当节点收到包含新交易或新区块信息的INV消息，并且这些信息在该节点的内存池或区块链中不存在时，该节点会对这个INV消息做出响应。响应的方式是通过发送GETDATA消息，请求发送INV消息的节点提供更多关于这些新交易或新区块的信息。
2. **收到已知交易或已知区块的INV消息**：如果节点收到的INV消息包含的交易或区块信息已经在其内存池或区块链中存在，那么该节点通常不会对这个INV消息做出响应。这是因为节点已经拥有这些信息，没有必要再次请求。

总的来说，INV消息的发送和响应是比特币网络中信息传播的重要机制，保证了所有节点能够获取到最新和完整的交易和区块信息，从而保持整个比特币网络的一致性。



### Running Steps

```bash
cargo run createwallet
> address: 36vU3DQNwtvQdK15YTukHVaRkgPi13mgE3
> address: 3Aqkny4HPEqvLNe5ASDH6B1ALRsoE7FgNU

cargo run createblockchain 36vU3DQNwtvQdK15YTukHVaRkgPi13mgE3  // 化身中本聪，创建创世交易区块链！
> create blockchain

cargo run getbalance  36vU3DQNwtvQdK15YTukHVaRkgPi13mgE3
cargo run getbalance  3Aqkny4HPEqvLNe5ASDH6B1ALRsoE7FgNU
> Balance: 10
> Balance: 0

cargo run send 3NxywAS3h4ieYLnA4drY2LrbAJJaArrcti  39XkmxAotz7thjkm8qXsdGmpFoYjso9zCa 2 -m

cargo run getbalance  39XkmxAotz7thjkm8qXsdGmpFoYjso9zCa


cargo tree  # 依赖查询


startminer <PORT> <ADDRESS>
startnode  <PORT> <ADDRESS>
cargo run printchain  # Block { ..TXInput {..TXOutput {..

cargo run listaddresses


cargo  test server
```

#### createwallet


#### createblockchain

```rust
fn cmd_create_blockchain(address: &str) -> Result<()> {
    let address = String::from(address);
    let bc = Blockchain::create_blockchain(address)?;

    let utxo_set = UTXOSet { blockchain: bc };
    utxo_set.reindex()?;
    println!("create blockchain");
    Ok(())
}
```

`blockchain.rs`

```rust
    /// CreateBlockchain creates a new blockchain DB
    /// 化身中本聪, 创建比特币区块链! 第一步: 创建创始区块!
    /// 先清空本地数据库，创建一个新的 db 文件，并添加一个创世区块，该区块包含一个 coinbase 交易 (cbtx)
    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Creating new blockchain");

        std::fs::remove_dir_all("data/blocks").ok();
        let db = sled::open("data/blocks")?;
        debug!("Creating new block database");
        // GENESIS_COINBASE_DATA: 这是当时《泰晤士报》的一条新闻标题，用来证明这个区块是在2009年1月3日之后创建的. 在比特币网络中，创世区块在2009年由中本聪(Satoshi)创建，并且从那时起就固定在比特币区块链的开始处.
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
```

`utxoset.rs`

```rust
    /// Reindex rebuilds the UTXO set
    /// 重建UTXO集合。首先删除数据库，然后通过查找区块链中的所有未花费交易输出来重新填充数据库。
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
```



#### send

从一个地址（`from`）向另一个地址（`to`）发送一定数量 BTC

```rust
/// mine_now变量用于控制是否立即开采一个新的区块来包含这笔交易。
/// mine_now:
///   true: 选择自己打包交易, 那么就需要自己 mine_block 进行挖矿
///   false: `Server::send_transaction` 将交易发送到其他节点。这个交易会被添加到内存池中，等待其他矿工再其挖矿过程中将其打包进新的区块。
fn cmd_send(from: &str, to: &str, amount: i32, mine_now: bool) -> Result<()> {
    let bc = Blockchain::new()?;  // 读取本地数据库, return Blockchain { tip: lasthash, db }
    let mut utxo_set = UTXOSet { blockchain: bc };
    let wallets = Wallets::new()?; // 读取本地数据库, return `mut wlt` obj
    let wallet = wallets.get_wallet(from).unwrap();
    let tx = Transaction::new_UTXO(wallet, to, amount, &utxo_set)?;
    if mine_now {
        let cbtx = Transaction::new_coinbase(from.to_string(), String::from("reward!"))?;
        let new_block = utxo_set.blockchain.mine_block(vec![cbtx, tx])?;

        utxo_set.update(&new_block)?;
    } else {
        Server::send_transaction(&tx, utxo_set)?;
    }

    println!("success!");
    Ok(())
}
```


`mine_now` 参数:

如果 `mine_now` 参数设为 `true`，那么在创建新的交易后，就会立即创建一个 coinbase 交易，并将这两个交易都放入一个新的区块中。然后通过 `utxo_set.blockchain.mine_block` 进行挖矿，将新的区块添加到区块链中。这样，新的交易就会被立即确认。

如果 `mine_now` 参数设为 `false`，那么在创建新的交易后，将不会立即进行挖矿确认，而是通过 `Server::send_transaction` 将新的交易发送到其他节点。这个交易会被添加到内存池中，等待矿工在之后的挖矿过程中将其打包进新的区块。

总的来说，`mine_now` 参数决定了是否在创建新的交易后立即进行挖矿以确认这个交易。如果 `mine_now` 参数设为 `true`，则立即进行挖矿；如果设为 `false`，则不立即进行挖矿，而是将新的交易发送到其他节点，等待被挖掘进新的区块。

对于发送者（也就是进行挖矿的矿工）来说，自己打包交易有以下好处：

1. 确认时间：当交易被打包到区块并添加到区块链中后，该交易就被认为是确认的。如果矿工自己挖矿并打包自己的交易，他可以更快地得到交易确认，而不用等待其他矿工进行挖矿。
2. 交易费：每个区块的矿工可以收取该区块中所有交易的交易费。如果矿工自己打包交易，他可以收取这些交易的交易费。
3. 有选择地包含交易：矿工可以选择性地包含他认为有利的交易，例如交易费高的交易。

对于其他矿工来说，打包交易有以下好处：

1. 挖矿奖励：当矿工挖出一个新的区块时，他会得到一定数量的比特币作为奖励，这就是所谓的区块奖励。这个奖励是矿工的主要收入来源。
2. 交易费：每个区块的矿工可以收取该区块中所有交易的交易费。这些交易费是矿工的另一种收入来源。

总的来说，自己挖矿和打包交易可以让矿工更快地确认交易，而且可以收取交易费。而对于其他矿工来说，打包交易可以让他们得到区块奖励和交易费。



但是自己打包有可能很长时间内都挖不到这个区块，这个交易也会迟迟得不到确认？

ChatGPT:  是的，您的理解完全正确。比特币的挖矿过程实际上是一个对于解决特定问题的竞争，这个问题的解决需要大量的计算能力和时间。这就意味着，一个矿工（无论是个人还是矿池）可能需要花费很长时间才能找到区块的正确哈希值，即成功地“挖到”新的区块。

因此，如果发送者选择自己来挖矿并打包交易，特别是在他的计算能力相对较弱时，这个过程可能会花费很长时间，交易的确认也可能会被相应地延迟。

这也是为什么很多时候，交易的发送者会选择将交易发布到比特币网络，由整个比特币网络中的矿工来共同竞争解决这个问题，这样可以更快地得到交易的确认。

然而，这也带来了一种权衡：如果发送者自己进行挖矿并成功了，他可以获得包括挖矿奖励和交易费用在内的全部收益；但如果他选择发布交易到网络中，虽然交易确认的速度可能会加快，但他就无法获得这些挖矿的奖励。



### code structure

![](assets/2023-07-13-114233.png)


### block.rs

```rust
pub struct Block {
    timestamp: u128,
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    hash: String,
    nonce: i32,
    height: i32,
}
```

`Block` 区块结构体：这是整个代码的核心，代表区块链中的一个区块。它包含了一个区块所需要的所有信息，包括时间戳（`timestamp`）、交易记录（`transactions`）、前一个区块的哈希值（`prev_block_hash`）、自身的哈希值（`hash`）、工作量证明所需要的随机数（`nonce`）和区块的高度（`height`）。

- `new_block`: 创建一个新的区块。这个方法会根据给定的交易记录、前一个区块的哈希值和高度创建一个新的区块，然后运行工作量证明算法，并设置区块的哈希值。
- `new_genesis_block`: 创建一个创世区块，创世区块是区块链的第一个区块，它没有前一个区块的哈希值。
- `run_proof_of_work`: 这是实现工作量证明算法的方法。工作量证明是一种防止区块链被恶意攻击的机制。这个方法会不断地改变随机数，直到找到一个使得区块的哈希值满足一定条件的随机数。
- `hash_transactions`: 将区块中的所有交易哈希化，然后用 Merkle 树组合这些哈希值，得到一个树根，这个树根是一个对区块中所有交易的摘要。
- `prepare_hash_data`: 准备用于哈希的数据，包括前一个区块的哈希值、所有交易的哈希值、时间戳、目标难度和随机数。
- `validate`: 验证区块的工作量证明是否有效。



#### `MergeVu8` 结构

`MergeVu8` 是一个实现了 `Merge` trait 的结构。定义了如何合并 Merkle 树中的两个子节点的哈希值。在 Merkle 树中，需要定义如何合并两个子节点的哈希值，`Merge` trait 就是用来定义这个合并操作的。`MergeVu8` 实现了 `Merge` trait 的 `merge` 方法，表示将两个子节点的哈希值通过 SHA256 算法进行哈希，得到新的哈希值。

整个代码的核心思路是模拟区块链的创建和验证过程。首先创建一个创世区块，然后添加新的区块，每添加一个新的区块都要进行工作量证明和哈希计算，保证区块链的安全性。



### blockchain.rs

把所有的数据都存储在了一个简单的键值对数据库（sled）中
这样的设计虽然简单，但是可能在处理大量数据时效率会较低。在真实的比特币实现中，会有更复杂的数据结构和算法来处理这些问题。

```rust
pub struct Blockchain {
    pub tip: String,   // 区块链上 最后一个区块的哈希
    pub db: sled::Db,
}
```

1. `Blockchain::new()`: 创建一个新的 Blockchain 实例，打开已存在的数据库，并获取数据库中 "LAST" 键的值作为区块链的最后一个区块的哈希。

2. `Blockchain::create_blockchain(address: String)`: 创建一个新的区块链并将第一笔交易（也叫 "coinbase" 交易）发给参数中的地址。这个函数会先清空已有的数据库，然后创建一个新的数据库，并且添加一个创世区块，该区块包含一个 coinbase 交易。

3. `Blockchain::mine_block(&mut self, transactions: Vec<Transaction>)`: 创建一个新的区块，包含了参数中的所有交易，并将新区块添加到区块链中。首先，这个函数会验证每一个交易的有效性，然后创建一个新的区块，并将新的区块存入数据库。

4. `Blockchain::iter(&self)`: 创建一个迭代器，可以用来遍历整个区块链。遍历开始于最后一个区块，然后依次向前遍历。

5. `Blockchain::find_UTXO(&self)`: 找到并返回所有未花费的交易输出（Unspent Transaction Output，简称 UTXO）。它会遍历整个区块链，找出所有的输出，并排除已经被花费的。

6. `Blockchain::find_transacton(&self, id: &str)`: 通过交易ID查找一个交易。这个函数会遍历整个区块链，直到找到含有指定交易ID的区块。

7. `Blockchain::get_prev_TXs(&self, tx: &Transaction)`: 获取一个交易所有输入引用的输出。这个函数在签名和验证交易的时候会用到。

8. `Blockchain::sign_transacton(&self, tx: &mut Transaction, private_key: &[u8])`: 对一个交易的每一个输入进行签名。它会先找到每一个输入引用的输出，然后用私钥进行签名。

9. `Blockchain::verify_transacton(&self, tx: &Transaction)`: 验证一个交易的每一个输入的签名。它也会先找到每一个输入引用的输出，然后用引用的输出的公钥进行验证。

10. `Blockchain::add_block(&mut self, block: Block)`: 将一个新的区块添加到区块链中。这个函数会先检查这个区块是否已经存在，然后将新的区块的数据存入数据库，并更新 "LAST" 键的值。

11. `Blockchain::get_block(&self, block_hash: &str)`: 通过区块的哈希找到并返回一个区块。这个函数会从数据库中获取区块的数据，然后反序列化为 `Block` 实例。

12. `Blockchain::get_best_height(&self)`: 返回区块链中最后一个区块的高度。

13. `Blockchain::get_block_hashs(&self)`: 返回区块链中所有区块的哈希。

14. `BlockchainIterator::next(&mut self)`: `BlockchainIterator` 的 `next` 方法，用来在区块链中获取下一个区块。


#### BlockchainIterator

```rust
pub struct BlockchainIterator<'a> {
    current_hash: String,
    bc: &'a Blockchain,  // bc 是指向 Blockchain 的引用, lifetime 标注告诉编译器: 引用的有效期至少和 'a 一样长
}

impl Blockchain {
    /// 为  Block 定义 iter() 方法辅助后面 Iterator 的实现.
    pub fn iter(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.tip.clone(),
            bc: &self,
        }
    }


    pub fn find_UTXO(&self) -> HashMap<String, TXOutputs> {

        // 迭代 整个区块链 db 的每个区块 & 每个区块中的每个交易
        for block in self.iter() {
        ...
    }
}

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
```

**问题 ①  Blockchain 没实现 Iterator trait, 为什么还能 self.iter() 调用迭代器方法?**

Blockchain 没实现 Iterator trait，但它通过实现 `iter()` 方法 returns `BlockchainIterator` 迭代器对象。
`BlockchainIterator` 迭代器对象实现了 `Iterator` trait，因此可以在 for 循环中使用。


**问题 ② 为什么要额外创建一个 BlockchainIterator， 而不是直接在 Blockchain 上实现 Iterator 呢？**

在这段代码中，使用额外的  `BlockchainIterator`  结构体作为迭代器的原因有几个：

1. 灵活性和封装性：通过将迭代器的实现与 `Blockchain` 结构体分离，可以将迭代器的逻辑与 `Blockchain` 的其他功能进行解耦。这样可以提高代码的模块化和可维护性，使得每个结构体负责自己的职责。
2. 生命周期限制：在 `BlockchainIterator` 结构体中，使用了生命周期参数 `'a` 来标注引用的有效期，以确保引用在整个迭代过程中保持有效。如果直接在 `Blockchain` 上实现 `Iterator`，则需要将生命周期参数传递给`Iterator` trait，这会增加对 `Blockchain` 结构体本身的生命周期限制和复杂性。
3. 代码可读性：将迭代器的实现从 `Blockchain` 分离出来，可以使得 `Blockchain` 的代码更加清晰和简洁。同时，使用 `for` 循环遍历时，通过 `self.iter()` 的语法，可以更直观地表示对 `Blockchain` 的迭代操作，提高代码的可读性和易用性。

总结起来，通过创建一个额外的  `BlockchainIterator`  结构体，可以提高代码的模块化、可维护性和可读性，同时避免了复杂的生命周期限制和耦合。这样的设计使得迭代器的实现更加灵活，并且更好地符合 `Rust` 的所有权和借用规则。


### transaction.rs


#### TXOutput

在UTXO（Unspent Transaction Output）模型中，交易的输出被标识为可花费的新资金。每个交易输出（TXOutput）都包含一个金额（value）和一个锁定脚本。这个锁定脚本通常包含收款人的公钥哈希（pub_key_hash），用于指示这些资金可以被哪个公钥所使用。

当创建一个新的交易时，我们通过引用之前交易的ID和输出序号（vout）来指定我们想要使用的UTXO。然而，每个交易输出（TXOutput）本身并不需要保存这个信息，因为他们的“身份”或者“引用”来自于它们是哪个交易的输出。

在这种情况下，交易ID和输出序号（vout）主要存在于交易输入（TXInput）中。在一个交易输入中，这两个字段用于指向正在被引用的UTXO。

所以，我们在TXOutput中看不到ID和vout字段，是因为这些字段被存储在引用这些TXOutput的TXInput中。TXOutput只需要知道它包含了多少价值（value）以及谁有权利花费它（pub_key_hash）。


#### verify

在Bitcoin或其他UTXO模型的加密货币中，交易的签名过程和验证过程是相对复杂的。这是因为每个输入的签名是单独生成的，而不是对整个交易生成一个签名。这意味着在签名或验证特定输入时，其他输入的数据需要被暂时移除。

以下是  `verify`  函数中所做操作的解释：

1. 清除修剪版交易副本中此输入的签名：这是为了模拟签名过程中的状态。当一个输入被签名时，它的签名字段是空的，因为签名本身还没有生成。这也是为了避免自己的签名影响到自己的生成（这在数学上是没有意义的）。

2. 将此输入在修剪版交易副本中的公钥替换为先前交易对应输出的公钥哈希：这一步是为了正确地构建要被签名的数据。在Bitcoin中，一个输入的签名实际上是对某些数据的签名，这些数据包括输入引用的输出的详情（比如金额和接收者公钥哈希），以及当前交易的其他信息。这一步实际上是在准备这些数据，以便于对其进行签名。

3. 清除修剪版交易副本中此输入的公钥：这一步是为了移除那些不应该被包括在签名过程中的数据。当一个输入被签名时，它自己的公钥字段是空的，因为这个公钥是用于验证签名的，而不是用于生成签名的。同时，这也避免了公钥对自身签名过程的影响。


这个函数的主要作用就是为了验证给定交易的所有输入是否已经被正确地签名。如果所有的输入都通过了验证，那么这个交易就被认为是有效的。否则，这个交易就被认为是无效的。




让我们考虑一种情况：Alice向Bob发送10个比特币，这就需要创建一个新的交易。

1. Alice找到一个未花费的交易输出（UTXO），它包含她之前收到的15个比特币。这就成为新交易的输入。
2. 在交易的输出部分，Alice添加两个输出：一个给Bob（10个比特币），一个找零给她自己（5个比特币）。
3. 现在，Alice需要为这个新交易的输入部分生成一个签名。但是，输入部分本身包含一个签名字段（它现在是空的，因为签名还没生成），并且还有一个公钥字段（包含Alice的公钥，用于验证签名）。
4. Alice为这个新交易创建一个副本，然后在这个副本中清除输入部分的签名字段和公钥字段。接下来，她将输入部分的公钥哈希字段替换为引用的UTXO的公钥哈希（这是一份她自己的公钥哈希的副本，因为这个UTXO是她之前收到的）。
5. 然后，Alice对这个修剪后的交易副本生成一个签名，并把这个签名添加到原始交易的输入部分的签名字段中。

验证过程正好相反：

1. Bob（或者任何其他试图验证交易的人）收到了Alice的交易，然后创建了它的一个副本。
2. 在这个副本中，Bob清除输入部分的签名和公钥字段，并用引用的UTXO的公钥哈希替换公钥哈希字段。
3. 然后，Bob用副本生成一个新的哈希，然后用这个哈希、Alice的公钥（这是公开的）以及交易的签名来验证签名。如果验证通过，这个交易就是有效的。

这样做的目的是确保交易的发送者（Alice）确实拥有她试图消费的比特币（她能够生成一个有效的签名），并且交易的内容在签名生成后没有被更改（否则，签名验证会失败）。


#### impl TXOutput (输出 lock)

在区块链交易中，`锁定交易输出` 实际上就是将交易输出与一个特定的公钥哈希（也就是地址）相关联。这个步骤确保了只有拥有相应私钥的用户才能在将来的交易中使用这个输出。

在实现 `TXOutput` 的 `lock` 函数时，我们将提供的地址编码为公钥哈希( `Address::decode(address)` )，并将其存储在 `pub_key_hash` 属性中。这意味着这个输出已经“锁定”，只有拥有与这个公钥哈希匹配的私钥的用户才能“解锁”它。换句话说，**只有拥有正确私钥的用户才能使用这个输出作为新交易的输入**。

当我们创建一个新的交易输出时，**例如当 Alice 向 Bob 发送一些币时，我们需要“锁定”这个输出到 Bob 的地址。这样，只有 Bob 可以在他自己的交易中使用这个输出**。同样，**找零输出也被锁定到 Alice 自己的地址**，这样她可以在将来的交易中再次使用这个输出。

总的来说，锁定一个交易输出的目的是确定哪个用户可以使用这个输出作为新交易的输入。

#### TXInput TXOutput Transaction 

1. `Transaction`:  代表一次交易，包括交易的ID和交易的输入与输出的列表。
2. `TXInput`:  代表交易的输入，包括用于查找UTXO的交易ID和输出索引，以及用于验证交易的签名和公钥。
3. `TXOutput`:  代表交易的输出，包括交易的值和锁定脚本（用公钥哈希表示）。

想理解这 3 个 struct 的关系, 直接看例子：

Alice 有 `50` 比特币，她想要给 Bob发送 30 比特币 :

首先，Alice 的钱包会创建一个 `Transaction`，并在这个交易中包含 **1 个 `TXInput` 和 2 个 `TXOutput`** 。

 - `TXInput` 是从 Alice 的 UTXO（Unspent Transaction Outputs，未消费的交易输出）中找到的。它将包含 UTXO 的交易 ID 和输出索引，以及 Alice 的公钥和由 Alice 的私钥生成的签名。
 - 接下来，创建 2 个 `TXOutput`。第一个是转给 Bob 的 30 比特币，其 `TXOutput` 的  `value` 为 30，`pub_key_hash`为 Bob 的公钥哈希（通过 Bob 的地址得到的）。第 2 个是 Alice 自己找零给自己的 20 比特币，其中  `value` 为20，`pub_key_hash` 为 Alice 的公钥哈希。



对于 Alice 的交易，可能会有如下的形式：

```rust
let transaction = Transaction {
    id: "d4c0d8d5d6...",  // This would be a real hash
    vin: vec![
        TXInput {
            txid: "abc123...",  // The id of a previous transaction where Alice received 50 BTC
            vout: 0,
            signature: vec![...],  // Alice's signature made with her private key
            pub_key: vec![...],  // Alice's public key
        },
    ],
    vout: vec![
        TXOutput {
            value: 30,
            pub_key_hash: vec![...],  // Bob's public key hash
        },
        TXOutput {
            value: 20,
            pub_key_hash: vec![...],  // Alice's public key hash
        },
    ],
};
```

vout : 交易索引

在这个示例中，`vout`字段表示的是**这个交易输出（TXOutput）在之前交易中的索引位置**。它代表了在某个特定交易中，哪一个输出（output）被用作当前输入（input）。为了指定这个索引，我们需要知道在之前的交易中，哪一个输出现在被用作输入。

在我们的示例中，我们假设了之前的交易只有一个输出，也就是Alice接收到的50比特币。因为计算机编程中的索引通常是从0开始的，所以这个输出的索引就是0。

如果在之前的交易中有多个输出，比如说在交易 `abc123...`中，Alice接收到50比特币作为第一个输出，又接收到25比特币作为第二个输出，那么这两个输出的索引就分别是0和1。如果Alice想要使用后面那个25比特币的输出作为新的交易的输入，那么`vout`就应该是1。

在实际的比特币系统中，一个交易可以有多个输入和输出，所以`vout`是非常重要的，因为它能够指定具体是哪一个输出被用作输入。

### utxoset.rs

`sled::open("data/utxos")?;`  始于 `find_spendable_outputs`  
始于` utxo.find_spendable_outputs`  中的  `new_UTXO` (`transaction.rs`)
始于 `fn cmd_send 中的 Transaction::new_UTXO  `  (`cli.rs`)


(`cli.rs`)

```rust
fn cmd_create_blockchain(address: &str) -> Result<()> {
    let address = String::from(address);
    let bc = Blockchain::create_blockchain(address)?;

    let utxo_set = UTXOSet { blockchain: bc };
    utxo_set.reindex()?;
    println!("create blockchain");
    Ok(())
}
```



### server.rs

> Mutex（互斥锁）是一个并发原语，它保证了同一时间只能有一个线程访问数据。当一个线程想要访问被 Mutex 包裹的数据时，它需要首先获取锁（lock），如果锁已经被另一个线程持有，那么这个线程就会被阻塞直到锁被释放 
>
> `Arc<Mutex<ServerInner>>`：Arc（Atomic Reference Counting，原子引用计数）是一个智能指针，它使得数据能在多个线程之间共享。当 Arc 的引用计数为 0 时，数据会被安全地清理

所以, ServerInner 类型的数据被一个 Mutex 包裹着以确保在同一时间只有一个线程可以访问和修改它，并且这个 Mutex 又被一个 Arc 包裹以使得这个 Mutex 可以在多个线程之间安全地共享

### error.rs

```rust
pub type Result<T> = std::result::Result<T, failure::Error>;
```





### 优化

 `rust-crypto` 库依赖于已经被废弃的 `rustc-serialize` 包，建议使用 serde 代替

或者直接更换 `rust-crypto` 库 为 `ring`  或者其他密码学库，这需要一些调研和修改。