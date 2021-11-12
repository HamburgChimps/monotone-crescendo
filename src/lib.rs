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

fn monotone_crescendo_prefix_sums(s: &str) -> i32 {
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
