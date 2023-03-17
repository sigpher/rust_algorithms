use stack::stack::base_converter;

fn main() {
    // let text = "hello world ()(())()[]";
    // println!("{}", par_checker(text));

    let ret = base_converter::base_converter(40, 2);
    println!("return: {}", ret);
}
