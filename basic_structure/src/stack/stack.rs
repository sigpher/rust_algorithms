#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
    top: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: Vec::new(),
            top: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top > 0 {
            self.top -= 1;
        }
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        return self.data.get(self.top - 1);
    }
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
    pub fn size(&self) -> usize {
        self.top
    }
}

pub fn par_checker(par: &str) -> bool {
    let mut stack = Stack::new();
    let chars = par.chars();
    for char in chars {
        if char == '(' || char == '[' || char == '{' {
            stack.push(char)
        }
        if char == ')' || char == ']' || char == '}' {
            if stack.is_empty() {
                return false;
            }
            let top = stack.pop().unwrap();
            if !par_match(top, char) {
                return false;
            }
        }
    }
    stack.is_empty()
}

fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

pub fn divide_by_two(mut dec_num: u32) -> String {
    let mut stack = Stack::new();

    while dec_num > 0 {
        let rem = dec_num % 2;
        stack.push(rem);
        dec_num /= 2;
    }
    let mut bin_str = String::new();
    while !stack.is_empty() {
        let rem = stack.pop().unwrap();
        bin_str = format!("{}{}", bin_str, rem);
    }
    bin_str
}

pub fn base_converter(mut dec_num: u32, base: u32) -> String {
    let mut stack = Stack::new();
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    while dec_num > 0 {
        let rem = dec_num % base;
        stack.push(rem);
        dec_num /= base;
    }
    let mut bin_str = String::new();
    while !stack.is_empty() {
        let rem = stack.pop().unwrap() as usize;
        bin_str = format!("{}{}", bin_str, &digits[rem])
    }
    bin_str
}
