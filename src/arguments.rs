use clap::Parser;

#[derive(Parser)]
#[command(about = "😋😋By 马培文, 衡浩, 路艺明from HNU🤪🤪")]
pub struct Args {
    #[arg(help = "🏫学校的数量")]
    pub k: usize,
}
