use std::fs::*;
use std::io::Read;
mod constant_item;
mod access_flags;
mod java_class;
use java_class::*;
use access_flags::*;
use constant_item::*;
#[derive(Debug)]
enum Atribute{

}
#[derive(Debug)]
struct AtributeInfo{
    name_index:u16,
    atribute:Atribute,
}
impl AtributeInfo{
    pub fn read(f:&mut File)->AtributeInfo{
        unimplemented!("Cant read AttributeInfo yet!");
    }
}
#[derive(Debug)]
pub struct FieldInfo{
    access_flags:FieldAccessFlags,
    name_index:u16,
    descriptor_index:u16,
    atributes:Box<[AtributeInfo]>
}
impl FieldInfo{
    pub fn read(f:&mut File)->FieldInfo{
        let access_flags = FieldAccessFlags::from_u16(read_u16_be(f));
        let name_index = read_u16_be(f);
        let descriptor_index = read_u16_be(f);
        let atributes_len = read_u16_be(f);
        let mut atributes = Vec::with_capacity(atributes_len as usize);
        for _ in 0..atributes_len{
            atributes.push(AtributeInfo::read(f));
        }
        let atributes:Box<[AtributeInfo]> = atributes.into();
        Self{access_flags,name_index,descriptor_index,atributes}
    }
}
pub fn read_u32_be(f:&mut File)->u32{
    let mut bytes = [0;4];
    if f.read(&mut bytes).expect("Could not read bytes") != 4{
        panic!("Could not read bytes!");
    }
    u32::from_be_bytes(bytes)  
}
pub fn read_u8(f:&mut File)->u8{
    let mut byte = [0;1];
    f.read(&mut byte).expect("Could not read byte");
    byte[0]
}
pub fn read_u16_be(f:&mut File)->u16{
    let mut bytes = [0;2];
    if f.read(&mut bytes).expect("Could not read bytes") != 2{
        panic!("Could not read bytes!");
    }
    u16::from_be_bytes(bytes)  
}
fn main() {
    let mut f = File::open("./Main.class").expect("Could not open file!");
    let class = JavaClass::from_file(&mut f).expect("Not a vaild Java class!");
    println!("Java:{:?}",class);
}
