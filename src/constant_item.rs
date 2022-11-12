use crate::*;
use std::fs::File;
use std::io::Read;
#[derive(Debug)]
pub enum ConstantItem{
    MethodRef(u16,u16),
    InterfaceMethodRef(u16,u16),
    InvokeDynamic(u16,u16),
    MethodHandle(u8,u16),
    FieldRef(u16,u16),
    Class(u16),
    NameAndType(u16,u16),
    UTF8(String),
    String(u16),
    Double(f64),
    Float(f32),
    Long(i64),
    Int(i32),
}
pub fn field_ref_from_index(index:u16,constant_items:&[ConstantItem])->(String,(String,String)){ 
    let index = match &constant_items[(index as usize) - 1]{
        ConstantItem::FieldRef(name_index,type_index)=>(name_index,type_index),
        _=>panic!("Expected field ref to get data from but got {:?}",&constant_items[(index as usize) - 1]), //TODO: more precise error message
    };
    (class_name_from_index(*index.0,constant_items),name_and_type_from_index(*index.1,constant_items))

}
pub fn read_float_at_index(constant_items:&[ConstantItem])->f32{
    todo!();
}
pub fn class_name_from_index(index:u16,constant_items:&[ConstantItem])->String{
    let class_name_index = match &constant_items[(index as usize) - 1]{
        ConstantItem::Class(class_name_index)=>class_name_index,
        _=>panic!("Expected class info to get name from but got {:?}",&constant_items[(index as usize) - 1]), //TODO: more precise error message
    };
    name_from_index(*class_name_index,constant_items)
}
pub fn method_from_index(index:u16,constant_items:&[ConstantItem])->(String,(String,String)){
    let index = match &constant_items[(index as usize) - 1]{
        ConstantItem::MethodRef(name_index,type_index)=>(name_index,type_index),
        _=>panic!("Expected method ref to get data from but got {:?}",&constant_items[(index as usize) - 1]), //TODO: more precise error message
    };
    (class_name_from_index(*index.0,constant_items),name_and_type_from_index(*index.1,constant_items))
}
pub fn interface_method_from_index(index:u16,constant_items:&[ConstantItem])->(String,(String,String)){
    let index = match &constant_items[(index as usize) - 1]{
        ConstantItem::InterfaceMethodRef(name_index,type_index)=>(name_index,type_index),
        _=>panic!("Expected method ref to get data from but got {:?}",&constant_items[(index as usize) - 1]), //TODO: more precise error message
    };
    (class_name_from_index(*index.0,constant_items),name_and_type_from_index(*index.1,constant_items))
}
pub fn name_and_type_from_index(index:u16,constant_items:&[ConstantItem])->(String,String){
    let index = match &constant_items[(index as usize) - 1]{
        ConstantItem::NameAndType(name_index,type_index)=>(name_index,type_index),
        _=>panic!("Expected name_and type o get data from but got {:?}",&constant_items[(index as usize) - 1]), //TODO: more precise error message
    };
    (name_from_index(*index.0,constant_items),name_from_index(*index.1,constant_items))
}
pub fn name_from_index(index:u16,constant_items:&[ConstantItem])->String{
    match &constant_items[(index as usize) - 1]{
        ConstantItem::UTF8(class_name)=>class_name.to_owned(),
        _=>panic!("Expected UTF8., but got {:?}!",&constant_items[(index as usize) - 1]), //TODO: more precise error message
    }
}
impl ConstantItem{
    pub fn read(f:&mut File)->ConstantItem{
        let const_type = read_u8(f);
        match const_type{
            1=>{
                let len = read_u16_be(f);
                let mut bytes = vec![0;len as usize];
                f.read(&mut bytes);
                ConstantItem::UTF8(
                    std::str::from_utf8(&bytes).expect("UTF8 constant pool item not a valid UTF8 string").to_owned()
                )
            },
            3=>{
                let int = read_i32_be(f);
                ConstantItem::Int(int)
            },
            4=>{
                let float = read_f32_be(f);
                ConstantItem::Float(float)
            },
            5=>{
                todo!("longs are not yet supported(they are buggy)");
                let long = read_i64_be(f);
                ConstantItem::Long(long)
            },
            6=>{
                todo!("doubles are not yet supported(they are buggy)");
                let double = read_f64_be(f);
                ConstantItem::Double(double)
            },
            7=>{
                let name_and_type_index = read_u16_be(f);
                ConstantItem::Class(name_and_type_index)
            },
            9=>{
                let class_index = read_u16_be(f);
                let name_and_type_index = read_u16_be(f);
                ConstantItem::FieldRef(class_index,name_and_type_index)
            },
            8=>{
                let string_index = read_u16_be(f);
                ConstantItem::String(string_index)
            },
            10=>{
                let class_index = read_u16_be(f);
                let name_and_type_index = read_u16_be(f);
                ConstantItem::MethodRef(class_index,name_and_type_index)
            },
            11=>{
                let class_index = read_u16_be(f);
                let name_and_type_index = read_u16_be(f);
                //IFACE
                ConstantItem::InterfaceMethodRef(class_index,name_and_type_index)
            },
            12=>{
                let name_index = read_u16_be(f);
                let type_index = read_u16_be(f);
                ConstantItem::NameAndType(name_index,type_index)
            },
            15=>{
                let reference_kind = read_u8(f);
                let reference_index = read_u16_be(f);
                ConstantItem::MethodHandle(reference_kind,reference_index)
            },
            18=>{
                let bootstrap_method_attr_index = read_u16_be(f);
                let name_and_type_index = read_u16_be(f);
                ConstantItem::InvokeDynamic(bootstrap_method_attr_index,name_and_type_index)
            }
            _=>panic!("Unsuported Const Item Type '{const_type}'"),
        }
    }
}
pub fn read_constant_item_pool(f:&mut File)->Box<[ConstantItem]>{
    let len = read_u16_be(f) - 1;
    let mut pool = Vec::with_capacity(len as usize);
    for i in 0..len{
        let item = ConstantItem::read(f);
        pool.push(item);
    }
    return pool.into();
}

