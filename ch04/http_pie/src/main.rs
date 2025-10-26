use clap::Parser;
#[derive(Parser, Debug)]
struct Get {
    #[arg(value_parser = parse_url)]
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    // HTTP请求的URL（例如：https://example.com）
    #[arg(value_parser = parse_url)]
    url: String,
    /// HTTP请求的body参数，格式为key=value（可多个）
    /// 例如：name=Alice age=30
    body: Vec<String>,
}

#[derive(Parser, Debug)]
enum HttpCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    http_cmd: HttpCommand,
}
use anyhow::Result;
use reqwest::Url;
//use std::error::Error;
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = Url::parse(s)?;
    Ok(s.into())
}

#[test]
fn test_parse_url() {
    let mut url = parse_url("https://www.google.com");
    println!("{:?}", url);

    url = parse_url("https://abc.com");
    println!("{:?}", url);
}

/**
 * usage
 *  cargo run -- -dd  -n myname test
 */
fn main() {
    let opts = Opts::parse();
    println!("parsed args : {:?}", &opts);

    match opts.http_cmd {
        HttpCommand::Get(Get { url }) => {
            println!("http cmd get url: {}", url);
        }
        HttpCommand::Post(Post { url, body }) => {
            println!("http cmd post url: {}, body: {:?}", url, body);
        }
    }
}
