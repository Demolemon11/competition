use once_cell::sync::Lazy;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::BTreeMap;
static RATIO: once_cell::sync::Lazy<f32> = Lazy::new(|| thread_rng().gen_range(0.7..0.8)); //定义一个静态的0..1的浮点
use super::Pattern;

impl Pattern {
    pub fn processor(&mut self) {
        let mut paper = Vec::new(); //新论文, 类型是vector, 可理解为数组
        let num = (self.school.len() as f32 * Lazy::force(&RATIO)) as usize; //num表示抽取的评委数量, 学校总数即是评委总数, 但是只有一部分评委能参评, 遂随机生成一个0..1的小数, 评委总数乘以这个小数, 得到实际参评的评委
        let mut reviewer = Vec::with_capacity(num); //新评委
        let _ = &self.school[..].shuffle(&mut thread_rng()); //使用shuffle方法随机打乱school这个vector, 达到洗牌学校的目的, 保证随机性

        let _ = self
            .school
            .iter_mut()
            .map(|item| {
                paper.append(&mut item.paper); //收集所有学校的论文, 使用append方法扩充新的paper这个vector
            })
            .collect::<Vec<_>>();
        let _ = (0..=num)
            .into_iter()
            .map(|_| {
                reviewer.push(
                    self.school[thread_rng().gen_range(0..self.school.len())] //随机在所有评委中抽取num位评委, 并放入新的reviewer这个vector里
                        .reviewer
                        .clone(),
                );
            })
            .collect::<Vec<_>>();
        let mut map = BTreeMap::new(); //新弄一个BTreeMap, 以便后续插入论文和评委
        let _ = (0..paper.len())
            .map(|_| loop {
                let r1 = thread_rng().gen_range(0..paper.len());
                let r2 = thread_rng().gen_range(0..reviewer.len()); //一个死循环,每次循环开始, 生成两个随机数(即论文和评委的索引下标), 目的是看看评委们将要批改的这个论文是不是TA自己学校的, 是的话就什么都不做, 此次循环重新开始; 不是的话, 就插入到BTreemap里,直到所有的评委都批改了不是非学校的论文, 且论文分配完毕, 循环退出
                match &paper[r1].0[..=2] == &reviewer[r2].0[..=2] {
                    true => {}
                    false => {
                        map.insert(paper[r1].0.clone(), reviewer[r2].0.clone());
                        paper.remove(r1);
                        break;
                    }
                }
            }) //随机地将文论和评委插入BTreeMap中, 一个论文对应一个评委, 一个评委对应多篇论文
            .collect::<Vec<_>>();
        println!(
            "最后的统计\n总论文数:{}, 总评委数量:{}, 有效评委数量:{}",
            map.keys().len(),
            self.school.len(),
            num
        ); //最后的一些简短的统计
        let _ = map
            .iter()
            .map(|(paper, reviewer)| println!("论文 {} 被评委 {} 批改", paper, reviewer))
            .collect::<Vec<_>>(); //最后的检测, 看看哪些评委批改了哪些论文, 看看是否真的公平, 看看评委的工作量是否相当
    }
}
