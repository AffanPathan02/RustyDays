fn main() {
    let mut array:[i32;5]=[1,2,3,4,5];
    array.reverse();
    println!("{:?}", array);

    let arr2=[1,2,3,4,5,6,7,8];
    let rev_iter=arr2.iter().rev();
    let rev_array:Vec<_>=rev_iter.collect();
    println!("{:?}",rev_array)
}
