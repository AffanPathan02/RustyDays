use std::collections::LinkedList;

fn main() {
    let mut list:LinkedList<i32>=LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);

    while let Some(value)=list.pop_front(){
        println!("Popped:{}",value);
    }
    println!("is it empty? -> {}", list.is_empty());
}