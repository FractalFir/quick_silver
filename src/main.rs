use std::fs::*;
use std::io::Read;
mod context;
mod class;
mod field;
mod method;
mod preprocessing;
use preprocessing::java_class::*;
fn main() {
    let mut f = File::open("./Main.class").expect("Could not open file!");
    let class = JavaClass::from_file(&mut f).expect("Not a vaild Java class!");
    println!("Java:{:?}",class);
    let llvm_ctx = context::QuickSilverContext::create_default_llvm_context();
    let ctx = context::QuickSilverContext::create(&[class],&llvm_ctx);
}
