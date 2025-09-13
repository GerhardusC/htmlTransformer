use clap_derive::Parser;

/// You may choose to specify a port, otherwise the application will default
/// to serving on port 3000.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ProgramArgs {
    /// Port to serve on
    #[arg(short, long, default_value_t = 3000)]
    pub port: u32,
}
