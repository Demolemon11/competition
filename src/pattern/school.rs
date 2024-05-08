pub mod paper;
pub mod reviewer;
use super::Inspect;
use paper::Paper;
use rand::{thread_rng, Rng};
use reviewer::Reviewer;

trait PushName //一个特质, 目的是在试卷的评委的前面加上学校名字.
{
    fn push_name(self, name: &str) -> Self;
}
pub struct School
//学校的类型: 有三个字段的结构体.
{
    pub name: String,
    //学校名字: 字符串
    pub paper: Vec<Paper>,
    //本校的试卷: Paper类型, 用Vector装起来.
    pub reviewer: Vec<Reviewer>,
    //本校的预选评委: Reviewer类型, 用Vector装起来.
    //但能参选多少, 全靠随机.
}
impl Default for School {
    fn default() -> Self {
        let mut name = String::with_capacity(2);
        //学校名: 随机两个大写字母

        let _ = (0..2)
            .into_iter()
            .map(|_| name.push(thread_rng().gen_range(65..=90) as u8 as char))
            //65到90的ascii码刚好是大写字母.
            .collect::<Vec<_>>();
        //map默认是惰性的, 这里收集一下, 让它执行.

        let paper_num = thread_rng().gen_range(45..=70);
        //本校试卷数: 随机50到80.

        let mut paper = Vec::with_capacity(paper_num);
        //一个空的Vector, 容量是本校论文数.
        let _ = (0..paper_num)
            .map(|_| paper.push(Paper::default().push_name(&name)))
            .collect::<Vec<_>>();
        //循环, 用于生成本校试卷, 循环paper_num次.

        let reviewer_num = (paper_num as f32 * thread_rng().gen_range(0.1..0.12)) as usize;
        //本校评委数: 基于本校试卷数, 试卷数越多, 预选评委就越多.
        let mut reviewer = Vec::with_capacity(reviewer_num);
        let _ = (0..reviewer_num)
            .map(|_| reviewer.push(Reviewer::default().push_name(&name)))
            .collect::<Vec<_>>();
        //循环, 用于生成本校评委, 循环reviewer_num次.

        Self {
            name,
            reviewer,
            paper,
        }
        //组合成结构体返回
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
        let _ = self
            .reviewer
            .iter()
            .inspect(|item| item.inspector())
            .collect::<Vec<_>>();

        println!(
            "此学校名称:{}, 论文数量:{}, 评委数量:{}\n--------学校分界线--------",
            self.name,
            self.paper.len(),
            self.reviewer.len()
        );

        //总结此学校
    }
}
