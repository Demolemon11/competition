use rand::{thread_rng, Rng};
pub struct Reviewer(pub String);
use crate::pattern::Inspect;

use super::PushName;
impl Default for Reviewer {
    fn default() -> Self {
        Self(
            vec!['A'; 3]
                .into_iter()
                .map(|_| thread_rng().gen_range(65..=90) as u8 as char)
                .collect(), //随机产生一个评委名字--3个大写字母
        )
    }
}
impl PushName for Reviewer {
    fn push_name(self, name: &str) -> Self {
        Self(format!("{}_{}", name, self.0))
    }
}
impl Inspect for Reviewer {
    fn inspector(&self) {
        println!("评委: {}", self.0)
    }
}
