use std::fs::*;
use std::io::Read;
mod constant_item;
mod access_flags;
mod java_class;
mod field;
mod io;
mod attribute;
mod method;
use java_class::*;
pub(crate) use io::*;
fn main() {
    let mut f = File::open("./Main.class").expect("Could not open file!");
    let class = JavaClass::from_file(&mut f).expect("Not a vaild Java class!");
    println!("Java:{:?}",class);
}
