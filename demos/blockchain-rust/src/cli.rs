#![allow(warnings)]
#![allow(non_snake_case)]
#![allow(unused)]
use std::process::exit;
use bitcoincash_addr::Address;
use clap::{arg, Command};
use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::server::Server;
use crate::transaction::Transaction;
use crate::utxoset::UTXOSet;
use crate::wallets::{Wallet, Wallets};

pub struct Cli {}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }
    // 每一个 subcommand 对应一个可能的命令，比如 "printchain"，"createwallet"，"listaddresses" 等等
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("behrouz.r.fa@gmail.com")
            .about("blockchain in rust: a simple blockchain for learning")
            .subcommand(Command::new("printchain").about("print all the chain blocks"))
            .subcommand(Command::new("createwallet").about("create a wallet"))
            .subcommand(Command::new("listaddresses").about("list all addresses"))
            .subcommand(Command::new("reindex").about("reindex UTXO"))
            .subcommand(Command::new("getbalance")
                .about("get balance in the blochain")
                .arg(arg!(<ADDRESS>"'The Address it get balance for'"))
            ).subcommand(Command::new("startnode")
            .about("start the node server")
            .arg(arg!(<PORT>"'the port server bind to locally'"))
        )
            .subcommand(Command::new("createblockchain").about("Create new blochain")
                .arg(arg!(<ADDRESS>"'The address to send gensis block reqward to' "))
            )

            .subcommand(
                Command::new("send")
                    .about("send  in the blockchain")
                    .arg(arg!(<FROM>" 'Source wallet address'"))
                    .arg(arg!(<TO>" 'Destination wallet address'"))
                    .arg(arg!(<AMOUNT>" 'Destination wallet address'"))
                    .arg(arg!(-m --mine " 'the from address mine immediately'")),
            )
            .subcommand(
                Command::new("startminer")
                    .about("start the minner server")
                    .arg(arg!(<PORT>" 'the port server bind to locally'"))
                    .arg(arg!(<ADDRESS>" 'wallet address'")),

            )
            .get_matches();

        /// 根据 matches 的结果，调用不同的函数执行相应的命令
        if let Some(ref matches) = matches.subcommand_matches("startminer") {
            let port = if let Some(port) = matches.get_one::<String>("PORT") {
                port
            } else {
                println!("PORT not supply!: usage");
                exit(1)
            };

            let address = if let Some(address) = matches.get_one::<String>("ADDRESS") {
                address
            } else {
                println!("ADDRESS not supply!: usage");
                exit(1)
            };
            let bc = Blockchain::new()?;
            let utxo_set = UTXOSet { blockchain: bc };
            let server = Server::new(port, address, utxo_set)?;
            server.start_server()?;
        }


        if let Some(ref matches) = matches.subcommand_matches("startnode") {
            if let Some(port) = matches.get_one::<String>("PORT") {
                let bc = Blockchain::new()?;
                let utxo_set = UTXOSet { blockchain: bc };
                let server = Server::new(port, "", utxo_set)?;
                server.start_server()?;
            }
        }

        if let Some(_) = matches.subcommand_matches("createwallet") {
            println!("address: {}", cmd_create_wallet()?);
        }

        if let Some(_) = matches.subcommand_matches("reindex") {
            let count = cmd_reindex()?;
            println!("Done! There are {} transactions in the UTXO set.", count);
        }

        if let Some(_) = matches.subcommand_matches("listaddresses") {
            cmd_list_address()?;
        }

        if let Some(ref matches) = matches.subcommand_matches("createblockchain") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                cmd_create_blockchain(address)?;
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let balance = cmd_get_balance(address)?;
                println!("Balance: {}\n", balance);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            if matches.contains_id("mine") {
                cmd_send(from, to, amount, true)?;
            } else {
                cmd_send(from, to, amount, false)?;
            }


            /*else {
                println!("Not printing testing lists...");
            }*/
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            cmd_print_chain()?;
        }

        Ok(())
    }
}

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

fn cmd_create_wallet() -> Result<String> {
    let mut ws = Wallets::new()?; // Wallets 是钱包集合
    let address = ws.create_wallet();
    ws.save_all()?;
    Ok(address)
}

fn cmd_reindex() -> Result<i32> {
    let bc = Blockchain::new()?;
    let utxo_set = UTXOSet { blockchain: bc };
    utxo_set.reindex()?;
    utxo_set.count_transactions()
}

fn cmd_create_blockchain(address: &str) -> Result<()> {
    let address = String::from(address);
    let bc = Blockchain::create_blockchain(address)?;

    let utxo_set = UTXOSet { blockchain: bc };
    utxo_set.reindex()?;
    println!("create blockchain");
    Ok(())
}

fn cmd_get_balance(address: &str) -> Result<i32> {
    let pub_key_hash = Address::decode(address).unwrap().body;
    let bc = Blockchain::new()?;
    let utxo_set = UTXOSet { blockchain: bc };
    let utxos = utxo_set.find_UTXO(&pub_key_hash)?;

    let mut balance = 0;
    for out in utxos.outputs {
        balance += out.value;
    }
    Ok(balance)
}

fn cmd_print_chain() -> Result<()> {
    let bc = Blockchain::new()?;
    for b in bc.iter() {
        println!("{:#?}", b);
    }
    Ok(())
}

fn cmd_list_address() -> Result<()> {
    let ws = Wallets::new()?;
    let addresses = ws.get_all_addresses();
    println!("addresses: ");
    for ad in addresses {
        println!("{}", ad);
    }
    Ok(())
}