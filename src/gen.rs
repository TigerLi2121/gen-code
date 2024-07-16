use crate::table::TableInfo;
use serde::Serialize;
use sqlx::types::chrono::Local;
use std::collections::HashMap;

use dotenv;

const ATTRIBUTE_TYPE_MAP: &[(&str, &str)] = &[
    ("varchar", "String"),
    ("datetime", "Date"),
    ("int", "Integer"),
    ("tinyint", "Integer"),
    ("bigint", "Long"),
];

#[derive(Debug, Serialize)]
pub struct ClassInfo {
    pub date_time: String,
    pub author: String,
    pub pkg_name: String,
    pub module_name: String,
    pub table_name: String,
    pub class_name: String,
    pub class_name_fl: String, // 首字母小写
    pub comment: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Serialize)]
pub struct Attribute {
    pub pk: bool,
    pub column_name: String,
    pub attribute_name: String,
    pub attribute_name_fl: String, // 首字母小写
    pub attribute_type: String,
    pub comment: String,
}

pub fn tpl_data(tis: Vec<TableInfo>) -> Vec<ClassInfo> {
    let mut cis: Vec<ClassInfo> = Vec::new();
    let now = Local::now().format("%Y-%m-%d").to_string();
    println!("now: {}", now);
    let author = dotenv::var("AUTHOR").unwrap();
    println!("author: {}", author);
    let pkg_url = dotenv::var("PACKAGE_URL").unwrap();
    println!("pkg_url: {}", pkg_url);
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
    println!("table_module_map: {:?}", table_module_map);

    for ti in tis {
        // 转化列信息
        let mut abs: Vec<Attribute> = Vec::new();
        for tc in ti.table_columns.iter() {
            let attribute_type = ATTRIBUTE_TYPE_MAP
                .iter()
                .filter(|(k, _)| *k == tc.data_type.as_str())
                .next()
                .map(|(_, v)| *v)
                .unwrap_or("String");
            let attribute = Attribute {
                pk: tc.column_key == "PRI".to_string(),
                column_name: tc.column_name.clone(),
                attribute_name: to_class_case(&tc.column_name.clone()),
                attribute_name_fl: to_camel_case(&tc.column_name.clone()),
                attribute_type: attribute_type.to_string(),
                comment: tc.column_comment.clone(),
            };
            abs.push(attribute);
        }

        let ci = ClassInfo {
            date_time: now.clone(),
            author: author.clone(),
            pkg_name: pkg_url.clone(),
            module_name: table_module_map.get(&ti.table_name).unwrap().to_string(),
            table_name: ti.table_name.to_string(),
            class_name: to_class_case(&ti.table_name.clone()),
            class_name_fl: to_camel_case(&ti.table_name.clone()),
            attributes: abs,
            comment: ti.table_comment,
        };
        cis.push(ci);
    }
    cis
}

fn to_class_case(str: &str) -> String {
    str.split('_')
        .map(|s| s.chars().nth(0).unwrap().to_uppercase().to_string() + &s[1..])
        .collect::<Vec<_>>()
        .join("")
}

fn to_camel_case(str: &str) -> String {
    let mut camel_str = to_class_case(str);
    camel_str[..1].make_ascii_lowercase();
    camel_str
}
