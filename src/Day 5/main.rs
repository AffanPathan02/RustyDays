fn kadane(arr:&mut Vec<i32>) -> i32{
    let mut curr_sum=0;
    let mut max_sum=i32::MIN;
    for element in arr.iter(){
        curr_sum+=element;
        if curr_sum>max_sum { max_sum=curr_sum; }
        if curr_sum<0 {curr_sum=0;}
    }
    return max_sum;
}

fn main() {
    let mut arr=vec![1,2,-3,4,5];
    let max_num=kadane(&mut arr);
    println!("{:?}",max_num);
}