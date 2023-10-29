use crate::Runner;

pub struct Hof {}

impl Runner for Hof {
    fn run(&self) {
        // 奇数を二乗した値の中で1000以下のものの合計を求める
        let upper = 1000;
        println!("Find the sum of all the squared odd numbers under {upper}");

        // 宣言型で書くと
        let mut acc = 0;
        for n in 0.. {
            let n_squared = n * n;
            if n_squared >= upper {
                break;
            } else if n_squared % 2 != 0 {
                acc += n_squared;
            }
        }
        println!("imperative style: {acc}");

        // 関数型で書くと
        let sum_of_squared_odd_numbers: u32 = (0..)
            .map(|n| n * n)
            .take_while(|&ns| ns < upper)
            .filter(|&ns| ns % 2 != 0)
            .sum();
        println!("functional style: {sum_of_squared_odd_numbers}");
    }
}
