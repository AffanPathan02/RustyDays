fn reversed_array(arr: &mut[i32;5] ){
    let mut start=0;
    let mut end=arr.len()-1;
    while start<=end {
        arr.swap(start,end);
        start+=1;
        end-=1;
    }
}

fn main(){
    let mut arr=[1,2,3,4,5];
    reversed_array(&mut arr);
    println!("{:?}",arr);

    let max_ele=arr.iter().max();
    println!("{:?}",max_ele);
}