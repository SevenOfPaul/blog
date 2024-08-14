mod cmd;
mod file;
mod cmp;
use std::{fs, thread};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::cmp::compress;
use crate::file::{File};

#[derive(Debug, Deserialize, Clone)]
struct config {
    path: String,
    temp_path: String,
}
 fn main() {
     let mut path=PathBuf::from(std::env::current_dir().unwrap());
     path.push("/config.json");
     let config = serde_json::from_str::<config>(&*fs::read_to_string(path).unwrap()).unwrap();
     let f1 =Arc::new( Mutex::new(File::create_from(&config.path)));
     let  mut f1Share=Arc::clone(&f1);
    thread::spawn(move|| {
        let mut start = Instant::now();
        loop {
            let mut f1S1=f1Share.lock().unwrap();
            if Instant::now() - start > Duration::from_secs(3) {
                let f2 = File::create_from(&config.path);
                if f1S1.is_modify(&f2) {
                    println!("文件已修改");
                } else {
                    println!("文件未修改");
                }
                *f1S1 =f2;
            }
            start = Instant::now();
           drop(f1S1);
        }
    });
    //第二个定时任务
     let  mut f2Share=Arc::clone(&f1);
    thread::spawn(move|| {
        let mut start = Instant::now();
        let config = serde_json::from_str::<config>(&*fs::read_to_string("./src/config.json").unwrap()).unwrap();
        loop {
            let mut f2S2=f2Share.lock().unwrap();
           if Instant::now() - start > Duration::from_secs(60 * 60 * 24){
               fs::write(&config.temp_path, compress(serde_json::to_string(&*f2S2).unwrap()).unwrap());
           }
            start=Instant::now();
            drop(f2S2)
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
    loop {}
}