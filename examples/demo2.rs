use std::{time::{SystemTime, UNIX_EPOCH}};

use sqlx::{mysql::MySqlRow, Row};



#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let conn = sqlx::mysql::MySqlPoolOptions::new().connect("mysql://root:123456@localhost:3306/ord").await?;
    let result: Vec<MySqlRow> = sqlx::query("SELECT wallet_id FROM wallet_info where wallet_id = ?").bind("wallet167841869333700547061").fetch_all(&conn).await?;
    for row in result.iter() {
        let wallet_id: String = row.get(0);
        println!("wallet_id: {}", wallet_id);
        // let delete = sqlx::query("DELETE FROM wallet_info where wallet_id = ?")
        //     .bind(wallet_id)
        //     .execute(&conn)
        //     .await?;
        let delete = sqlx::query("DELETE FROM wallet_info where wallet_id = ?")
            .bind(wallet_id)
            .fetch_all(&conn)
            .await?;
        println!("delete: {:?}", delete);        
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let insert = sqlx::query("insert into wallet_info values(?,?,?,?)")
            .bind(0)
            .bind(format!("wallet{}", now))
            .bind(format!("test-{}", now))
            .bind(now.to_string())
            .fetch_all(&conn)
            .await?;
    println!("insert result: {:?}", insert);

    let query = sqlx::query("SELECT * FROM wallet_info")
        .fetch_all(&conn)
        .await?;

    for row in query {
        let id: i64 = row.get(0);
        let wallet_id: String = row.get(1);
        let address: String = row.get(2);
        let create_time: i64 = row.get(3);
        println!("{:?}, {:?}, {:?}, {:?}", id, wallet_id, address, create_time);
    }
    

    Ok(())
}





