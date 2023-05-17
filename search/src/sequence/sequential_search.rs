pub fn sequential_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;

    while pos < nums.len() {
        if num == nums[pos] {
            return true;
        }
        pos += 1;
    }
    false
}

pub fn sequential_search_pos(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos = 0;

    while pos < nums.len() {
        if num == nums[pos] {
            return Some(pos);
        }
        pos += 1;
    }
    None
}

pub fn ordered_sqeuentail_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    while pos < nums.len() {
        if num == nums[pos] {
            return true;
        } else {
            if num < nums[pos] {
                return false;
            }
        }
        pos += 1;
    }
    true
}
