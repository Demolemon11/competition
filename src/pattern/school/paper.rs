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
