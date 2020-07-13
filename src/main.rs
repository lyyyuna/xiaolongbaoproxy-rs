#![allow(warnings)]

use clap::{App, SubCommand, Arg, value_t};
use std::io::Write;
use log::LevelFilter;

mod proxy;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let matches = App::new("xiaolongbao proxy")
                .author("lyyyuna")
                .subcommand(SubCommand::with_name("basic")
                        .arg(Arg::with_name("listen")
                                .long("listen")
                                .takes_value(true)
                                .short("l")))
                .get_matches();

    if let Some(basic_matches) = matches.subcommand_matches("basic") {
        let addr = basic_matches.value_of("listen").unwrap_or("0.0.0.0:8080");
    
        let p = proxy::Proxy::new(addr);
        p.serve().await
    }
}
