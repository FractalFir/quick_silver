use std::fs::File;
use crate::*;
use crate::constant_item::ConstantItem;
#[derive(Debug)]
pub struct Code{
    pub max_stack:u16,
    pub max_locals:u16,
    pub code:Box<[u8]>,
    // Exceptions
    pub attributes:Box<[Attribute]>,
}
impl Code{
   pub fn read(f:&mut File,constant_items:&[ConstantItem])->Self{
        let max_stack = read_u16_be(f);
        let max_locals = read_u16_be(f);
        let code_length = read_u32_be(f);
        let mut code:Vec<u8> = vec![0; code_length as usize];
        f.read(&mut code).expect("Could not read code!");
        let code:Box<[u8]> = code.into();
        let execption_table_length = read_u16_be(f);
        for i in 0..execption_table_length{
            todo!("exceptions unsupported!");
        }
        let attributes = read_attributes(read_u16_be(f) as usize,f,constant_items);
        Self{max_stack,max_locals,code,attributes}
    }
}
#[derive(Debug)]
pub enum Attribute{
    Code(Code),
    LineNumberTable(Box<[(u16,u16)]>),
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
        let name = crate::java_class::name_from_index(read_u16_be(f),constant_items);
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
            _=>panic!("Unsupported attribute:{name}!"),
        }
     }
}
