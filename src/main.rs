use std::fs::*;
use std::io::{Read,Write};
pub(crate) mod constant_item;
pub(crate) mod access_flags;
pub(crate) mod java_class;
pub(crate) mod field;
pub(crate) mod io;
pub(crate) mod attribute;
pub(crate) mod method;
pub(crate) mod code;
pub(crate) use io::*;
pub(crate) mod type_mappings;
use java_class::*;
use crate::type_mappings::TypeMappings;
fn write_assembly<T:Write>(classes:&[JavaClass],file:&mut T,asm_name:&str,mappings:&TypeMappings)->std::io::Result<()>{
    writeln!(file,".assembly {asm_name} {{}}")?;
    writeln!(file,".assembly extern mscorlib {{}}")?;
    //writeln!(file,".assembly extern jbridge {{}}")?;
    for class in classes{
        class.write_to_asm(file,mappings);
    }
    Ok(())
}
fn main() {
    let mappings = TypeMappings::from_file("./type_mappings.json");
    let class = JavaClass::in_directory(".");
    let mut out = File::create("./target/result.il").expect("Could not create output assembly!");
    println!("Java:{:?}",class);
    write_assembly(&class,&mut out,"TestAssembly",&mappings);

}
