mod args;
mod errors;
mod handlers;
mod parsing;
mod server;
mod test;

use clap::Parser;

use crate::args::ProgramArgs;
use crate::server::create_server;

#[tokio::main]
async fn main() {
    let args = ProgramArgs::parse();

    println!("\x1b[1;32mServing on port: \x1b[0m\x1b[1;37m{}\x1b[0m", args.port);
    if let Err(e) = create_server(&format!("0.0.0.0:{}", args.port)).await {
        println!(
            "\x1b[1;31mStartup error: \x1b[0m\x1b[1;37;41m{} on port {}\x1b[0m",
            e, args.port
        );
   }
}
