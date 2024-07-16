use crate::db;
use dotenv;
use sqlx::{Error, FromRow, MySql, QueryBuilder};

#[derive(Debug, FromRow)]
pub struct TableInfo {
    pub table_name: String,
    pub table_comment: String,

    #[sqlx(skip)]
    pub table_columns: Vec<ColumnInfo>,
}

#[derive(Debug, FromRow)]
pub struct ColumnInfo {
    pub column_name: String,
    pub data_type: String,
    pub column_comment: String,
    pub column_key: String,
}

pub async fn table_infos() -> Result<Vec<TableInfo>, Error> {
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    // 获取数据库名
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
    let mut table_separated = db_sql.separated(", ");
    for table_name in table_names.iter() {
        table_separated.push_bind(table_name);
    }
    table_separated.push_unseparated(") ");
    // 查询表信息
    let mut tis: Vec<TableInfo> = db_sql
        .build_query_as()
        .fetch_all(db::get_pool().unwrap())
        .await?;
    println!("tis: {:?}", tis);

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
            .await?;
        println!("cis: {:?}", cis);
        ti.table_columns = cis;
    }
    Ok(tis)
}
