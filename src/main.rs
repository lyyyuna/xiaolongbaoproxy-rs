#![deny(warnings)]

use clap::{App, SubCommand};

#[tokio::main]
async fn main() {
    let _ = App::new("xiaolongbao proxy")
                .version("0.0.1")
                .author("lyyyuna")
                .subcommand(SubCommand::with_name("basic"))
                .get_matches();
}
