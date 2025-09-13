mod args;
mod test;
mod errors;
mod handlers;
mod server;
mod parsing;

use clap::Parser;

use crate::server::create_server;
use crate::args::ProgramArgs;

#[tokio::main]
async fn main() {
    let args = ProgramArgs::parse();

    if let Err(e) = create_server(&format!("0.0.0.0:{}", args.port)).await {
        println!(
            "\x1b[1;31mStartup error: \x1b[0m\x1b[1;37;41m{} on port {}\x1b[0m",
            e,
            args.port
        );
    }
}

