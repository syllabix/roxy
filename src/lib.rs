//! # roxy
//!
//! roxy is a local development web proxy.
//! it enables users to configure routes, certificates, and plugins that can
//! process network traffic and forward it to another destintion on your development
//! machine
//!
mod controller;
mod proxy;

use clap::{Args, Parser, Subcommand, ValueEnum};
use env_logger::Env;
use log::LevelFilter;
use std::{convert::Infallible, ops::Add};

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
struct Commander {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start up the local development proxy
    Start(StartArgs),
    /// Add various entities to a running proxy
    Add { entity: Entity } ,
}

#[derive(Args, Debug)]
struct StartArgs {
    #[arg(default_value_t = 3128)]
    port: u16,
}

#[derive(Clone, ValueEnum)]
enum Entity {
    /// Add a route to the proxy
    Route,
    /// Add a global plugin to the proxy (that will be executed on each route)
    Plugin,
    /// Add a certificate to the proxy
    Certificate
}

pub async fn run() -> Result<(), Infallible> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let args = Commander::parse();

    match &args.command {
        Commands::Start(args) => {
            log::info!("starting up the proxy with args {:?}", args);
            Ok(())
        }
        Commands::Add { entity } => match entity {
            Entity::Route => {
                log::info!("add route");
                Ok(())
            },
            Entity::Plugin => {
                log::info!("add plugin");
                Ok(())
            },
            Entity::Certificate => {
                log::info!("add certificate");
                Ok(())
            },
        },
    }
}
