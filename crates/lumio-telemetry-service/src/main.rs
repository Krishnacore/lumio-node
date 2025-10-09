#![forbid(unsafe_code)]

// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use lumio_telemetry_service::LumioTelemetryServiceArgs;
use clap::Parser;

#[tokio::main]
async fn main() {
    lumio_logger::Logger::new().init();
    LumioTelemetryServiceArgs::parse().run().await;
}
