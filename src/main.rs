mod cmd;
mod file;
mod cmp;
use std::fs;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use serde_json;
use tokio;
use tokio::time::{interval, Instant};
use crate::file::{File};

#[derive(Debug,Deserialize,Clone)]
struct config{
path:String,
temp_path:String
}
#[tokio::main]
async fn main() {
   tokio::spawn(async{
        let mut interval = interval(Duration::from_secs(1800));
        let config= serde_json::from_str::<config>(&*fs::read_to_string("./src/config.json").unwrap()).unwrap();
         let mut f1=File::create_from(&config.path);
        loop{
            interval.tick().await;
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
    //第二个定时任务
    tokio::spawn(async{
        let mut interval = interval(Duration::from_secs(60*60*24));
        let config= serde_json::from_str::<config>(&*fs::read_to_string("./src/config.json").unwrap()).unwrap();
        let mut f1=File::create_from(&config.path);
        loop{
            interval.tick().await;
            fs::write("./src/temp.f", serde_json::to_string(&f1).unwrap());
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