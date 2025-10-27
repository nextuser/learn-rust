use sqlparser::{
    dialect::
        {
            GenericDialect,
            Dialect
        },
    parser::Parser};

fn main(){
    tracing_subscriber::fmt::init();
    let sql = " \
    SELECT a a1,b , 123, myfunc(b),*\
    from data_source \
    where a > b and b < 100 and c between 10 and 20 \
    order by a desc , b \
    limit 50 offset 10";
    let ast = Parser::parse_sql(&GenericDialect::default(),sql);
    println!("{:#?}",ast);
}

#[derive(Debug,Default)]
pub struct MyDialect;

impl Dialect for MyDialect{
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || [':','_','-','.','?','&','=','/'].contains(&ch)
    }
}

pub fn example_sql()->String{
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let sql = format!("select location  name,total_cases,new_cases,total_deaths,new_deaths \
     from {} where new_deaths >= 500 order by new_cases desc limit 6 offset 5",  url);
    sql
}

#[cfg(test)]
mod tests{
    use crate::{example_sql, MyDialect};
    use sqlparser::parser::Parser;
    #[test]
    fn it_works(){
        assert!(Parser::parse_sql(&MyDialect::default(),&example_sql()).is_ok())
    }
}