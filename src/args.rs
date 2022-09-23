use clap::Parser;

/// Make sure JCloud instances stay online!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to flow.yml (jc deploy will be called if not exists)
    #[clap(short, long, value_parser)]
    flow_yml_path: String,

    /// Current Jina Cloud ID for instance of choice if its already running (if any)
    #[clap(short, long, value_parser)]
    current_jcloud_id: Option<String>,

    /// Port number
    #[clap(short, long, value_parser, default_value_t = 8080)]
    port: u16,
}