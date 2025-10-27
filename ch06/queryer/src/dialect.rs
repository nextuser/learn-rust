#[derive(Debug, Default)]
pub struct MyDialect;

use sqlparser::dialect::Dialect;

impl Dialect for MyDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || [':', '_', '-', '.', '?', '&', '=', '/'].contains(&ch)
    }
}
//const URL_COVID: &str = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
const URL_COVID: &str = "https://gitee.com/nextuser/todo_android/raw/master/covid-5k.csv";

pub fn example_sql() -> String {
    let url = URL_COVID;
    let sql = format!(
        "select location  name,total_cases,new_cases,total_deaths,new_deaths \
     from {} where new_deaths >= 500 order by new_cases desc limit 6 offset 5",
        url
    );
    sql
}

#[cfg(test)]
mod tests {
    use super::{MyDialect, example_sql};
    use sqlparser::parser::Parser;
    #[test]
    fn it_works() {
        assert!(Parser::parse_sql(&MyDialect::default(), &example_sql()).is_ok())
    }
}
