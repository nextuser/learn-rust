use anyhow::{Result, anyhow};
use async_trait::async_trait;
use tokio::fs;

#[async_trait]
pub trait Fetch {
    async fn fetch(&self) -> anyhow::Result<String>;
}

pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();
    match &name[..4] {
        "http" => UrlFetcher(name).fetch().await,
        "file" => FileFetcher(name).fetch().await,
        _ => Err(anyhow!("Invalid source")),
    }
}

struct UrlFetcher<'a>(&'a str);
struct FileFetcher<'a>(&'a str);

#[async_trait]
impl<'a> Fetch for UrlFetcher<'a> {
    async fn fetch(&self) -> anyhow::Result<String> {
        let url = self.0;
        let client = reqwest::Client::new();
        let resp = client.get(url).send().await?;
        let text = resp.text().await?;
        Ok(text)
    }
}

#[async_trait]
impl<'a> Fetch for FileFetcher<'a> {
    async fn fetch(&self) -> anyhow::Result<String> {
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}
