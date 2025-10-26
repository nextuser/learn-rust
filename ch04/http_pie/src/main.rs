use clap::Parser;
#[derive(Parser, Debug)]
struct Get {
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    // HTTP请求的URL（例如：https://example.com）
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

/**
 * usage
 *  cargo run -- -dd  -n myname test
 */
fn main() {
    let opts = Opts::parse();
    println!("parsed args : {:?}", &opts);
}
