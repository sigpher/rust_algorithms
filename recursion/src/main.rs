use recursion::*;

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let ret = nums_sum(&nums);
    println!("ret1: {ret}");
    let ret2 = nums_sum2(&nums);
    println!("ret2: {ret2}");
    let ret3 = nums_sum3(&nums);
    println!("ret3: {ret3}");
    let ret4 = nums_sum4(0, &nums);
    println!("ret4: {ret4}");
    let ret5 = nums_sum4(0, &nums);
    println!("ret5: {ret5}");
    // let bin = num2str(100, 8);
    // println!("{bin}");

    let cashes = [1, 5, 10, 20, 50];
    let amount = 31_u32;

    let cashe_num = rec_mc1(&cashes, amount);
    println!("{cashe_num}");
}
