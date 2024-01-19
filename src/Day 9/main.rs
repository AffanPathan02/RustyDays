use std::collections::VecDeque;

fn main() {
    let mut queue :VecDeque<i32>=VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    while let Some(value)=queue.pop_front(){
        println!("Dequeued Item:{}",value);
    }

    println!("Is queue empty ? {}",queue.is_empty());

    let mut stack:VecDeque<i32>=VecDeque::new();
    stack.push_front(1);
    stack.push_front(2);
    stack.push_front(3);

    while let Some(value)=stack.pop_front(){
        println!("Stack items are :{}",value);
    }
    println!("Is stack empty? {}",stack.is_empty());
}