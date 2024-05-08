mod school;
use self::school::School;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use school::reviewer::Reviewer;
use std::collections::{BTreeMap, HashMap};
pub struct Pattern(BTreeMap<String, Vec<Reviewer>>);
pub trait Inspect {
    fn inspector(&self);
}
//声明一个特质, 用于试卷和评委的检查.

impl Pattern {
    pub fn new(k: usize) -> Self {
        //学校个数: k

        let m = thread_rng().gen_range(2..k);
        //能参选评委总数: m

        println!("一共有{k}个学校, 有{m}位评委参选\n================");
        //打印检查一下.

        let mut school = Vec::with_capacity(k);
        let _ = (0..k)
            .map(|_| {
                school.push(School::default());
            })
            .collect::<Vec<_>>();
        //生成学校.

        let _ = &school.shuffle(&mut thread_rng());
        //洗牌学校顺序, 保证随机性和公平性.

        let _ = school
            .iter()
            .map(|item| item.inspector())
            .collect::<Vec<_>>();
        //打印检查一下学校的试卷和评委.

        let mut full_paper = Vec::new();
        //一个Vector, 准备装所有学校的试卷.
        let mut full_reviewer = Vec::new();
        //一个Vector, 准备装所有学校的评委(有的不能参选, 这里先装进去, 下边会处理).

        let _ = school
            .iter_mut()
            .map(|item| {
                full_paper.append(&mut item.paper);
                full_reviewer.append(&mut item.reviewer);
            })
            .collect::<Vec<_>>();

        //循环, 用于装试卷和论文.

        let mut sel_rev = Vec::with_capacity(m);
        //一个Vector, 装真正能参选的评委, 容量是m.
        loop {
            let _ = &full_reviewer.shuffle(&mut thread_rng());
            if &full_reviewer[0].0[..=1] != &full_reviewer[1].0[..=1] {
                sel_rev.push(full_reviewer[0].clone());
                sel_rev.push(full_reviewer[1].clone());
                break;
            }
        }
        //一个能跳出的死循环,
        //每次循环, 使用shuffle方法打乱full_reviewer这个Vector的顺序,看看前两个评委是不是来自同一个学校,
        //是, 就插入这俩评委到sel_rev这个Vetor, 循环结束;
        //不是, 就在再洗牌,
        //一直迭代, 直到选出两个不同学校的评委.

        let _ = (0..(m - 2))
            .map(|_| sel_rev.push(full_reviewer.pop().unwrap()))
            .collect::<Vec<_>>();
        //一个循环, 用于选剩下的评委,
        //上边已经选了两个评委了, 而且上边loop内已经洗牌, 这边直接用pop方法把full_reviewer这个方法的最后(m-2)评委插进去.

        let _ = sel_rev
            .iter()
            .map(|item| println!("{}", item.0))
            .collect::<Vec<_>>();
        //检查一下能参选的评委.
        let mut map = HashMap::new();
        //这里先新建一个表.
        let _ = sel_rev
            .iter()
            .map(|item| map.insert(&item.0[..=1], "None"))
            .collect::<Vec<_>>();
        let p = thread_rng().gen_range(1..map.len());
        //这里利用map类型的key值会覆盖, 不会重复的特性,
        //把评委前两个字母, 也就是学校名字, 插入到map里,
        //如果map长度是x, 那这些评委就来自x个学校,
        //p的范围, 也就是1到map的长度.
        //这样做能保证每份试卷不会出现要求a个评委改, 选出来却只有不到a个非本校评委.

        println!("========\n||每份试卷由{p}位评委审评||\n========");
        //打印检查一下

        let mut map = BTreeMap::new();
        //前边已经有map这个变量了对吧, 基于Rust语言的特性, 旧map会被覆盖掉, 此处无需担心重名问题.

        let _ = (0..full_paper.len())
            .map(|_| {
                let r1 = thread_rng().gen_range(0..full_paper.len());
                loop {
                    let mut vec = Vec::with_capacity(p);
                    let _ = (0..p)
                        .map(|_| loop {
                            let r2 = thread_rng().gen_range(0..sel_rev.len());

                            if &full_paper[r1].0[..=1] != &sel_rev[r2].0[..=1] {
                                vec.push(sel_rev[r2].clone());
                                break;
                            }
                        })
                        .collect::<Vec<_>>();
                    let mut vec1 = Vec::new();
                    let _ = (0..p)
                        .map(|i| {
                            ((i + 1)..p)
                                .map(|j| vec1.push((vec[i].clone(), vec[j].clone())))
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    if vec1.into_iter().all(|(a, b)| a.0 != b.0) {
                        map.insert(full_paper[r1].0.clone(), vec.clone());
                        vec.clear();
                        full_paper.remove(r1);
                        break;
                    }
                }
            })
            .collect::<Vec<_>>();
        //几个组合循环.
        //最外部是 (0..full_paper.len()), 每次迭代, 都重新计算full_paper这个Vector长度
        //在0到他的长度, 也就是索引范围内, 生成一个随机数r1表示索引,
        //接着是一个能跳出的死循环,
        //循环内, 先声明一个长度为P的Vector, 备用装评委,
        //然后又是一个循环, 循环p次, 对于每次循环, 都是一个能跳出的死循环, 随机出抽一个评委, 根论文前两个字母不一样就插到vec里循环结束, 一样就再循环, 直到为此试卷分配三个非本校评委, 但这些评委可能是同一个人, 下边会继续处理,
        //刚才说了, 我们没有remove这个评委, 能保证评委肯定不是本校的, 但有可能是别校的同一个人,
        //这里我们设计一个算法来避免这种情况,
        //新建一个vec1, 空的,
        //然后是一个循环, 往vec1里插入值, 简单来说就是: 将来这个vec1会充满vec的仅有一个顺序的组合(就是把vec里元素两两组合, 插到vec1),
        //然后对vec1迭代, 适用迭代器下边的all方法, 检查这里边的亚元素(评委)是不是都不一样,
        //都不一样就证明这次上边的小loop是有效的, 插入vec和paper到map里, 删除r1这个索引对应的元素(上边说了, 每次都重新计算full_paper的长度), 跳出大loop,
        //有一样的认为此次小loop无效, 重新开始大loop,
        //这样, 通过改变程序的鲁棒性, 我们保证了回避原则和公平性.
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
