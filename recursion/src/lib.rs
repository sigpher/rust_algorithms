pub fn nums_sum(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}

pub fn nums_sum2(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let first = nums[0];
        first + nums_sum2(&nums[1..])
    }
}

pub fn nums_sum3(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let last = nums[nums.len() - 1];
        last + nums_sum3(&nums[..nums.len() - 1])
    }
}

pub fn nums_sum4(sum: i32, nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        sum + nums[0]
    } else {
        nums_sum4(sum + nums[0], &nums[1..])
    }
}

pub fn nums_sum5(sum: i32, nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        sum + nums[0]
    } else {
        nums_sum5(sum + nums[nums.len() - 1], &nums[..nums.len() - 1])
    }
}

pub fn num2str(num: i32, base: i32) -> String {
    const BASESTR: [&str; 16] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ];

    if num < base {
        BASESTR[num as usize].to_string()
    } else {
        num2str(num / base, base) + BASESTR[(num % base) as usize]
    }
}

pub fn rec_mc1(cashes: &[u32], amount: u32) -> u32 {
    let mut min_cashes = amount;

    if cashes.contains(&amount) {
        return 1;
    } else {
        for c in cashes
            .iter()
            .filter(|&c| *c <= amount)
            .collect::<Vec<&u32>>()
        {
            let num_cashes = 1 + rec_mc1(&cashes, amount - c);
            if num_cashes < min_cashes {
                min_cashes = num_cashes;
            }
        }
    }
    min_cashes
}
