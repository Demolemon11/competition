use super::PushName;
use crate::pattern::Inspect;
use rand::{thread_rng, Rng};

pub struct Paper(pub String);
//论文的类型是结构体, 里边是一个字符串, 用于表示论文编号.

impl Default for Paper {
    fn default() -> Self {
        Self(
            vec!['a'; 5]
                //一个5个长度的vector.
                .into_iter()
                .map(|_| thread_rng().gen_range(97..=122) as u8 as char)
                //97到122的ascii码是小写字母.
                .collect(),
            //随机产生一篇论文编号: 5个小写字母
        )
    }
}
impl PushName for Paper {
    //在论文的5个小写字母前边加上学校, 后续会检验论文和评委前两个字母是否相同, 以此达到回避原则
    fn push_name(self, name: &str) -> Self {
        Self(format!("{}_{}", name, self.0))
    }
}
impl Inspect for Paper {
    //为论文实现Inspect特质, 方便调试和检查.
    fn inspector(&self) {
        println!("论文: {}", self.0)
    }
}
