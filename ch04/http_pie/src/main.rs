use anyhow::{Result, anyhow};
use clap::Parser;
use reqwest::Url;

#[derive(Parser, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

// get 子命令

#[derive(Parser, Debug)]
struct Post {
    // HTTP请求的URL（例如：https://example.com）
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP请求的body参数，格式为key=value（可多个）
    /// 例如：name=Alice age=30
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Parser, Debug)]
enum HttpCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    #[clap(subcommand)]
    http_cmd: HttpCommand,
}

//use std::error::Error;
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = Url::parse(s)?;
    Ok(s.into())
}
#[derive(Debug, PartialEq)]
struct KvPair {
    key: String,
    value: String,
}
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
    // let mut split = s.split("=");

    // let key = split
    //     .next()
    //     .ok_or_else(|| anyhow!("invalid key-value pair: {}", s))?;
    // let value = split
    //     .next()
    //     .ok_or_else(|| anyhow!("invalid key-value pair: {}", s))?;

    // Ok(KvPair {
    //     key: key.into(),
    //     value: value.into(),
    // })
}

use std::str::FromStr;

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");

        let err_fn = || anyhow!(format!("Failed to parse {}", s));
        let key = split.next().ok_or_else(err_fn)?.to_string();
        let value = split.next().ok_or_else(err_fn)?.to_owned();
        Ok(Self {
            key: key.into(),
            value: value.into(),
        })
    }
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
