use super::Pattern;
use once_cell::sync::Lazy;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::BTreeMap;
static RATIO: once_cell::sync::Lazy<f32> = Lazy::new(|| thread_rng().gen_range(0.7..0.8));
//定义一个静态的0.7到0.8的浮点

impl Pattern {
    pub fn processor(&mut self) {
        let mut paper = Vec::new();
        //一个Vector, 准备装所有学校的论文.

        let num = (self.school.len() as f32 * Lazy::force(&RATIO)) as usize;
        //num表示能参评的评委数量, 学校总数即是预选评委总数, 但是只有一部分评委能参评,这是能参评的(有效评委数).

        let mut reviewer = Vec::with_capacity(num);
        //一个Vector, 准备装能参评的评委(有效评委).

        let _ = &self.school[..].shuffle(&mut thread_rng());
        //用shuffle方法随机打乱school这个Vector里元素的索引顺序, 达到洗牌学校的目的, 保证随机性.

        let _ = self
            .school
            .iter_mut()
            .map(|item| {
                paper.append(&mut item.paper);
            })
            .collect::<Vec<_>>();
        //收集所有学校的论文,使用append方法对上述Vector扩展.

        let mut count = 0;
        //一个可变的计数器, 用于抽取前num个学校
        let _ = (0..num)
            .into_iter()
            .map(|_| {
                reviewer.push(self.school[count].reviewer.clone());
                count += 1;
            })
            //前边已经洗牌过学校了, 这里直接取前num个学校的评委, 不会破坏随机性
            .collect::<Vec<_>>();
        //每次循环都随机地插入一个学校的评委到上述的Vector里, 一共循环num次, 也就是选出来了num个评委(有效评委).

        let mut map = BTreeMap::new();
        //一个新的BTreeMap, 以便后续插入论文和评委

        let _ = (0..paper.len())
            .map(|_| loop {
                let r1 = thread_rng().gen_range(0..paper.len());
                let r2 = thread_rng().gen_range(0..reviewer.len());
                match &paper[r1].0[..=1] == &reviewer[r2].0[..=1] {
                    true => {}
                    false => {
                        map.insert(paper[r1].0.clone(), reviewer[r2].0.clone());
                        paper.remove(r1);
                        break;
                    }
                }
            })
            .collect::<Vec<_>>();
        //嵌套循环, 外部循环--有多少篇论文, 就执行多少次; 内部循环--一个能跳出的死循环(loop),
        //对于每次外部循环, 内部循环会先随机生成两个随机数(r1, r2), r1范围是0到论文的数量, 用于随机抽取--索引; r2范围是0到评委数量, 同理也用于随机抽取--索引,
        //判断前两个字母--学校名字, 如果论文前两个字母和评委前两个字母完全一样, 就本校评委批改了本校论文,
        //显然违反了回避原则, 所以如果相等的话, 就什么都不做, 此次外部循环重新开始, 内部循环是死循环, 无需担心次数问题;
        //不相等, 就认为没违反回避原则, 就插入此论文和此评委到BTreeMap, 并用remove方法销毁此论文,此次内部循环结束
        //依次迭代......, 直到所有论文和评委被插入BTreeMap里.

        println!(
            "最后的统计\n总论文数:{}, 总评委数量:{}, 有效评委数量:{}",
            map.keys().len(),
            self.school.len(),
            num
        );
        //简短的统计

        let _ = map
            .iter()
            .map(|(paper, reviewer)| println!("论文 {} 被评委 {} 批改", paper, reviewer))
            .collect::<Vec<_>>();
        //最后的检测, 看看哪些评委批改了哪些论文, 看看是否真的公平, 看看评委的工作量是否相当
    }
}
