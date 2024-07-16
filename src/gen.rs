use std::collections::HashMap;

use crate::table::{ColumnInfo, TableInfo};

use dotenv;

pub struct ClassInfo {
    pub pkg_name: String,
    pub module_name: String,
    pub author: String,
    pub table_name: String,
    pub class_name: String,
    pub class_name_fl: String, // 首字母小写
    pub comment: String,
    pub date_time: String,

    pub attributes: Vec<Attribute>,
}

pub struct Attribute {
    pub pk: bool,
    pub column_name: String,
    pub attribute_name: String,
    pub attribute_name_fl: String, // 首字母小写
    pub attribute_type: String,
    pub comment: String,
}

pub fn gen(tis: Vec<TableInfo>) -> Vec<ClassInfo> {
    let mut cis: Vec<ClassInfo> = Vec::new();
    let pkg_url = dotenv::var("PACKAGE_NAME").unwrap();
    let table_module = dotenv::var("TABLE_MODULE").unwrap();
    let table_module_map: HashMap<_, _> = table_module
        .split(",")
        .map(|s| {
            let mut ps = s.split(":");
            (
                ps.next().unwrap().to_string(),
                ps.next().unwrap().to_string(),
            )
        })
        .collect();
    cis
}
