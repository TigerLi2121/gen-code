use crate::db;
use dotenv;
use serde::Serialize;
use sqlx::{Error, FromRow, MySql, QueryBuilder};

#[derive(Debug, FromRow, Serialize)]
pub struct TableInfo {
    #[sqlx(rename = "TABLE_NAME")]
    pub table_name: String,
    #[sqlx(rename = "TABLE_COMMENT")]
    pub table_comment: String,

    #[sqlx(skip)]
    pub table_columns: Vec<ColumnInfo>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct ColumnInfo {
    #[sqlx(rename = "COLUMN_NAME")]
    pub column_name: String,
    #[sqlx(rename = "DATA_TYPE")]
    pub data_type: String,
    #[sqlx(rename = "COLUMN_COMMENT")]
    pub column_comment: String,
    #[sqlx(rename = "COLUMN_KEY")]
    pub column_key: String,
}

pub async fn table_infos() -> Result<Vec<TableInfo>, Error> {
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    // 获取数据库名
    println!("database_url: {}", database_url);
    let db_name = database_url.split("/").last().unwrap();
    println!("db_name: {}", db_name);
    let table_module = dotenv::var("TABLE_MODULE").unwrap();
    // 获取表名
    let table_names = table_module
        .split(",")
        .map(|s| s.split(":").next().unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    println!("table_names: {:?}", table_names);

    let mut db_sql: QueryBuilder<MySql> = QueryBuilder::new(
        "SELECT table_name,table_comment FROM information_schema.tables WHERE table_schema = ",
    );
    db_sql.push_bind(db_name);
    db_sql.push(" AND table_name IN (");
    let mut ts = db_sql.separated(", ");
    for tn in table_names.iter() {
        ts.push_bind(tn);
    }
    ts.push_unseparated(") ");
    // println!("db_sql: {}", db_sql.sql());

    // 查询表信息
    let mut tis: Vec<TableInfo> = db_sql
        .build_query_as()
        .fetch_all(db::get_pool().unwrap())
        .await
        .expect("查询表信息失败");
    // println!("tis: {:?}", tis);

    // 查询列信息
    for ti in tis.iter_mut() {
        let mut table_sql:QueryBuilder<MySql> = QueryBuilder::new(
            "SELECT column_name,data_type,column_comment,column_key FROM information_schema.columns WHERE table_schema = "
        );
        table_sql.push_bind(db_name);
        table_sql.push(" AND table_name = ");
        table_sql.push_bind(ti.table_name.clone());
        let cis: Vec<ColumnInfo> = table_sql
            .build_query_as()
            .fetch_all(db::get_pool().unwrap())
            .await
            .expect("查询列信息失败");
        // println!("cis: {:?}", cis);
        ti.table_columns = cis;
    }
    Ok(tis)
}
