use crate::Runner;

pub struct DivergingFunction {}

impl Runner for DivergingFunction {
    fn run(&self) {
        // 発散する関数はreturnしない関数のこと、-> !を使って表される
        // panic!/continueとかがそれに該当する
        // 発散する型は他のどの型にもキャストできるので以下の感じでmatch armで使える
        fn sum_odd_numbers(up_to: u32) -> u32 {
            let mut acc = 0;
            for i in 0..up_to {
                let addition: u32 = match i % 2 == 1 {
                    true => i,         // これは問題ない
                    false => continue, // これも発散するので問題ない
                };
                acc += addition;
            }
            acc
        }
        println!(
            "Sum of odd numbers up to 9 (excluding): {}",
            sum_odd_numbers(9)
        );
    }
}
