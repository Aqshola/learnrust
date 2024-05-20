use std::any::Any;

fn main() {
    // learn_part_one();
    // learn_part_two();
    // learn_part_three();

    // learn_part_four();

    // learn_part_five();

    // learn_part_six();

    // learn_part_seven();

    learn_part_eight();

}




// fn learn_part_one(){
//     //LEARN SHADOWING
//     let x=6;

//     let x=x+1;

//     {
//         let x=x*2;
//         println!("Scoped x = {x}");
//     }

//     println!("Outer x = {x}");

//     //NUMBER ANOTATION
//     let x=100_000_000;

//     println!("Formatted : {x}");

//     //COMPUND TYPE RUST

//     //TUPLE => Bisa berisi banyak tipe
//     let tup:(i32,&str)=(200,"abc");
//     let (a,b)= tup;

//     println!("{a}, {b}");


//     //ARRAY
//     let array_a=[1,2,3,4,3];

//     let first=array_a[0];

//     println!("{first}");
// }



// fn learn_part_two() {
//     printing_value_x(2);


//     //FUNCITON EXPRESSION
//     let yaya={
//         let abc=2;
//         abc+2
//     };

//     println!("{yaya}");
    
//     let new_arrow=arrow_return();

//     println!("new Arrow : {new_arrow}");
// }

// fn learn_part_three(){
//     let mut count=0;
//     'counting_up:loop {
//         println!("Counting");
//         let mut remaining=9;

//         loop {
//             println!("Remaining = {remaining}");

//             if remaining==5{
//                 break;
//             }

//             if count==2 {
//                 println!("Break Count");
//                 break 'counting_up

//             }

//             remaining-=1;
//         }
        
//         count+=1;
//     }
// }


// fn learn_part_four(){
//     let mut s=String::from("Sasageyo");
//     s.push_str("Shinzou");

//     println!("{s}");

//     let x=String::from("valueone");
//     let _y=x;

//     // println!("{x}");

//     let tes_owner=String::from("Hellow");

//     takes_ownershop(tes_owner);

//     let tes_copy=5;
//     makes_copy(tes_copy);

    
// }

// fn learn_part_five(){
//     let mut s1=String::from("lalatina");
//     change_str(&mut s1);

//     let len=calculate_length_str(&s1);
//     println!("The length of {} is {}",s1, len);
// }


// fn learn_part_six(){
//     let str=String::from("Hello World");
//     let len_str=str.len();
//     let part_one=&str[0..len_str];

//     println!("Part One : {}",part_one);
// }
// #[derive(Debug)]
// struct User{
//     name: String,
//     age: u32,
// }

// impl User{
//     fn get_name(&self)->&String{
//         &self.name
//     }
// } 



// fn learn_part_seven(){
//     let new_user=User{
//         age:20,
//         name:String::from("Ereh"),
//     };


//     let user_name=new_user.get_name();
//     println!("The name is {} with age {}", new_user.name,new_user.age);
//     println!("Struct user is {:?}",new_user);

//     println!("Get name result : {}",user_name);
// }


#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn learn_part_eight(){
    let ip_four=IpAddrKind::V4;
    let ip_six=IpAddrKind::V6;

    println!("IPFOUR {:?} and IPSIX {:?}",ip_four.type_id(),ip_six);

    value_in_cents(Coin::Nickel);


}


//fn arrow return 
// fn printing_value_x(x:i32){
//     println!("The value of x is: {x}");

// }
// fn arrow_return()->i32{
//     5
// }


// //LEARNING FUNCTION

// fn calculate_length_str(str:&String)-> usize{
//     str.len()
// }

// fn change_str(str:&mut String){
//     str.push_str(" Lalatina ");
// }


// fn another_function(x:i32){
//     println!("Value of Parameter x is {x}")
// }

// fn takes_ownershop(some_string:String){
//     println!("{}",some_string)
// }

// fn makes_copy(some_integer:i32){
//     println!("{}",some_integer)
// }