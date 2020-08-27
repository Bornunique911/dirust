extern crate hyper;
use hyper::Client;

use structopt::StructOpt;
#[derive(StructOpt, Debug)]
struct Opt {
    /// Target url
    #[structopt(short, long)]
    url: String,

    /// Set extensions, separated by a comma (E.g. php,html,js)
    #[structopt(short, long, use_delimiter = true)]
    extensions: Option<Vec<u16>>,

    /// Select the wordlist you would like to use (Full file path)
    #[structopt(short, long)]
    wordlist: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let url = Opt::from_args();
    println!("{:?}", url);

    let client = Client::new();
    let uri = "http://scanme.nmap.org/".parse()?;
    let resp = client.get(uri).await?;
    println!("Response: {}", resp.status());

    Ok(())
}
