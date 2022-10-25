use std::fs::File;
use crate::*;
use crate::constant_item::ConstantItem;
use crate::code::*;
#[derive(Debug)]
pub enum Attribute{
    Code(Code),
    LineNumberTable(Box<[(u16,u16)]>),
    ConstantValue(u16),
    LocalVaraibleTable(Box<[(u16,u16,String,String,u16)]>)
}
pub fn read_attributes(attribute_count:usize,f:&mut File,constant_items:&[ConstantItem])->Box<[Attribute]>{
    let mut res = Vec::with_capacity(attribute_count);
    for i in 0..attribute_count{
        res.push(Attribute::read(f,constant_items));
    }
    let res:Box<[Attribute]> = res.into();
    res
}
impl Attribute{
     pub fn read(f:&mut File,constant_items:&[ConstantItem])->Self{
        let name = crate::constant_item::name_from_index(read_u16_be(f),constant_items);
        let attribute_length = read_u32_be(f);
        match &name as &str{
            "Code"=>Attribute::Code(Code::read(f,constant_items)),
            "LineNumberTable"=>{
                let len = read_u16_be(f) as usize;
                let mut res = Vec::with_capacity(len);
                for _ in 0..len{
                    let start_pc = read_u16_be(f);
                    let line_number = read_u16_be(f);
                    res.push((start_pc,line_number));
                }
                let res:Box<[(u16,u16)]> = res.into();
                Attribute::LineNumberTable(res)
            },
            "ConstantValue"=>{
                Attribute::ConstantValue(read_u16_be(f))
            },
            "LocalVariableTable"=>{
                let local_variable_table_length = read_u16_be(f);
                let mut lvt = Vec::with_capacity(local_variable_table_length as usize);
                for _ in 0..local_variable_table_length{
                    let start_pc = read_u16_be(f);
                    let length = read_u16_be(f);
                    let name = crate::constant_item::name_from_index(read_u16_be(f),constant_items);
                    let descrpitor = crate::constant_item::name_from_index(read_u16_be(f),constant_items);
                    let index = read_u16_be(f);
                    lvt.push((start_pc,length,name,descrpitor,index));
                }
                Attribute::LocalVaraibleTable(lvt.into())
            },
            _=>panic!("Unsupported attribute:{name}!"),
        }
     }
}
