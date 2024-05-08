use clap::Parser;
use competition::arguments::Args;
use competition::pattern::{Inspect, Pattern};
use std::process;

fn main() {
    let k = Args::parse().k;
    //从环境变量获取用户输入
    if k < 4 || k > 7 {
        println!("学校数太多或太少!");
        //范围[4,7], 输错退出
        process::exit(0)
    }
    Pattern::new(k).inspector()
    //检查&处理模型
}
