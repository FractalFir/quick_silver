use std::fs::*;
use std::io::Read;
mod constant_item;
mod access_flags;
mod java_class;
use java_class::*;
use access_flags::*;
use constant_item::*;
pub fn read_u32_be<T:Read>(f:&mut T)->u32{
    let mut bytes = [0;4];
    if f.read(&mut bytes).expect("Could not read bytes") != 4{
        panic!("Could not read bytes!");
    }
    u32::from_be_bytes(bytes)  
}
pub fn read_i64_be<T:Read>(f:&mut T)->i64{
    let mut bytes = [0;std::mem::size_of::<i64>()];
    if f.read(&mut bytes).expect("Could not read bytes") != 8{
        panic!("Could not read bytes!");
    }
    i64::from_be_bytes(bytes)  
}
pub fn read_i32_be<T:Read>(f:&mut T)->i32{
    let mut bytes = [0;std::mem::size_of::<i32>()];
    if f.read(&mut bytes).expect("Could not read bytes") != std::mem::size_of::<i32>(){
        panic!("Could not read bytes!");
    }
    i32::from_be_bytes(bytes)  
}
pub fn read_u8<T:Read>(f:&mut T)->u8{
    let mut byte = [0;1];
    f.read(&mut byte).expect("Could not read byte");
    byte[0]
}
pub fn read_u16_be<T:Read>(f:&mut T)->u16{
    let mut bytes = [0;2];
    if f.read(&mut bytes).expect("Could not read bytes") != 2{
        panic!("Could not read bytes!");
    }
    u16::from_be_bytes(bytes)  
}
pub fn read_f64_be<T:Read>(f:&mut T)->f64{
    let mut bytes = [0;8];
    if f.read(&mut bytes).expect("Could not read bytes") != 8{
        panic!("Could not read bytes!");
    }
    f64::from_be_bytes(bytes)  
}
pub fn read_f32_be<T:Read>(f:&mut T)->f32{
    let mut bytes = [0;4];
    if f.read(&mut bytes).expect("Could not read bytes") != 4{
        panic!("Could not read bytes!");
    }
    f32::from_be_bytes(bytes)  
}
fn main() {
    let mut f = File::open("./Main.class").expect("Could not open file!");
    let class = JavaClass::from_file(&mut f).expect("Not a vaild Java class!");
    println!("Java:{:?}",class);
}
