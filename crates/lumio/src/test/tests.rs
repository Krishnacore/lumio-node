// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    move_tool::{ArgWithType, FunctionArgType},
    CliResult, Tool,
};
use clap::Parser;
use std::str::FromStr;

/// In order to ensure that there aren't duplicate input arguments for untested CLI commands,
/// we call help on every command to ensure it at least runs
#[tokio::test]
async fn ensure_every_command_args_work() {
    assert_cmd_not_panic(&["lumio"]).await;

    assert_cmd_not_panic(&["lumio", "account"]).await;
    assert_cmd_not_panic(&["lumio", "account", "create", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "account", "create-resource-account", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "account", "fund-with-faucet", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "account", "list", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "account", "lookup-address", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "account", "rotate-key", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "account", "transfer", "--help"]).await;

    assert_cmd_not_panic(&["lumio", "config"]).await;
    assert_cmd_not_panic(&["lumio", "config", "generate-shell-completions", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "config", "init", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "config", "set-global-config", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "config", "show-global-config"]).await;
    assert_cmd_not_panic(&["lumio", "config", "show-profiles"]).await;

    assert_cmd_not_panic(&["lumio", "genesis"]).await;
    assert_cmd_not_panic(&["lumio", "genesis", "generate-genesis", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "genesis", "generate-keys", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "genesis", "generate-layout-template", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "genesis", "set-validator-configuration", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "genesis", "setup-git", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "genesis", "generate-admin-write-set", "--help"]).await;

    assert_cmd_not_panic(&["lumio", "governance"]).await;
    assert_cmd_not_panic(&["lumio", "governance", "execute-proposal", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "governance", "generate-upgrade-proposal", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "governance", "propose", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "governance", "vote", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "governance", "delegation_pool", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "governance", "delegation_pool", "vote", "--help"]).await;
    assert_cmd_not_panic(&[
        "lumio",
        "governance",
        "delegation_pool",
        "propose",
        "--help",
    ])
    .await;

    assert_cmd_not_panic(&["lumio", "info"]).await;

    assert_cmd_not_panic(&["lumio", "init", "--help"]).await;

    assert_cmd_not_panic(&["lumio", "key"]).await;
    assert_cmd_not_panic(&["lumio", "key", "generate", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "key", "extract-peer", "--help"]).await;

    assert_cmd_not_panic(&["lumio", "move"]).await;
    assert_cmd_not_panic(&["lumio", "move", "clean", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "compile", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "compile-script", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "decompile", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "disassemble", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "download", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "init", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "list", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "prove", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "publish", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "run", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "run-script", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "test", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "transactional-test", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "move", "view", "--help"]).await;

    assert_cmd_not_panic(&["lumio", "node"]).await;
    assert_cmd_not_panic(&["lumio", "node", "check-network-connectivity", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "get-stake-pool", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "analyze-validator-performance", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "bootstrap-db-from-backup", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "initialize-validator", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "join-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "leave-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "run-local-testnet", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "show-validator-config", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "show-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "show-validator-stake", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "node", "update-consensus-key", "--help"]).await;
    assert_cmd_not_panic(&[
        "lumio",
        "node",
        "update-validator-network-addresses",
        "--help",
    ])
    .await;

    assert_cmd_not_panic(&["lumio", "stake"]).await;
    assert_cmd_not_panic(&["lumio", "stake", "add-stake", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "stake", "increase-lockup", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "stake", "initialize-stake-owner", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "stake", "set-delegated-voter", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "stake", "set-operator", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "stake", "unlock-stake", "--help"]).await;
    assert_cmd_not_panic(&["lumio", "stake", "withdraw-stake", "--help"]).await;
}

/// Ensure we can parse URLs for args
#[tokio::test]
async fn ensure_can_parse_args_with_urls() {
    let result = ArgWithType::from_str("string:https://lumiolabs.com").unwrap();
    matches!(result._ty, FunctionArgType::String);
    assert_eq!(
        result.arg,
        bcs::to_bytes(&"https://lumiolabs.com".to_string()).unwrap()
    );
}

async fn assert_cmd_not_panic(args: &[&str]) {
    // When a command fails, it will have a panic in it due to an improperly setup command
    // thread 'main' panicked at 'Command propose: Argument names must be unique, but 'assume-yes' is
    // in use by more than one argument or group', ...

    match run_cmd(args).await {
        Ok(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
        Err(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
    }
}

async fn run_cmd(args: &[&str]) -> CliResult {
    let tool: Tool = Tool::try_parse_from(args).map_err(|msg| msg.to_string())?;
    tool.execute().await
}
