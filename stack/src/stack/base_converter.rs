use crate::Stack;

pub fn base_converter(number: u32, base: u32) -> String {
    let degits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    let mut stack = Stack::new();
    let mut decimal_number = number;

    while decimal_number > 0 {
        stack.push(decimal_number % base);
        decimal_number /= base;
    }
    println!("{:?}", decimal_number);
    println!("{:?}", stack);

    let mut base_str = String::new();
    while !stack.is_empty() {
        let rem = stack.pop().unwrap() as usize;
        base_str += &degits[rem].to_string();
    }
    base_str
}
