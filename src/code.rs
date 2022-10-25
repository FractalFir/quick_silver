use std::fs::File;
use crate::*;
use crate::constant_item::ConstantItem;
use crate::attribute::{read_attributes,Attribute};
use crate::constant_item::{class_name_from_index,name_from_index,method_from_index};
// *n* field 
// [n] stack form to
// <n> local variable
#[derive(Debug)]
pub enum OpCode{
    Nop,
    ALoad(u8), // Load local object reference *0* to [n]
    InvokeSpecial(String,(String,String)), // Invoke special method on class *0* with name *1* and type *2*
    ConstIntVal(i32), // push const int *0* on the stack to [0]
    PutField(u16), // set field *0* of [0] to [1]
    PushByte(u8), // pushes byte *0* to [0]
    LoadConstant(u16), // loads constant at field *0* and pushes it to [0]
    RetVoid, // Returns void from method
    IStore(u8), // Store [0] into <*0*>
    ILoad(u8), // Load <*0*> into [0]
    IMul, // Multiply [0] by [1] and write int to [0]
    GetStatic(u16), // Gets static filed at
    InvokeVirtual(String,(String,String)), // invokes virtual method *0* on object [0]
}

impl OpCode{
    fn get_static(index:u16,constant_items:&[ConstantItem])->Self{
        OpCode::GetStatic(index) // TODO: change some statics which can be evaluated at compile time
        // to constant values.
    }  
    fn load_constant(index:u16,constant_items:&[ConstantItem])->Self{
        OpCode::LoadConstant(index) // TODO: change some constants to values.
    }
    pub fn read_opcodes(f:&mut File,mut code_length:u32,constant_items:&[ConstantItem])->Box<[Self]>{
       let mut res = Vec::with_capacity(code_length as usize);
       println!("BEGIN OP READ!");
       while code_length != 0{
            let opCode = read_u8(f);
            code_length -= 1;
            let code = match opCode{
                0=>OpCode::Nop,
                2..=8=>OpCode::ConstIntVal((opCode as i8 - 3) as i32),
                18=>{
                    code_length -= 1;
                    Self::load_constant(read_u8(f) as u16,constant_items)
                },
                19=>{
                    code_length -= 2;
                    Self::load_constant(read_u16_be(f),constant_items)
                },
                20=>{
                    code_length -= 2;
                    Self::load_constant(read_u16_be(f),constant_items)
                },
                16=>{
                    code_length -= 1;
                    OpCode::PushByte(read_u8(f))
                },
                26..=29=>OpCode::ILoad(opCode - 26),
                42..=45=>OpCode::ALoad(opCode - 42),
                59..=63=>{
                    OpCode::IStore(opCode - 59)
                },
                104=>OpCode::IMul,
                177=>OpCode::RetVoid,
                178=>{
                    code_length -= 2;
                    Self::get_static(read_u16_be(f),constant_items)
                },
                181=>{
                    let field_index = read_u16_be(f);
                    code_length -= 2;
                    OpCode::PutField(field_index)
                },
                182=>{
                    let method_index = read_u16_be(f);
                    code_length -= 2;
                    let method = method_from_index(method_index,constant_items);
                    OpCode::InvokeVirtual(method.0,method.1)
                },
                183=>{
                    let method_index = read_u16_be(f);
                    code_length -= 2;
                    let method = method_from_index(method_index,constant_items);
                    OpCode::InvokeSpecial(method.0,method.1)
                },
                _=>panic!("Unhanded opCode:{opCode} hex: {opCode:x}!"),
            };
            res.push(code);
       }
       let res:Box<[Self]> = res.into(); 
       res
    }
}
#[derive(Debug)]
pub struct Code{
    pub max_stack:u16,
    pub max_locals:u16,
    pub code:Box<[OpCode]>,
    // Exceptions
    pub attributes:Box<[Attribute]>,
}
impl Code{
   pub fn read(f:&mut File,constant_items:&[ConstantItem])->Self{
        let max_stack = read_u16_be(f);
        let max_locals = read_u16_be(f);
        let code_length = read_u32_be(f);
        let code = OpCode::read_opcodes(f,code_length,constant_items);
        println!("code:{code:?}");
        let execption_table_length = read_u16_be(f);
        for i in 0..execption_table_length{
            todo!("exceptions unsupported!");
        }
        let attributes = read_attributes(read_u16_be(f) as usize,f,constant_items);
        Self{max_stack,max_locals,code,attributes}
    }
} 
