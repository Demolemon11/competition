mod school;
use self::school::School;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use school::reviewer::Reviewer;
use std::collections::BTreeMap;
pub struct Pattern(BTreeMap<String, Vec<Reviewer>>);
pub trait Inspect {
    fn inspector(&self);
}
//声明一个特质, 用于试卷和评委的检查.

impl Pattern {
    pub fn new(k: usize) -> Self {
        //学校个数: k

        let mm = thread_rng().gen_range(2..k);
        //能参选评委总数: M

        println!("\n一共有{k}个学校, 有{mm}位评委参选\n");
        //打印检查一下.

        let mut school = Vec::with_capacity(k);
        let _ = (0..k)
            .map(|_| {
                school.push(School::default());
            })
            .collect::<Vec<_>>();
        //生成学校.

        let _ = school
            .iter()
            .map(|item| item.inspector())
            .collect::<Vec<_>>();
        //打印检查一下学校的试卷和评委.

        let mut papers = Vec::new();
        //一个Vector, 准备装所有学校的试卷.

        let mut reviewers = Vec::with_capacity(k);
        //一个Vector, 准备装所有评委.

        let _ = school
            .iter_mut()
            .map(|item| {
                papers.append(&mut item.paper);
                reviewers.push(&mut item.reviewer);
            })
            .collect::<Vec<_>>();
        let _ = &reviewers.shuffle(&mut thread_rng());
        //循环, 填装试卷和评委.

        let _ = (0..(k - mm)).map(|_| reviewers.pop()).collect::<Vec<_>>();

        let p = thread_rng().gen_range(1..reviewers.len());
        println!("================\n||每份试卷由{p}位评委审评||\n================");
        //打印检查一下

        let mut map = BTreeMap::new();

        let _ = (0..papers.len())
            .map(|_| {
                let r1 = thread_rng().gen_range(0..papers.len());
                let mut vec = Vec::with_capacity(p);
                while vec.len() != p {
                    let mut count = 0;
                    let _ = &reviewers.shuffle(&mut thread_rng());
                    loop {
                        if &papers[r1].0[..=1] != &reviewers[count].0[..=1] {
                            vec.push(reviewers[count].clone());
                            break;
                        }
                        count += 1;
                    }
                }
                map.insert(papers[r1].0.clone(), vec.clone());
                papers.remove(r1);
            })
            .collect::<Vec<_>>();
        //嵌套循环, 3层
        //最外边: 有多少份试卷, 就执行多少次,把得到的我们的目标类型插入map, 删除r1对应的元素,
        //中间: vec里元素不到p个就一直循环, 有个count计数器, 目的是保证将来每个vec元素数量都一样,
        //最里边: 保证每个vec里元素(评委)都不一样, 避免一份试卷被同一个评委多次批改,
        //这样, 就保证了每一分试卷都能随机地被p个不同的外校的评委批改.

        Self(map)
        //返回, 用于检查
    }
}
impl Inspect for Pattern {
    fn inspector(&self) {
        let _ = self
            .0
            .iter()
            .map(|(paper, reviewer)| {
                reviewer
                    .iter()
                    .map(|item| println!("试卷 {} 被评委 {} 批改", paper, item.0))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        //检查模型
    }
}
