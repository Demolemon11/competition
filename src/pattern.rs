use rand::{thread_rng, Rng};

use self::school::School;
pub mod school;
pub struct Pattern {
    school: Vec<School>, //vec里装了很多学校
}
impl Default for Pattern {
    fn default() -> Self {
        let school_num = thread_rng().gen_range(3..=5);
        let mut school = Vec::with_capacity(school_num); //随机产生3-5个学校,用动态数组装起来
        let _ = (0..school_num)
            .map(|_| {
                school.push(School::default());
                println!(
                    "--------------------------------学校分界线---------------------------------"
                )
            })
            .collect::<Vec<_>>(); //产生学校,school.rs模块里为它实现了default特质
        Self { school }
    }
}
