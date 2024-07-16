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
    // 获取文件
    let file_paths = all_file_paths();
    println!("file_paths: {:?}", file_paths);
    // 模板渲染并生成新文件
    let reg = Handlebars::new();
    for td in tpl_data {
        for fp in &file_paths {
            let tpl = fs::read_to_string(fp).expect("获取文件模板文件内容失败");
            let file_name = PathBuf::from(fp)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            // 新文件名
            let mut new_file_name = td.class_name.clone();
            new_file_name.push_str(&file_name);
            // println!("new_file_name: {}", new_file_name);
            // 新文件路径
            let mut new_file_path = PathBuf::from("./output");
            new_file_path.push(new_file_name);
            println!("new_file_path: {}", new_file_path.display());
            // 创建新文件目录
            fs::create_dir_all(new_file_path.parent().unwrap()).expect("创建目录失败");
            let new_file = File::create(new_file_path).expect("创建文件失败");
            reg.render_template_to_write(&tpl, &td, new_file)
                .expect("渲染模板失败");
        }
    }
}

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
