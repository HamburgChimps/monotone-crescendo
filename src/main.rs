use std::{cmp::min, i32::MAX};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_prefix_sums() {
        assert_eq!(monotone_crescendo_prefix_sums("11010"), 2);
    }

    #[test]
    fn test() {
        assert_eq!(monototone_crescendo("11010"), 2);
    }
}

fn monotone_crescendo_prefix_sums(s: &str) -> i32 {
    let str_size = s.chars().count() as i32;
    let mut prefix_sums = vec![0; str_size as usize + 1];
    let mut ans: i32 = MAX;

    for (i, char) in s.chars().enumerate() {
        prefix_sums[i + 1] = prefix_sums[i] + (if char == '1' { 1 } else { 0 });
    }

    let mut j = 0;

    while j <= str_size {
        ans = min(
            ans,
            prefix_sums[j as usize] + str_size
                - (j as i32)
                - (prefix_sums[str_size as usize] - prefix_sums[j as usize]),
        );

        j += 1;
    }

    println!("{:?}", ans);

    ans
}

fn monototone_crescendo(s: &str) -> i32 {
    let mut ones = 0;
    let mut flips = 0;

    for char in s.chars() {
        match char {
            '1' => ones += 1,
            _ => flips += 1,
        };

        flips = min(ones, flips);
    }

    println!("{:?}", flips);

    flips
}

fn main() {
    monototone_crescendo("11010");
    monotone_crescendo_prefix_sums("11010");
}
