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
pub extern "C" fn alloc(len: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(len);
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

#[no_mangle]
pub unsafe extern "C" fn make_me_monotone_crescendo_prefix_sums(ptr: *mut u8) {
    let input = CStr::from_ptr(ptr as *const i8).to_str().unwrap();
    let flips = monotone_crescendo_prefix_sums(input);
    let answer = format!("The minimum number of flips needed is: {}", flips);

    let c_str = CString::new(answer).unwrap();
    let c_str_bytes = c_str.as_bytes_with_nul();

    let return_bytes = std::slice::from_raw_parts_mut(ptr, 1024);
    return_bytes[..c_str_bytes.len()].copy_from_slice(c_str_bytes);
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
        let thing_i_dont_know = prefix_sums[j as usize] + str_size;
        let thing_i_dont_know_two = prefix_sums[str_size as usize] - prefix_sums[j as usize];
        ans = min(
            ans,
            thing_i_dont_know
                - (j as i32)
                - (prefix_sums[str_size as usize] - prefix_sums[j as usize]),
        );

        println!(
            "ans: {:?}, thing_i_dont_know: {:?}, thing_i_dont_know_two: {:?}",
            ans, thing_i_dont_know, thing_i_dont_know_two
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
