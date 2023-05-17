// binary_search.rs
pub fn binary_search1(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;
    // 注意 是 <= 不是 <
    while low <= high && !found {
        let mid: usize = (low + high) >> 1;
        // 若 low + high 可 能 溢 出 ， 可 转 换 为 减 法
        //let mid: usize = low + ((high - low) >> 1);
        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1; // num < 中间值 ， 省 去 后 半 部 数 据
        } else {
            low = mid + 1;
            // num >= 中间值 ， 省 去 前 半 部 数 据
        }
    }

    found
}

pub fn binary_search2(nums: &[i32], num: i32) -> bool {
    // 基 本 情 况 1: 项 不 存 在
    if 0 == nums.len() {
        return false;
    }
    let mid: usize = nums.len() >> 1;
    // 基 本 情 况 2: 项存在
    if num == nums[mid] {
        return true;
    } else if num < nums[mid] {
        // 减 小 问 题 规 模
        return binary_search2(&nums[..mid], num);
    } else {
        return binary_search2(&nums[mid + 1..], num);
    }
}
