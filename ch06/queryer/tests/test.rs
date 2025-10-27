#[cfg(test)]
mod tests {
    use queryer::example_sql;
    use queryer::query;
    #[tokio::test]
    async fn test_query() {
        let sql = example_sql();
        let data = query(sql).await.unwrap();
        let s = data.to_csv();
        match s {
            Ok(s) => println!("test_query :string length {} {}", s.len(), &s),
            Err(e) => panic!("test_query error:{}", e),
        }
    }
}
