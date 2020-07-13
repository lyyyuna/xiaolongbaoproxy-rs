#![deny(warnings)]

use clap::{App, SubCommand, Arg, value_t};
mod proxy;

#[tokio::main]
async fn main() {
    let matches = App::new("xiaolongbao proxy")
                .version("0.0.1")
                .author("lyyyuna")
                .subcommand(SubCommand::with_name("basic")
                        .arg(Arg::with_name("server")
                                .long("server")
                                .takes_value(true)
                                .short("s"))
                        .arg(Arg::with_name("port")
                                .long("port")
                                .takes_value(true)
                                .short("p")))
                .get_matches();

    if let Some(basic_matches) = matches.subcommand_matches("basic") {
        let server = basic_matches.value_of("server").unwrap_or("0.0.0.0");
        let port = value_t!(basic_matches.value_of("port"), i32).unwrap_or(8080);
    
        let p = proxy::Proxy::new(server, port);
        p.serve();
    }
}
