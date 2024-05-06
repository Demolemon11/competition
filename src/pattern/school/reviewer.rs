use super::PushName;
use crate::pattern::Inspect;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Reviewer(pub String);
//评委的类型是结构体, 里边是一个字符串, 用于表示评委名字.

impl Default for Reviewer {
    fn default() -> Self {
        Self(
            vec!['A'; 3]
                .into_iter()
                .map(|_| thread_rng().gen_range(65..=90) as u8 as char)
                .collect(),
            //随机产生一个评委名字: 3个大写字母
        )
    }
}
impl PushName for Reviewer {
    fn push_name(self, name: &str) -> Self {
        Self(format!("{}_{}", name, self.0))
        //同理, 见paper.rs的push_name函数
    }
}
impl Inspect for Reviewer {
    //为评委实现检查特质
    fn inspector(&self) {
        println!("评委: {}", self.0)
    }
}
