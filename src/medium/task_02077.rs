// 2007. Find Original Array From Doubled Array
// https://leetcode.com/problems/find-original-array-from-doubled-array/

use crate::Solution;

impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        if changed.len() % 2 == 1 {
            return Vec::new();
        }

        let half_size = changed.len() / 2;
        let mut hm = std::collections::HashMap::with_capacity(changed.len());
        let mut res = Vec::with_capacity(half_size);

        changed.sort_unstable();
        changed.iter().for_each(|&x| {
            let e = hm.entry(x).or_insert(0);
            match *e == 0 {
                true => {
                    hm.entry(2 * x).and_modify(|n| *n += 1).or_insert(1);
                    res.push(x);
                }
                false => *e -= 1,
            };
        });

        match res.len() == half_size {
            true => res,
            false => vec![],
        }
    }

    // https://leetcode.com/problems/find-original-array-from-doubled-array/discuss/2579841/rust-bucket-sort-with-comments
    pub fn find_original_array_bucket_sort(changed: Vec<i32>) -> Vec<i32> {
        let n = changed.len();
        if n % 2 != 0 {
            return vec![];
        }
        let mut hist = [0; 100001];
        let (mut min, mut max) = (usize::MAX, 0);
        for i in changed.into_iter().map(|i| i as usize) {
            hist[i] += 1;
            min = min.min(i);
            max = max.max(i);
        }
        let mut rez = Vec::<i32>::with_capacity(n / 2);
        for i in (1.max(min)..=max).rev() {
            if hist[i] != 0 {
                if i % 2 == 0 && hist[i / 2] >= hist[i] {
                    rez.extend(std::iter::repeat((i / 2) as i32).take(hist[i]));
                    hist[i / 2] -= hist[i];
                } else {
                    return vec![];
                }
            }
        }
        rez.extend(std::iter::repeat(0).take(hist[0] / 2));
        rez
    }
}
