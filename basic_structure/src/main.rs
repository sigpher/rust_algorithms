use basic_structure::{base_converter, divide_by_two, par_checker};

fn main() {
    // let sa = "(y = 5(x+1) * (2x-2))/2";
    let sa = "(*){{{{{{";
    let flag = par_checker(&sa);
    println!("{flag}");

    let bin_str = divide_by_two(233);
    println!("{}", bin_str);
    let ret = base_converter(233, 2);
    println!("{}", ret);
}
