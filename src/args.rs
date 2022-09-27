use std::net::IpAddr;
use clap::Parser;
use url::Url;

/// Make sure JCloud instances stay online!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to flow.yml (jc deploy will be called if an instance does not exist) (it not defined, alternate_url will be used by default)
    #[clap(short, long, value_parser)]
    pub flow_yml_path: Option<String>,

    /// Current Jina Cloud URL for instance of choice if its already running (if any)
    #[clap(short, long, value_parser)]
    pub current_jcloud_url: Option<Url>,

    /// If Jina Cloud is offline or if you simply want your own URL without JCloud
    #[clap(short, long, value_parser)]
    pub alternate_url: Option<Url>,

    /// Port number
    #[clap(short, long, value_parser, default_value_t = 8080)]
    pub port: u16,

    /// Host address (127.0.0.1 by default, to avoid a public service in case you use a reverse proxy like nginx. You can set this to 0.0.0.0 otherwise!)
    #[clap(short, long, value_parser, default_value = "127.0.0.1")]
    pub host: IpAddr,
}