use rand::{thread_rng, Rng};
use std::collections::BTreeMap;

use super::Pattern;

impl Pattern {
    pub fn processor(&mut self) {
        let mut paper = Vec::new();

        let mut reviewer = Vec::new();
        let _ = self
            .school
            .iter_mut()
            .map(|item| {
                paper.append(&mut item.paper);
                reviewer.append(&mut item.reviewer)
            })
            .collect::<Vec<_>>();

        let mut map = BTreeMap::new();
        let _ = (0..paper.len())
            .map(|_| loop {
                let r1 = thread_rng().gen_range(0..paper.len());
                let r2 = thread_rng().gen_range(0..reviewer.len());
                match &paper[r1].0[..=2] == &reviewer[r2].0[..=2] {
                    true => {}
                    false => {
                        map.insert(paper[r1].0.clone(), reviewer[r2].0.clone());
                        paper.remove(r1);
                        break;
                    }
                }
            })
            .collect::<Vec<_>>();
        println!("最后的统计\n总论文数: {}\n", map.keys().len());
        let _ = map
            .iter()
            .map(|(paper, reviewer)| println!("论文 {} 被评委 {} 批改", paper, reviewer))
            .collect::<Vec<_>>();
    }
}
