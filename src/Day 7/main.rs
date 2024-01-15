use std::cmp::max;

#[derive(Debug)]
struct TreeNode{
    value:i32,
    left: Option<Box<TreeNode>>,        //the left or right child either contains "Some" value or None
    right:Option<Box<TreeNode>>
}

impl TreeNode{
    fn new(value:i32)->Self{
        TreeNode{
            value,
            left:None,
            right:None,
        }
    }

    fn insert(&mut self, value:i32){
        if value<self.value{
            if let Some(ref mut left)=self.left{        //"ref" is used to borrow the value without taking ownership of it
                left.insert(value);
            }else{
                self.left=Some(Box::new(TreeNode::new(value)));
            }
        }
        else{
            if let Some(ref mut right)=self.right{
                right.insert(value);
            }else{
                self.right=Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    fn height(&self)-> i32{
        let mut count:i32=1;

        if let Some (ref left)=self.left{
            count+=left.height();
        }

        if let Some (ref right)=self.right{
            count=std::cmp::max(count,right.height());
        }
        count

    }

    fn height_rs(&self)->i32{
        match (self.left.as_ref(),self.right.as_ref()) {
            (Some(left),Some(right))=>1+std::cmp::max(left.height(),right.height()),
            (Some(left),None)=>1+left.height(),
            (None,Some(right))=>1+right.height(),
            (None,None)=>1,
        }
    }

    fn preorder(&self){
        print!("{} ",self.value);
        if let Some(ref left)=self.left{
            left.preorder();
        }
        if let Some(ref right)=self.right{  //"ref" is used to borrow the value without taking ownership of it
            right.preorder();
        }
    }

    fn postorder(&self){
        if let Some(ref left)=self.left {
            left.postorder();
        }

        if let Some(ref right)=self.right{
            right.postorder();
        }

        print!("{} ",self.value);
    }
}

fn main() {
    let mut root=TreeNode::new(5);
    root.insert(3);
    root.insert(8);
    root.insert(2);
    root.insert(4);
    root.insert(7);

    // println!("{:?}",root);
    root.preorder();
    println!("");
    root.postorder();
    println!("height :{:?}",root.height());
    println!("height :{:?}",root.height_rs());
}