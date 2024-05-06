mod paper;
mod reviewer;
use super::Inspect;
use paper::Paper;
use rand::{thread_rng, Rng};
use reviewer::Reviewer;
trait PushName {
    fn push_name(self, name: &str) -> Self;
}
pub struct School
//学校的类型: 有三个字段的结构体.
{
    pub name: String,
    //学校名字: 字符串
    pub paper: Vec<Paper>,
    //本校的论文: 用Vector装起来.
    pub reviewer: Reviewer,
    //本校的评委.
}
impl Default for School {
    fn default() -> Self {
        let mut name = String::with_capacity(2);
        let _ = (0..2)
            .into_iter()
            .map(|_| name.push(thread_rng().gen_range(65..=90) as u8 as char))
            //65到90的ascii码刚好是大写字母.
            .collect::<Vec<_>>();
        //产生学校名: 一个由两个随机的大写字母组成的字符串.

        let paper_num = thread_rng().gen_range(50..=80);
        //随机学校内论文数量: 50到80

        let mut paper = Vec::with_capacity(paper_num);
        let _ = (0..paper_num)
            .map(|_| paper.push(Paper::default().push_name(&name)))
            .collect::<Vec<_>>();
        //一个循环, 用于生成本校论文, paper_num的值是多少, 就循环多少次.

        let reviewer = Reviewer::default().push_name(&name);
        //为此学校生成评委.

        Self {
            name,
            reviewer,
            paper,
        }
        //组合成结构体返回, process.rs会用到
    }
}
impl Inspect for School {
    fn inspector(&self) {
        let _ = self
            .paper
            .iter()
            .inspect(|item| item.inspector())
            .collect::<Vec<_>>();
        //检查论文(打印出来)

        println!(
            "此学校校名:{}, 论文数量:{}, 评委:{}\n------------学校分界线------------",
            self.name,
            self.paper.len(),
            self.reviewer.0
        )

        //总结此学校
    }
}
