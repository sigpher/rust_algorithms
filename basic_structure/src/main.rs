use basic_structure::LVec;
// use basic_structure::deque::*;
// use basic_structure::queue::*;
// use basic_structure::stack::*;
use basic_structure::linkedlist::*;
// use basic_structure::liststack::*;

fn main() {
    let mut lvec = LVec::new();
    lvec.push(10);
    lvec.push(11);
    lvec.push(12);
    lvec.push(13);
    lvec.insert(0, 9);

    let mut lvec2 = LVec::new();
    lvec2.insert(0, 8);
    lvec2.append(&mut lvec);
    println!("lvec2 len: {}", lvec2.len());
    lvec2.print_lvec();

    let res1 = lvec2.pop();
    let res2 = lvec2.remove(0);
    println!("pop {:#?}", res1.unwrap());
    println!("remove {:#?}", res2.unwrap());
}

fn basics() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.peek(), Some(&2));
    assert_eq!(list.peek_mut(), Some(&mut 2));

    list.peek_mut().map(|val| *val = 4);

    assert_eq!(list.peek(), Some(&4));
    println!("basics test Ok!");
}

fn into_iter() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    println!("into_iter test Ok!");
}

fn iter() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
    println!("iter test Ok!");
}

fn iter_mut() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.iter_mut();

    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), None);
    println!("iter_mut test Ok!");
}
