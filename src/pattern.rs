mod process;
mod school;
use self::school::School;
use rand::{thread_rng, Rng};
pub struct Pattern {
    //Pattern就是我们的模型, 结构体类型.
    pub school: Vec<School>,
    //school类型是Vector, 里边装学校.
}
pub trait Inspect {
    fn inspector(&self);
}
//声明一个特质, 用于论文和评委的检查.

impl Default for Pattern {
    fn default() -> Self {
        let school_num = thread_rng().gen_range(5..=8);
        let mut school = Vec::with_capacity(school_num);
        let _ = (0..school_num)
            .map(|_| {
                school.push(School::default());
            })
            .collect::<Vec<_>>();
        //随机产生5到8个学校, 用Vector装起来.

        Self { school }
        //返回模型, 后续processor函数会用到.
    }
}
