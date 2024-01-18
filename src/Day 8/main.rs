
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {   // "||" is an anonymous function
            self.head = node.next;
            node.data
        })
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    while let Some(value) = list.pop() {
        println!("Popped: {}", value);
    }

    println!("Is empty: {}", list.is_empty());
}
