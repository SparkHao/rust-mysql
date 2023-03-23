use sqlx::{mysql::MySqlRow, Row};



#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let conn = sqlx::mysql::MySqlPoolOptions::new().connect("mysql://root:123456@localhost:3306/ord").await?;
    let result: Vec<MySqlRow> = sqlx::query("SELECT wallet_id FROM wallet_info").fetch_all(&conn).await?;
    for row in result.iter() {
        let wallet_id: String = row.get(0);
        println!("wallet_id: {}", wallet_id);
    }


    Ok(())
}