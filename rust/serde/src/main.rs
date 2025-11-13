use serde::{Serialize,Deserialize};
#[derive(Deserialize,Serialize,Debug)]
struct User{
  name:String,
  age:i32,
  mark:Vec<i32>,
}

fn main(){
  let user1=User{
    name:String::from("messi"),
    age:5,
    mark:vec!(1,2,3,4,5,6),
  };
  let struct_string=serde_json::to_string(&user1).expect("err");
  println!("{}",struct_string);
  let string_struct:Result<User,serde_json::Error>=serde_json::from_str(&struct_string);
  match string_struct{
    Ok(val)=>{println!("{:?}",val)}
    Err(_)=>{println!("error")}
  }
  
  
}