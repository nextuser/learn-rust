use anyhow::{Result, anyhow};
use clap::Parser;
use colored::Colorize;
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
}

use std::collections::HashMap;
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

fn print_headers(resp: &Response) {
    println!("Header: ");
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name, value);
    }
    println!("");
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(pair.key.clone(), pair.value.clone());
    }
    let resp = client.post(&args.url).json(&body).send().await.unwrap();
    Ok(print_resp(resp).await?)
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

use mime::Mime;
use reqwest::header;
fn get_content_type(resp: &Response) -> Option<Mime> {
    let content_type = resp.headers().get(header::CONTENT_TYPE);
    content_type.map(|s| s.to_str().unwrap().parse().unwrap())
}
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use syntect::util::as_24_bit_terminal_escaped;
fn print_syntect(body: &str, ext: &str) {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ss.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(body) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ss);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

fn print_body(m: Option<Mime>, body: &str) {
    match m {
        Some(m) => {
            if m == mime::TEXT_HTML {
                print_syntect(body, "html");
            } else if m == mime::APPLICATION_JSON {
                print_syntect(body, "json");
            } else {
                println!("{}", body.blue());
            }
        }

        None => println!("{}", body.blue()),
    }
}

use reqwest::Client;
use reqwest::Response;
async fn print_resp(response: Response) -> Result<()> {
    print_status(&response);
    print_headers(&response);
    let mime = get_content_type(&response);
    let body = response.text().await?;
    print_body(mime, &body);
    Ok(())
}
async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client
        .get(&args.url)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to connet to server: {}", e))?;
    Ok(print_resp(resp).await?)
}

/**
 * usage
 *  cargo run -- -dd  -n myname test
 */

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("parsed args : {:?}", &opts);

    let client = Client::new();
    let result = match opts.http_cmd {
        HttpCommand::Get(ref args) => get(client, args).await?,
        HttpCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}

//fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.com").is_ok());
        assert!(parse_url("http://dd.com/post").is_ok());
    }

    #[test]
    fn parse_args_works() {
        assert!(parse_kv_pair("abc").is_err());
        assert_eq!(
            parse_kv_pair("abc=").unwrap(),
            KvPair {
                key: "abc".to_string(),
                value: "".to_string()
            }
        );
        assert_eq!(
            parse_kv_pair("a=").unwrap(),
            KvPair {
                key: "a".to_string(),
                value: "".to_string()
            }
        );
    }
}
