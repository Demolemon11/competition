use competition::pattern::Inspect;
use competition::pattern::Pattern;

fn main() {
    //每个正常的学校都会有很多位评委, 这是预选评委(不是每个人都能参评).
    //要求评委数量小于学校数量, 我们就默认每个学校最多有1个评委, 最少有0个评委, 这样最公平.
    //就是从这些预选评委中筛掉一些人, 用一个随机浮点筛掉一些评委(总评委数 * 这个随机浮点) 强制转化成整数, 作为有效评委数.
    //用rand库洗牌所有学校, 洗牌所有论文, 洗牌所有评委.
    //最后将洗牌过的论文和评委插入BTreeMap里, key是论文编号(每篇论文独一无二), val是评委名字(也是独一无二).
    //最后打印map里的所有key和val, 观察每位评委批改的论文是否是否足够随机, 每个评委工作量是否相当.
    let mut pattern = Pattern::default();
    //生成模型

    let _ = pattern
        .school
        .iter()
        .map(|item| item.inspector())
        //对于每个学校, 调用inspector函数, 检查一下.
        .collect::<Vec<_>>();
    pattern.processor()
    //调用processor函数, 处理模型.
}
