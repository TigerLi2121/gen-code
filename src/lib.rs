pub mod db;
pub mod gen;
pub mod table;

#[cfg(test)]
mod tests {
    use std::{env, fs, path::PathBuf};

    fn read_file_contents(path: &str) -> String {
        fs::read_to_string(path).unwrap_or_else(|e| {
            println!("读取文件错误{:?}", e);
            String::new()
        })
    }

    fn all_file_paths() -> Vec<String> {
        let mut files = Vec::new();
        let current_dir = env::current_dir().expect("获取当前目录错误");
        let mut path = PathBuf::from(current_dir);
        path.push("templates");
        println!("模板路径：{:?}", path);
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

    fn file_extension(filename: &str) -> Option<&str> {
        // 找到文件名中最后一个'.'的位置
        filename.rfind('.')
            // 检查'.'是否是文件名的一部分，而不是目录的一部分
            .filter(|&pos| pos != filename.len() - 1)
            .and_then(|pos| filename.get(pos + 1..).map(str::trim))
    }

    #[test]
    fn get_file_extension() {
        let fe = file_extension("index.vue").expect("获取扩展名失败");
        println!("文件扩展名：{}", fe)
    }

    #[test]
    fn get_all_file_paths() {
        let files = all_file_paths();
        println!("全部文件路径：{:?}", files)
    }

    #[test]
    fn get_all_file_contents() {
        let files = all_file_paths();
        println!("全部文件路径：{:?}", files);
        for file in files {
            println!("文件内容：\n{}", read_file_contents(&file))
        }
    }
}
