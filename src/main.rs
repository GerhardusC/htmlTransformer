mod test;
mod errors;
mod handlers;
mod server;
mod parsing;

use crate::server::create_server;

#[tokio::main]
async fn main() {
    if let Err(e) = create_server("0.0.0.0:3000").await {
        println!("\x1b[1;31mStartup error: \x1b[0m\x1b[1;37;41m{}\x1b[0m", e);
    }
}

