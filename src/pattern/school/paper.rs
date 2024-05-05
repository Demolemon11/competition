use crate::pattern::Inspect;

use super::PushName;
use rand::{thread_rng, Rng};
pub struct Paper(pub String);

impl Default for Paper {
    fn default() -> Self {
        Self(
            vec!['a'; 5]
                .into_iter()
                .map(|_| thread_rng().gen_range(97..=122) as u8 as char)
                .collect(), //随机产生一篇论文编号: 5个小写字母
        )
    }
}
impl PushName for Paper {
    fn push_name(self, name: &str) -> Self {
        Self(format!("{}_{}", name, self.0))
    }
}
impl Inspect for Paper {
    fn inspector(&self) {
        println!("论文: {}", self.0)
    }
}
