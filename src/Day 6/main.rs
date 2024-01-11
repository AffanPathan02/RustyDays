enum Coin{
    Rupees,
    Paise,
    Deno(String)
}

fn main() {
    let mut name="affan";       //example of string slice
    let mut name2=String::from("affan");      //example of Owned String
    println!("Operation on name which is a slice string ");
    if name.contains("x"){
        println!("a is there");
    }

    if name==name2{
        println!("equal he beta")
    }

    let x=String::from("Affan");
    let y=&x;
    println!("{:?}",x);
    println!("{:?}",y);


    let input="affan";
    match input.parse::<i32>() {
        Ok(number)=>{
            println!("parsed number:{}",number);
        }
        Err(e)=>{
            println!("some error occured:{}",e);
        }
    }

    let mut owner_string=String::from("Hello, Ownership");
    take_ownership(owner_string);
    //println!("After ownership is taken:{}",owner_string);

    let burrowed_string=String::from("Hello Burrowing");
    borrow_string(&burrowed_string);
    println!("After burrowing :{}",burrowed_string);

    let z=Coin::Deno(String::from("Five Rupees"));
    match z {
        Coin::Paise=>println!("Paise mile"),
        Coin::Rupees=>println!("Rupees mile"),
        Coin::Deno(name)=>{
            println!("tere baap ka paisa he kya aee")

        }
    }

    let a=String::from("123abc");
    for b in a.bytes(){
        println!("{}",b)
    }

    for b in a.chars(){
        println!("{}",b)
    }

    for (b,c) in a.chars().enumerate(){
        println!("Index :{}, Character:{}",b,c)
    }
}

fn take_ownership(s:String) -> String{
    println!("inside take_ownership:{}",s);
    return s
}

fn borrow_string(s: &String){
    println!("inside borrow_string:{}",s);
}