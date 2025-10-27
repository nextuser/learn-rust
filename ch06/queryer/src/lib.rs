use polars::frame::DataFrame;
use polars::prelude::*;
use sqlparser::parser::Parser;
use std::ops::{Deref, DerefMut};

mod loader;
use tracing::info;
mod convert;
mod dialect;
use dialect::MyDialect;
pub use dialect::example_sql;

use crate::fetcher::retrieve_data;
//mod dialect;
mod fetcher;

#[derive(Debug)]
pub struct DataSet(DataFrame);

impl Deref for DataSet {
    type Target = DataFrame;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSet {
    pub fn to_csv(&self) -> anyhow::Result<String> {
        let mut buf = Vec::new();
        let writer = CsvWriter::new(&mut buf);
        let _ = writer.finish(self);
        Ok(String::from_utf8(buf)?)
    }
}

pub async fn query<T: AsRef<str>>(sql: T) -> anyhow::Result<DataSet> {
    let sql = sql.as_ref();
    let ast = Parser::parse_sql(&MyDialect::default(), sql)?;
    if ast.len() != 1 {
        return Err(anyhow::anyhow!("only support single sql statement"));
    }

    let sql = &ast[0];
    let convert::Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;
    info!("retrieve data from source {:?}", source);

    let ds = loader::detect_content(retrieve_data(source).await?).load()?;
    let mut filtered = match condition {
        Some(condition) => ds.0.lazy().filter(condition),
        None => ds.0.lazy(),
    };

    filtered = order_by
        .into_iter()
        .fold(filtered, |acc, (col, desc)| acc.sort(&col, desc));
    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX));
    }

    Ok(DataSet(filtered.select(selection).collect()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
