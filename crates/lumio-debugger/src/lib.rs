// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub enum Cmd {
    #[clap(subcommand)]
    LumioDb(lumio_db_tool::DBTool),

    Decode(lumio_move_debugger::bcs_txn_decoder::Command),

    DumpPendingTxns(lumio_consensus::util::db_tool::Command),

    #[clap(subcommand)]
    Move(lumio_move_debugger::common::Command),
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        match self {
            Cmd::LumioDb(cmd) => cmd.run().await,
            Cmd::Decode(cmd) => cmd.run().await,
            Cmd::DumpPendingTxns(cmd) => cmd.run().await,
            Cmd::Move(cmd) => cmd.run().await,
        }
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    Cmd::command().debug_assert()
}
