//! A library for calculating the least number of flips needed to make a binary string monotone increasing.
//! Also contains utility functions for allocating WebAssembly linear memory and returning a pointer to it, as well
//! as deallocating said memory.

#[cfg(test)]
mod tests {
    use super::solution::*;
    use super::wasm_memory::call_solution_with_input;

    #[test]
    fn test_call_solution_with_input() {
        let input = String::from("Cumulative\011010\0");
        let mut buf = input.into_bytes();
        let ptr = buf.as_mut_ptr();

        unsafe {
            call_solution_with_input(ptr);
        }
    }

    #[test]
    fn test_with_prefix_sums() {
        assert_eq!(monotone_crescendo_prefix_sums("11010"), 2);
    }

    #[test]
    fn test_with_prefix_sums_without_redundant_zero() {
        assert_eq!(
            monotone_crescendo_prefix_sums_without_redundant_zero("11010"),
            2
        );
    }

    #[test]
    fn test_cumulative() {
        assert_eq!(monototone_crescendo_cumulative("11010"), 2);
    }
}

pub mod wasm_memory {
    //! # Utilities for interacting with WASM's linear memory

    use super::solution::*;
    use std::{ffi::CString, mem, os::raw::c_void};

    /// # Allocate memory in WebAssembly's linear memory and return its offset
    #[no_mangle]
    pub extern "C" fn alloc() -> *mut c_void {
        let mut buf = Vec::with_capacity(1024);
        let ptr = buf.as_mut_ptr();

        mem::forget(buf);

        ptr
    }

    /// # Deallocate memory in WebAssembly's linear memory at the offset located at `ptr`
    ///
    /// No explicit calls to [std::mem::drop] are needed as [std::vec::Vec::from_raw_parts] takes ownership
    /// of the memory pointed at by `ptr`.
    #[no_mangle]
    pub unsafe extern "C" fn dealloc(ptr: *mut c_void) {
        let _ = Vec::from_raw_parts(ptr, 1024, 1024);
    }

    /// # Call the specified [solution][super::solution] method with the given input
    ///
    /// `ptr` should be a pointer containing UTF-8 encoded characters separated by a null character. The first block of characters
    /// should be the name of solution to use, and the second block of characters should be the input to the solution itself.
    #[no_mangle]
    pub unsafe extern "C" fn call_solution_with_input(ptr: *mut u8) {
        let mut iter = ptr;

        let mut solution_name = String::new();
        let mut input = String::new();

        let mut null_count = 0;
        while null_count < 2 {
            let c = *iter as char;

            iter = iter.add(1);

            if c == '\0' {
                null_count += 1;
                continue;
            }

            match null_count {
                0 => solution_name.push(c),
                1 => input.push(c),
                _ => panic!("Unexpected null_count value"),
            }
        }

        let flips = match solution_name.as_str() {
            "Prefix Sums" => monotone_crescendo_prefix_sums(&input),
            "Cumulative" => monototone_crescendo_cumulative(&input),
            "Prefix Sums w/o Redundant Zero" => {
                monotone_crescendo_prefix_sums_without_redundant_zero(&input) as i32
            }
            _ => -1,
        };

        let answer = match flips {
            -1 => format!("Solution name \"{}\" does not exist", solution_name),
            _ => format!("The minimum number of flips needed is: {}", flips),
        };

        let c_str = CString::new(answer).unwrap();
        let c_str_bytes = c_str.as_bytes_with_nul();

        let return_bytes = std::slice::from_raw_parts_mut(ptr, 1024);
        return_bytes[..c_str_bytes.len()].copy_from_slice(c_str_bytes);
    }
}

pub mod solution {
    //! # Implementations of the monotone increasing problem

    use std::cmp::min;
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
    /// to [prevent some sort of possible overflow condition][3], out of principle more than anything else. See [monotone_crescendo_prefix_sums_without_redundant_zero] for an example.
    ///
    /// [0]: <https://leetcode.com/problems/flip-string-to-monotone-increasing/solution/>
    /// [1]: <https://leetcode.com/problems/flip-string-to-monotone-increasing/>
    /// [2]: <https://en.wikipedia.org/wiki/Prefix_sum>
    /// [3]: <https://leetcode.com/problems/flip-string-to-monotone-increasing/solution/1047706>
    pub fn monotone_crescendo_prefix_sums(s: &str) -> i32 {
        let str_size = s.chars().count() as i32;
        let mut prefix_sums = vec![0; str_size as usize + 1];
        let mut min_flips: i32 = i32::MAX;

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

    /// # Essentially the same solution as [monotone_crescendo_prefix_sums] except without the redundant leading zero in the prefix sum array
    ///
    /// This function also utilizes [u8]'s instead of [i32]'s.
    /// This could have also been done in [monotone_crescendo_prefix_sums] as our input will always lead to positive prefix sums and a positive
    /// return value which should all be less than or equal to 255. The only reason [monotone_crescendo_prefix_sums] uses [i32]'s is that I made an
    /// arbitrary choice to do so, and when removing the redundant zero I thought about it a bit more and realized [u8]'s would be fine.
    pub fn monotone_crescendo_prefix_sums_without_redundant_zero(s: &str) -> u8 {
        let str_size = s.chars().count();
        let mut prefix_sums: Vec<u8> = Vec::with_capacity(str_size);
        let mut min_flips: u8 = u8::MAX;

        println!("min_flips: {:?}", min_flips);

        for (i, char) in s.chars().enumerate() {
            let num = match char {
                '1' => 1,
                _ => 0,
            };
            if i == 0 {
                prefix_sums.push(num);
                continue;
            }

            prefix_sums.push(prefix_sums.get(i - 1).unwrap() + num);
        }

        let mut j = 0;

        while j < str_size {
            let ones_before_j = match j {
                0 => 0,
                _ => prefix_sums[j],
            };
            let ones_after_j = prefix_sums[str_size - 1] - prefix_sums[j];

            let length_of_str_after_j = (str_size - 1 - j) as u8;
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

    /// # This is an ingenious solution which was [contributed][0] by LeetCode user [tarunbisht][1] and translated to Rust by me
    ///
    /// ## Explanation
    ///
    /// This solution works by keeping a running total of all ones and all zeroes, where the count of zeroes is initlally contained in the variable `flips`.
    /// The variable `flips` is then set to the minimum value of the running total of zeroes or running total of ones.
    ///
    /// Overall this solution is not only simpler to understand, at least for me, but also requires only O(1) space as opposed to O(N) space.
    ///
    /// [0]: <https://leetcode.com/problems/flip-string-to-monotone-increasing/solution/725259>
    /// [1]: <https://leetcode.com/tarunbisht>
    pub fn monototone_crescendo_cumulative(s: &str) -> i32 {
        let mut ones = 0;
        let mut flips = 0;

        for char in s.chars() {
            match char {
                '1' => ones += 1,
                _ => flips += 1,
            };

            flips = min(ones, flips);
        }

        flips
    }
}
