use basic_structure::deque::*;
// use basic_structure::queue::*;
// use basic_structure::stack::*;

fn main() {
    let text = "rustsur";
    let if_pal = deque::pal_check(text);
    println!("{:?}", if_pal);
}
