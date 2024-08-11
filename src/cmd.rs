use std::process::Command;
pub fn excu(path:&str,target:&str,cmds:Vec<&str>){
    Command::new(target).args(cmds) // 替换为参数
        .current_dir(path) // 设置工作目录
        .output();
}