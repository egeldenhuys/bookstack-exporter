use bookstack_exporter::BookstackClient;
use std::{error::Error, process::exit};

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let conf_result: Result<args::Conf, confique::Error> = args::load();

    let conf: args::Conf = match conf_result {
        Ok(res) => res,
        Err(error) => {
            println!("{:?}", error);
            exit(1)
        }
    };

    let client: BookstackClient = BookstackClient::new(
        &conf.bookstack_host,
        &conf.bookstack_api_token_id,
        &conf.bookstack_api_token_secret,
    );
    client.clone_bookstack(&bookstack_exporter::ExportType::HTML, &conf.output_dir)?;

    Ok(())
}
