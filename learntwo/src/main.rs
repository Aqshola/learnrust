// use std::collections::HashMap;

// use std::{fs::File, io::ErrorKind};

use std::{fs::File, io::{self, Read}};

fn main() { 

    learn_duplication();
    // read_username_from_file_run();
    // learn_error_with_file();
    // learn_error();
    // learn_hashmap();
    // learn_string_vector();
    
}

fn learn_duplication(){
    let number_list=vec![34,1,2,3,444,332,44];

    let mut largest= &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}

// fn read_username_from_file_run(){
//     let res=read_username_from_file_simple();

//     // let res=read_username_from_file();

//     match res {
//         Ok(result)=>println!("{:?}",result),
//         Err(_)=>println!("Not Found")
//     }
// }

// fn read_username_from_file_simple() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }
// fn read_username_from_file()->Result<String, io::Error>{
//     let username_file_result=File::open("username.txt");
//     let mut username_file=match username_file_result {
//         Ok(file)=>file,
//         Err(err)=>return Err(err)
//     };

//     let mut username=String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_)   => Ok(username),
//         Err(err)=>Err(err)
//     }
// }

// fn learn_error_with_file(){
//     let greeting_file_result=File::open("hello.txt");

//     match greeting_file_result {
//         Ok(file)=>file,
//         Err(error)=>match error.kind() {
//             ErrorKind::NotFound=>match File::create("hello.txt") {
//                 Ok(fc)=>fc,
//                 Err(e)=>panic!("Problem creat file")
//             },
//             other_error=>{
//                 panic!("Problem opening file {:?}",other_error)
//             }
//         }
//     };
// }

// fn learn_error(){
//     // panic!("Something HAPPEN !!!");
//     let v=vec![1,2,3];
//     v[120];
// }

// fn learn_string_vector(){
//     let mut v:Vec<i32>=Vec::new();

//     for num  in 0..10  {
//             v.push(num);
//     }

//     let third= &v[2];
//     println!("the number is {third}");

//     let mut test="Aqshol";
//     println!("Before {test}");
//     test="afid";

//     println!("After {test}");
// }


// fn learn_hashmap(){
//     let mut scores= HashMap::new();

//     scores.insert(String::from("Ble"), 10);
//     scores.insert(String::from("Red"), 20);

//     let team_score=scores.get("Blue").copied().unwrap_or(0);

//     println!("{:?}",team_score);

    
//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");


    

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);

//     scores.entry(String::from("BLACK")).or_insert(90);

//     println!("{:?}",scores);


    
// }