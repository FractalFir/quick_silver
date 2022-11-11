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
use java_class::*;
fn map_java_class_to_cli(mut path:std::str::Split<char>,res:&mut String){
    match path.next().unwrap(){
        "lang"=>{
            let next = path.next().unwrap();
             match next{
                "String;"=>res.push_str("[System] System.String"),
                "Object;"=>res.push_str("[System] System.Object"),
                _=>todo!("Use of unsupported java standard library class {next}"),
             }
        }
        _=>todo!("Use of unsupported java standard library class"),
    }
}
fn java_class_path_to_cli_path(path:&str)->String{
    let mut res = String::with_capacity(path.len());
    let mut path = path.split('/');
    let first = path.next().unwrap();
    match first{
        "java"=>{
            map_java_class_to_cli(path,&mut res);
            return res;
        },
        _=>{
            res.push_str(first);
            res.push('.');
        }
    }
    todo!("res:{res}");
}
fn write_assembly<T:Write>(classes:&[JavaClass],file:&mut T,asm_name:&str)->std::io::Result<()>{
    writeln!(file,".assembly {asm_name} {{}}")?;
    writeln!(file,".assembly extern mscorlib {{}}")?;
    //writeln!(file,".assembly extern jbridge {{}}")?;
    for class in classes{
        class.write_to_asm(file);
    }
    write!(file,".method static void Main()\n{{\n.entrypoint\n.maxstack 1\nldstr \"Hello, World!\"\ncall void [mscorlib]System.Console::WriteLine(string)\nret\n}}")?;
    Ok(())
}
fn main() {
    let mut f = File::open("./Main.class").expect("Could not open file!");
    let class = JavaClass::from_file(&mut f).expect("Not a vaild Java class!");
    let mut out = File::create("./target/result.il").expect("Could not create output assembly!");
    println!("Java:{:?}",class);
    write_assembly(&[class],&mut out,"TestAssembly");

}
