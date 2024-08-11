mod cmd;
mod file;
mod cmp;

use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;
use std::{io, process, thread};
use std::time::Duration;
use crate::file::{File};

#[derive(Debug,Deserialize,Clone)]
struct config{
path:String,
temp_path:String
}
fn main() {
    let handle = thread::spawn(|| {
         //子线程比对文件是否一致,不一致就重新启动
        let config= serde_json::from_str::<config>(&*fs::read_to_string("./src/config.json").unwrap()).unwrap();
         let mut f1=File::create_from(&config.path);
        loop{
             thread::sleep(Duration::from_secs(4));
             let f2=File::create_from(&config.path);
            if  f1.is_modify(&f2){
                //
                println!("文件已修改")
            }else{
                println!("文件未修改")
            }
            f1=f2;
        }

    });
// //触发函数，等待相应
//     let mut input=String::new();
//     loop{
//         input.clear();
//       match  io::stdin().read_line(&mut input){
//           Ok(_)=>{
//               let input = input.trim();
//               if input == "status" {
//                   //状态正常打印1
//                   println!("{}",1);
//               }else if input=="exit"{
//                   println!("安全退出");
//                   process::exit(0)
//               }
//           }
//           Err(err) => {
//               println!("程序发生问题{}",err);
//               process::exit(0)
//           }
//       }
//     }
    //读取本地config
     loop{}
}