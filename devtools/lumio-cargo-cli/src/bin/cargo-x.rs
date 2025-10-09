// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use lumio_cargo_cli::{LumioCargoCommand, SelectedPackageArgs};
use clap::Parser;
use std::process::exit;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    #[command(name = "x")]
    LumioCargoTool(LumioCargoToolArgs),
}

#[derive(Parser)]
struct LumioCargoToolArgs {
    #[command(subcommand)]
    cmd: LumioCargoCommand,
    #[command(flatten)]
    package_args: SelectedPackageArgs,
}

fn main() {
    let CargoCli::LumioCargoTool(args) = CargoCli::parse();
    let LumioCargoToolArgs { cmd, package_args } = args;
    let result = cmd.execute(&package_args);

    // At this point, we'll want to print and determine whether to exit for an error code
    match result {
        Ok(_) => {},
        Err(inner) => {
            println!("{}", inner);
            exit(1);
        },
    }
}
