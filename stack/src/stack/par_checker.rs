use crate::Stack;

pub fn par_checker(text: &str) -> bool {
    let mut stack = Stack::new();
    let mut balance = true;

    text.chars()
        .filter(|&c| c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}')
        .for_each(|c| {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
            } else {
                if c == ')' || c == ']' || c == '}' {
                    if stack.is_empty() {
                        balance = false;
                        return;
                    }
                    let top = stack.pop().unwrap();
                    // println!("{}", top);
                    if !par_match(top, c){
                        balance = false;
                        return;
                    }
                }
            }
        });
    balance && stack.is_empty()
}

fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}
