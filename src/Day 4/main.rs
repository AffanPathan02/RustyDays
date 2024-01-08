fn reverse_array(arr:&mut Vec<i32>,start:usize,end:usize){
    let mut i=start;
    let mut j=end;
    while i<j{
        arr.swap(i,j);
        i+=1;
        j-=1;
    }
}

fn rotate_array(arr:&mut Vec<i32>, k:usize){
    let k=k%arr.len();
    reverse_array(arr,0,arr.len()-1);
    reverse_array(arr,0,k-1);
    reverse_array(arr,k,arr.len()-1);
}

fn main() {
    let mut arr=vec![1,2,3,4,5];
    arr.rotate_left(1);
    arr.rotate_right(1);
    println!("{:?}",arr);

    rotate_array(&mut arr,2);
    println!("{:?}",arr);
}