//! A library for calculating the least number of flips needed to make a binary string monotone increasing.
//! Also contains utility functions for allocating webassembly memory and returning a pointer to it, as well
//! as deallocating said memory. It also contains a function that can be called via WebAssembly, and thus Javascript.
use std::{cmp::min, ffi::CStr, ffi::CString, i32::MAX, mem, os::raw::c_void};

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

#[no_mangle]
pub extern "C" fn alloc() -> *mut c_void {
    let mut buf = Vec::with_capacity(1024);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut c_void, size: usize) {
    let _ = Vec::from_raw_parts(ptr, size, size);
}

#[no_mangle]
pub unsafe extern "C" fn make_me_monotone_crescendo(ptr: *mut u8) {
    let input = CStr::from_ptr(ptr as *const i8).to_str().unwrap();
    let flips = monototone_crescendo(input);
    let answer = format!("The minimum number of flips needed is: {}", flips);

    let c_str = CString::new(answer).unwrap();
    let c_str_bytes = c_str.as_bytes_with_nul();

    let return_bytes = std::slice::from_raw_parts_mut(ptr, 1024);
    return_bytes[..c_str_bytes.len()].copy_from_slice(c_str_bytes);
}

/// # This is the [official solution][0] to [LeetCode problem #926][1]
///
/// The solution was simply translated by me into rust from java.
///
/// ## Explanation
///
/// I find the explanation on LeetCode to this solution somewhat overcomplicated and hard to understand, so I will attempt
/// to explain it (hopefully) a bit simpler here.
///
/// The first thing this solution does is calculate the [prefix sum][2] of every item in the string and store it in an array. So for example, if we had the string `11010`, the prefix sum array
/// for that string would look like: `[0,1,2,2,3,3]`. This prefix sum array represents the the number of 1's that are present before the corresponding character index in the string.
///
/// This solution then works by looping through the given string and, for each character in the string, analyzing the string as two halves partioned
/// at that character. The number of 1's in the first half are added to the number of 0's in the second half to determine the total number of flips
/// needed to make the string monotone increasing for that location in the string.
///
/// We get the number of 1's in the first half by looking up the corresponding index in the prefix sum array.
/// We get the number of 0's in the second half by subtracting the number of 1's in the second half from the length
/// of the second half.
/// 
/// We then compare this number to the current minimum flips value and take the minimum of these two values again. The minimium
/// we have in the end is what we return.
/// 
/// Note: the prefix sum array could also be optimized to remove the redundant element at index 0. The way the solution on LeetCode
/// calculates the prefix sum array, the first element will always be zero. For example, if we had the string `11010`, the prefix sum array
/// could be optimized to `[1,2,2,3,3]`. Apparently the official solution calculates the prefix sum array with the redundant zero at the beginning
/// to [prevent some sort of possible overflow condition][3], out of principle more than anything else.
///
/// [0]: <https://leetcode.com/problems/flip-string-to-monotone-increasing/solution/>
/// [1]: <https://leetcode.com/problems/flip-string-to-monotone-increasing/>
/// [2]: <https://en.wikipedia.org/wiki/Prefix_sum>
/// [3]: <https://leetcode.com/problems/flip-string-to-monotone-increasing/solution/1047706>
pub fn monotone_crescendo_prefix_sums(s: &str) -> i32 {
    let str_size = s.chars().count() as i32;
    let mut prefix_sums = vec![0; str_size as usize + 1];
    let mut min_flips: i32 = MAX;

    for (i, char) in s.chars().enumerate() {
        prefix_sums[i + 1] = prefix_sums[i] + (if char == '1' { 1 } else { 0 });
    }

    let mut j = 0;

    while j <= str_size {
        let ones_before_j = prefix_sums[j as usize];
        let ones_after_j = prefix_sums[str_size as usize] - prefix_sums[j as usize];

        let length_of_str_after_j = str_size - j;
        let zeroes_after_j = length_of_str_after_j - ones_after_j;

        min_flips = min(min_flips, ones_before_j + zeroes_after_j);

        println!(
            "min_flips: {:?}, j: {:?}, ones_before_j: {:?}, zeroes_after_j: {:?}",
            min_flips, j, ones_before_j, zeroes_after_j
        );

        j += 1;
    }

    min_flips
}

pub fn monototone_crescendo(s: &str) -> i32 {
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
