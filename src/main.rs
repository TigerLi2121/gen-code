use dotenv;
use handlebars::Handlebars;
use serde_json::json;
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

use gen_code::db;
use gen_code::gen;
use gen_code::table;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    db::init_db_pool().await.expect("数据库连接失败");
    // 获取表数据
    let table_infos = table::table_infos().await.expect("获取表信息失败");
    println!("table_infos: {}", json!(table_infos));
    // 获取模板数据
    let tpl_data = gen::tpl_data(table_infos);
    println!("tpl_data: {}", json!(tpl_data));
    // 获取模板文件
    let tpl_file_paths = all_file_paths();
    println!("tpl_file_paths: {:?}", tpl_file_paths);
    // 默认路径
    let default_path = String::from("./output");
    // 获取java生成文件路径
    let java_path = dotenv::var("JAVA_PATH").unwrap_or(default_path.clone());
    println!("java_path: {}", java_path);
    // 获取vue生成文件路径
    let vue_path = dotenv::var("VUE_PATH").unwrap_or(default_path.clone());
    println!("vue_path: {}", vue_path);
    // 模板渲染并生成新文件
    let reg = Handlebars::new();
    for td in tpl_data {
        for fp in &tpl_file_paths {
            let tpl_content = fs::read_to_string(fp).expect("获取文件模板文件内容失败");
            let file_name = PathBuf::from(fp)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let file_ext = file_extension(&file_name).expect("获取模板文件扩展名失败");

            let mut new_file_name = String::new();
            let mut new_file_path = PathBuf::new();
            // java新文件名、文件路径
            if file_ext == "java" {
                new_file_name = td.class_name.clone();
                new_file_name.push_str(&file_name);
                new_file_path.push(java_path.clone());
                new_file_path.push(&td.module_name);
                if new_file_name.contains("Entity") {
                    new_file_path.push("entity")
                }
                if new_file_name.contains("Dao") {
                    new_file_path.push("dao")
                }
                if new_file_name.contains("Service") {
                    new_file_path.push("service")
                }
                if new_file_name.contains("Impl") {
                    new_file_path.push("impl")
                }
                if new_file_name.contains("Controller") {
                    new_file_path.push("controller")
                }
            }
            // vue 新文件名、文件路径
            else if file_ext == "vue" {
                new_file_name = td.table_name.clone();
                new_file_name.push_str(".");
                new_file_name.push_str(file_ext);
                new_file_path.push(vue_path.clone());
            }
            // println!("new_file_name: {} "new_file_path: {}", new_file_name, new_file_path);

            let mut new_file_url = PathBuf::from(new_file_path);
            new_file_url.push(new_file_name);
            println!("new_file_path: {}", new_file_url.display());
            // 创建新文件目录
            fs::create_dir_all(new_file_url.parent().unwrap()).expect("创建目录失败");
            let new_file = File::create(new_file_url).expect("创建文件失败");
            reg.render_template_to_write(&tpl_content, &td, new_file)
                .expect("渲染模板失败");
        }
    }
}

// 获取文件扩展名
fn file_extension(filename: &str) -> Option<&str> {
    // 找到文件名中最后一个'.'的位置
    filename
        .rfind('.')
        // 检查'.'是否是文件名的一部分，而不是目录的一部分
        .filter(|&pos| pos != filename.len() - 1)
        .and_then(|pos| filename.get(pos + 1..).map(str::trim))
}

// 获取全部模板文件路径
fn all_file_paths() -> Vec<String> {
    let mut files = Vec::new();
    let current_dir = env::current_dir().expect("获取当前目录错误");
    let mut path = PathBuf::from(current_dir);
    path.push("templates");
    println!("templates_path: {:?}", path);
    let dir = fs::read_dir(path).expect("读取模板目录错误");
    for entry in dir {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    files.push(path.display().to_string())
                }
            }
            Err(e) => {
                println!("获取文件路径错误{:?}", e)
            }
        }
    }
    files
}
