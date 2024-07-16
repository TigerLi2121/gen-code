use dotenv;
use handlebars::Handlebars;
use serde_json::json;
use std::{
    fs::{self, File},
    path::Path,
};

use gen_code::db;
use gen_code::table;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    db::init_db_pool().await.expect("数据库连接失败");

    let tables = table::table_infos().await.unwrap();
    println!("全部表：{:?}", tables);

    let reg = Handlebars::new();
    let tpl = fs::read_to_string("./templates/Controller.java").expect("获取文件模板文件内容失败");
    println!("tpl:\n{}", tpl);

    let path = Path::new("./output/Controller.java");
    fs::create_dir_all(path.parent().unwrap()).expect("创建目录失败");
    let file = File::create(path).expect("创建文件失败");

    reg.render_template_to_write(&tpl, &json!({"name": "world"}), file)
        .expect("渲染模板失败");
}
