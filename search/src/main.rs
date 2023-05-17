use search;
use search::binary_search::*;
use search::sequential_search::*;

fn main() {
    let is_here = sequential_search_pos(&[1, 2, 10, 3], 10);
    if let Some(found) = is_here {
        println!("{found}")
    } else {
        println!("None")
    }

    let nums = [1, 4, 5, 9, 12];
    let found = ordered_sqeuentail_search(&nums, 10);

    println!("{found}");

    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];
    let target = 3;
    let found = binary_search1(&nums, target);
    println!("{target} is in nums: {found}");

    let num = 63;
    let found = binary_search2(&nums, num);
    println!("{num} is in nums: {found}");
}
