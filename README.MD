sqlx <--> mysql
Rust use sqlx connect mysql

数据库连接方式：
1、 let conn = sqlx::mysql::MySqlPoolOptions::new().connect("mysql://xxx:xxxx@localhost:3306/xxx").await?;
2、 let conn = sqlx::mysql::MySqlPool::connect("mysql://xxx:xxxx@localhost:3306/xxx").await?;

查询：
1、    let result: Vec<MySqlRow> = sqlx::query("SELECT wallet_id FROM wallet_info where wallet_id = ?").bind("wallet167842097596300844844").fetch_all(&conn).await?;
