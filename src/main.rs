use competition::pattern::Pattern;
//思路: 生成模型->进行处理->得到结果
//具体实现见注释
//什么编程语言都能做, python会非常简单, 但这里我们选择比较有难度的Rust
use competition::pattern::Inspect; //定义检查的特质, 特质(trait)是一组具有相同行为的函数

fn main() {
    let mut pattern = Pattern::default(); //生成默认模型
    let _ = pattern
        .school
        .iter() //Iterator是迭代器, 类似for循环
        .map(|item| item.inspector()) //调用inspector函数, 检查每个学校
        .collect::<Vec<_>>();
    pattern.processor() //调用processor函数, 处理模型
}
