// 476. Number Complement
// https://leetcode.com/problems/number-complement/

use crate::Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        !num & ((num as u32).next_power_of_two() - 1).max(1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::find_complement(5));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::find_complement(0));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::find_complement(1));
    }

    #[test]
    fn test_4() {
        assert_eq!(10760938, Solution::find_complement(123456789));
    }
}
