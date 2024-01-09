fn kadane(arr:&mut Vec<i32>) -> i32{
    let mut currSum=0;
    let mut maxSum=i32::MIN;
    for element in arr.iter(){
        currSum+=element;
        if currSum>maxSum { maxSum=currSum; }
        if currSum<0 {currSum=0;}
    }
    return maxSum;
}

fn main() {
    let mut arr=vec![1,2,-3,4,5];
    let max_num=kadane(&mut arr);
    println!("{:?}",max_num);
}