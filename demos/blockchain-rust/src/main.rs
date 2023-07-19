#![allow(warnings)]
use crate::cli::Cli;
use crate::errors::Result;

mod block;

mod errors;
mod blockchain;
mod cli;
mod transaction;
mod wallets;
mod utxoset;
mod server;

/// 入口点 main 函数
fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())

}