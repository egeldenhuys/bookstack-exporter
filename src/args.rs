use bookstack_exporter::ExportType;
use clap::Parser;
use confique::{Config, File, FileFormat, Partial};

// TODO(evert): Generate sample config using confique
const AFTER_LONG_HELP: &str = color_print::cstr!(
    r#"<bold><underline>Examples:</underline></bold>

    Load all config values from a custom config file path,
    and override export type to use the command line argument instead

    <dim>$</dim> <bold>bookstack-exporter --config-path settings.toml --export-type pdf</bold>

    <bold>settings.toml:</bold>
        bookstack_host = "https://docs.example.com"
        output_dir = "export"
        export_type = "html"
        bookstack_api_token_id = "<<token_id>>"
        bookstack_api_token_secret = "<<token_secret>>"
"#
);

#[derive(Parser)]
#[command(author, version, about, long_about = None, after_long_help = AFTER_LONG_HELP)]
pub struct Args {
    /// Bookstack Host. Example: docs.example.com
    #[arg(long = "host")]
    pub bookstack_host: Option<String>,

    /// Type of export to perform. Required unless set in the config file.
    #[arg(short, long)]
    pub export_type: Option<ExportType>,

    /// Directory to export Bookstack to
    #[arg(short, long)]
    pub output_dir: Option<String>,

    /// Bookstack API Token ID
    ///
    /// Can also be set with the environment variable BOOKSTACK_API_TOKEN_ID
    #[arg(short = 'i', long = "api-id")]
    pub bookstack_api_token_id: Option<String>,

    /// Bookstack API Token Secret
    ///
    /// Can also be set with the environment variable BOOKSTACK_API_TOKEN_SECRET
    #[arg(short = 's', long = "api-secret")]
    pub bookstack_api_token_secret: Option<String>,

    // /// Optional proxy to use. Example: socks5h://user:password@proxy.example.com:1080
    // #[arg(short, long)]
    // pub proxy_url: Option<String>,

    /// Optional config file
    #[arg(short, long)]
    pub config_path: Option<String>,
}

#[derive(Config)]
pub struct Conf {
    pub bookstack_host: String,
    pub export_type: ExportType,
    pub output_dir: String,
    #[config(env = "BOOKSTACK_API_TOKEN_ID")]
    pub bookstack_api_token_id: String,
    #[config(env = "BOOKSTACK_API_TOKEN_SECRET")]
    pub bookstack_api_token_secret: String,
    // pub proxy_url: Option<String>,
    pub config_path: Option<String>,
}
type PartialConf = <Conf as Config>::Partial;

pub fn load() -> Result<Conf, confique::Error> {
    let args = Args::parse();

    // Args is first priority
    let mut merged = PartialConf {
        bookstack_host: args.bookstack_host,
        output_dir: args.output_dir,
        bookstack_api_token_id: args.bookstack_api_token_id,
        bookstack_api_token_secret: args.bookstack_api_token_secret,
        export_type: args.export_type,
        // proxy_url: args.proxy_url,
        config_path: args.config_path.clone(),
    };

    // Env is second priority
    merged = merged.with_fallback(PartialConf::from_env()?);

    // Config file, is third priority
    if let Some(config_path) = args.config_path {
        let from_file = File::with_format(config_path, FileFormat::Toml)
            .required()
            .load()?;

        merged = merged.with_fallback(from_file);
    }

    Conf::from_partial(merged)
}
