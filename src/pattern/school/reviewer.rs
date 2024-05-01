use rand::{thread_rng, Rng};
pub struct Reviewer(pub String);

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
