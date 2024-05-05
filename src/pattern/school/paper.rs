use crate::pattern::Inspect;

use super::PushName;
use rand::{thread_rng, Rng};
pub struct Paper(pub String);

impl Default for Paper {
    fn default() -> Self {
        Self(
            vec!['a'; 5] //新弄一个5个长度的vector, 97到122的ascii码刚好是英文的小写字母
                .into_iter()
                .map(|_| thread_rng().gen_range(97..=122) as u8 as char)
                .collect(), //随机产生一篇论文编号: 5个小写字母
        )
    }
}
impl PushName for Paper {
    fn push_name(self, name: &str) -> Self {
        Self(format!("{}_{}", name, self.0))
        //在论文的5个小写字母前边加上学校, 后续会检验论文和评委前两个字母是否相同, 以此达到规避原则
    }
}
impl Inspect for Paper {
    fn inspector(&self) {
        println!("论文: {}", self.0) //为论文实现Inspect特质, 调试过程中检查会很方便
    }
}
