use clap::Parser;

/// Make sure JCloud instances stay online!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to flow.yml (jc deploy will be called if not exists)
    #[clap(short, long, value_parser)]
    pub flow_yml_path: String,

    /// Current Jina Cloud URL for instance of choice if its already running (if any)
    #[clap(short, long, value_parser)]
    pub current_jcloud_url: Option<String>,

    /// Port number
    #[clap(short, long, value_parser, default_value_t = 8080)]
    pub port: u16,
}