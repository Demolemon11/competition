use clap::Parser;

#[derive(Parser)]
#[command(about = "😋😋By 马培文, 衡浩, 路艺明from HNU🤪🤪")]
pub struct Args {
    #[arg(help = "🏫学校的数量, [4,7], 别输错, 输错就删系统文件, 电脑坏了我们不负责!")]
    pub k: usize,
}
