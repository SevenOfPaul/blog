use std::{fs, process};
use std::fs::ReadDir;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug,Serialize,Deserialize)]
pub enum File {
    DT(DirTree),
    FT(FileTree),
}

#[derive(Clone, Debug,Serialize,Deserialize)]
struct DirTree {
    name: String,
    mod_time: SystemTime,
    children: Vec<File>,
}

impl DirTree {
    fn new(name: String, mod_time: SystemTime, children: Vec<File>) -> DirTree {
        DirTree { name, mod_time, children }
    }
}

#[derive(Clone, Debug,Serialize,Deserialize)]
struct FileTree {
    name: String,
    mod_time: SystemTime,
    val: Vec<u8>,
}

impl FileTree {
    fn new(name: String, mod_time: SystemTime, val: Vec<u8>) -> FileTree {
        FileTree { name, mod_time, val }
    }
}

impl<'a> File {
    //读取文件路径创建FT
    pub fn create_from(path: &str) -> File {
        if let Ok(dir) = fs::metadata(path) {
            if dir.is_dir() {
                return File::DT(DirTree::new(String::from(path),dir.modified().unwrap(), File::add_tree(fs::read_dir(path).unwrap())));
            } else {
                println!("所提交的路径不是文件夹");
                process::exit(0)
            }
        } else {
            println!("未找到所标识的目录");
            process::exit(0)
        }
    }
    fn new(name: String, mod_time: SystemTime, children: Vec<File>) -> File {
        File::DT(DirTree { name, mod_time, children })
    }
    fn read_file(path: &str) -> File {
        let meta = fs::metadata(path).unwrap();
        if let Ok(file) = fs::read(path) {
            return File::FT(FileTree::new(String::from(path), meta.modified().unwrap(), file));
        } else {
            println!("文件读取失败");
            process::exit(1)
        }
    }
    pub fn is_dir(&self) -> bool {
        return match self {
            File::DT(_) => true,
            File::FT(_) => false
        };
    }
    pub fn is_file(&self) -> bool {
        return match self {
            File::DT(_) => false,
            File::FT(_) => true
        };
    }
    fn add_tree(dirs: ReadDir) -> Vec<File> {
        let mut ans = vec![];
        for dir in dirs {
            let file = dir.unwrap();
            let path_dir = file.path();
            println!("{:?}",path_dir);
            if path_dir.is_dir() {
                //文件夹
                    ans.push(Self::create_from(path_dir.to_str().unwrap()));
            } else {
                //文件
                ans.push(Self::read_file(path_dir.to_str().unwrap()));
            }
        }
        ans
    }

    pub fn is_modify(&self, new_file: &File) -> bool {
     match (self, new_file) {
            //皆为函数
            (File::DT(old_dt), File::DT(new_dt)) => {
                if old_dt.name == new_dt.name && old_dt.mod_time == new_dt.mod_time && old_dt.children.len() == new_dt.children.len() {
                  //对两个children使用all方法进行迭代，all相当于js中的every
                    old_dt.children.iter().zip(new_dt.children.iter()).any(|(o,n)| Self::is_modify(o,n))
                } else {
                    true
                }
            }
            (File::FT(old_ft), File::FT(new_ft)) => {
                if old_ft.name == new_ft.name && old_ft.mod_time == new_ft.mod_time && old_ft.val.len() == new_ft.val.len() {
                    let mut old_hasher = DefaultHasher::new();
                    let mut new_hasher = DefaultHasher::new();
                    old_ft.val.hash(&mut old_hasher);
                    new_ft.val.hash(&mut new_hasher);
                 old_hasher.finish() != new_hasher.finish()
                } else {
                    true
                }
            }
            _ => true
        }
    }
}


