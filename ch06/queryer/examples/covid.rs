use anyhow::Result;
use queryer::query;
//const URL_COVID: &str = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
const URL_COVID: &str = "https://gitee.com/nextuser/todo_android/raw/master/covid-5k.csv";
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let url = URL_COVID;
    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 500 ORDER BY new_cases DESC limit 20 ",
        url
    );

    let df1 = query(sql).await?;
    //dataset 不能直接输出，直接输出会crash，需要转换成csv输出
    println!("{}", df1.to_csv()?);

    Ok(())
}
