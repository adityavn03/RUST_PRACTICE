use borsh::{BorshDeserialize,BorshSerialize};
#[derive(BorshDeserialize,BorshSerialize,Debug)]
struct User{
    name:String,
    age:i32,
}

#[derive(BorshDeserialize,BorshSerialize,Debug)]
enum Dircetion {
    North(u8),
    West(u8),
    
}
fn main(){
    let user1=User{
        name:String::from("messi"),
        age:38,
    };
    let mut bit_storage:Vec<u8>=Vec::new();

    let struct_bit=user1.serialize(&mut bit_storage).expect("err");
    println!("{:?}",bit_storage);
    
    let bit_struct=User::try_from_slice(&bit_storage).expect("err");
    println!("{:?}",bit_struct);


    let mut vector:Vec<u8>=Vec::new();
    let enum_bits=Dircetion::West(9).serialize(&mut vector).expect("error");
    print!("{:?}",enum_bits);
    println!("vector : {:?}",vector);
    let bits_enums=Dircetion::try_from_slice(& vector).expect("error");
    println!("{:?}",bits_enums);
    match Dircetion::try_from_slice(&mut vector).expect("err"){
        Dircetion::North(val)=>{print!("{}",val)}
        Dircetion::West(val)=>{println!("{}",val)}
    }
}



