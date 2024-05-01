use clap::Parser;
use paper::Paper;
use rand::{thread_rng, Rng};
use reviewer::Reviewer;

pub mod paper;
pub mod reviewer;

pub struct School {
    //一个学校
    name: String,
    reviewer: Vec<Reviewer>, //vec里装了很多评委, 这些评委是同一所学校的
    paper: Vec<Paper>,       //vec里装了很多论文, 同理, 这些论文也是同一所学校的
}
impl Default for School {
    fn default() -> Self {
        let mut name = String::with_capacity(2);
        let _ = (0..2)
            .into_iter()
            .map(|_| name.push(thread_rng().gen_range(65..=90) as u8 as char))
            .collect::<Vec<_>>(); //随机产生由两个大写字母组成的学校名称

        let reviewer_num = thread_rng().gen_range(3..=7); //随机产生评委数量: 3到7
        let mut reviewer = Vec::with_capacity(reviewer_num);
        let _ = (0..reviewer_num)
            .map(|_| reviewer.push(Reviewer::default()))
            .collect::<Vec<_>>(); //生成评委
                                  //test
        let _ = reviewer
            .iter()
            .inspect(|item| println!("一位评委名字: {}", item.0))
            .collect::<Vec<_>>(); //测试
        println!("----评委论文分界线----"); //test
        let paper_num = thread_rng().gen_range(50..=80); //随机学校内论文数量: 50到80
        let mut paper = Vec::with_capacity(paper_num);
        let _ = (0..paper_num)
            .map(|_| paper.push(Paper::default()))
            .collect::<Vec<_>>(); //同理, 生成论文

        //test
        let _ = paper
            .iter()
            .inspect(|item| println!("一篇论文编号: {}", item.0))
            .collect::<Vec<_>>(); //测试
                                  //test
        println!(
            "此学校名字: {}, 评委数量: {}, 论文数量: {}",
            name, reviewer_num, paper_num
        ); //检查是否成功产生, 打印一下

        Self {
            name,
            reviewer,
            paper,
        } //组合成结构体返回
    }
}
