use super::Inspect;
use once_cell::sync::Lazy;
use paper::Paper;
use rand::{thread_rng, Rng};
use reviewer::Reviewer;
pub mod paper;
pub mod reviewer;
trait PushName {
    fn push_name(self, name: &str) -> Self;
}
static RATIO: once_cell::sync::Lazy<f32> = Lazy::new(|| thread_rng().gen_range(0.1..0.16));
pub struct School {
    //一个学校
    pub name: String,
    pub paper: Vec<Paper>, //vec里装了很多论文, 同理, 这些论文也是同一所学校的
    pub reviewer: Vec<Reviewer>, //vec里装了很多评委, 这些评委是同一所学校的
}
impl Default for School {
    fn default() -> Self {
        let mut name = String::with_capacity(2);
        let _ = (0..2)
            .into_iter()
            .map(|_| name.push(thread_rng().gen_range(65..=90) as u8 as char))
            .collect::<Vec<_>>(); //随机产生由两个大写字母组成的学校名称

        let paper_num = thread_rng().gen_range(50..=80); //随机学校内论文数量: 50到80
        let mut paper = Vec::with_capacity(paper_num);
        let _ = (0..paper_num)
            .map(|_| paper.push(Paper::default().push_name(&name)))
            .collect::<Vec<_>>(); //同理, 生成论文

        let reviewer = vec![Reviewer("None".to_string())];
        Self {
            name,
            reviewer,
            paper,
        } //组合成结构体返回
    }
}
impl School {
    pub fn fix(&mut self) {
        let mut reviewer = Vec::new();
        let _ = (0..(self.paper.len() as f32 * Lazy::force(&RATIO)) as u32)
            .map(|_| reviewer.push(Reviewer::default().push_name(&self.name)))
            .collect::<Vec<_>>();
        self.reviewer = reviewer
    }
}
impl Inspect for School {
    fn inspector(&self) {
        let _ = self
            .paper
            .iter()
            .inspect(|item| item.inspector())
            .collect::<Vec<_>>();

        let _ = self
            .reviewer
            .iter()
            .inspect(|item| item.inspector())
            .collect::<Vec<_>>();
        println!(
            "此学校校名:{}, 论文数量:{}, 评委数量:{}\n------------学校分界线------------",
            self.name,
            self.paper.len(),
            self.reviewer.len()
        )
    }
}
