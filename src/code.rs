use std::fs::File;
use crate::*;
use crate::constant_item::ConstantItem;
use crate::attribute::{read_attributes,Attribute};
use crate::constant_item::{class_name_from_index,name_from_index,method_from_index,interface_method_from_index};
// *n* field 
// [n] stack form to
// <n> local variable
#[derive(Debug)]
pub enum OpCode{
    Nop,
    AStore(u8), // Store local object reference to [n] in <*0*>
    ALoad(u8), // Load local object reference <*0*> to [n]
    ConstIntVal(i32), // push const int *0* on the stack to [0]
    PutField(u16), // set field *0* of [0] to [1]
    GetField(u16), // get field *0* of [0] and pushes it to stack
    GetStatic(u16), // Gets static filed at *0* to [0]
    PutStatic(u16), // Set static *0* to [1]
    PushByte(u8), // pushes byte *0* to [0]
    PushShort(u16), // pushes short *0* to [0]
    LoadConstant(u16), // loads constant at field *0* and pushes it to [0]
    RetVoid, // Returns void from a method
    RetLong, // Returns long from a method
    RetA,// Returns an object reference from a method
    CheckCast(String), // Checks if object [0] is type *0* and if so pushes it to stack ?(otherwise throws an exception)?
    IStore(u8), // Store i32 [0] into <*0*>
    ILoad(u8), // Load i32 <*0*> into [0]
    LLoad(u8),// Load i64 <*0*> into [0]
    IMul, // Multiply i32 [0] by [1] and write int to [0]
    InvokeVirtual(String,(String,String)), // invokes virtual method *0* on object [0]
    InvokeInterface(String,(String,String)), // invokes interface method *0* on object [0]
    InvokeSpecial(String,(String,String)), // Invoke special method on class *0* with name *1* and type *2*
    InvokeStatic(String,(String,String)), // Invoke static method on class *0* with name *1* and type *2*
    Pop, // Discard the top value on the stack
    Dup, // Duplicate i32 on top of the stack
    New(String), // creates an new instance of class *0*
    ANewArray(String), // create new array of [0] elements of type *0*.
    ArrayLen, // get length of array [0]
    Ba, // get boolean value [1] from array [0]
    IfEq(i16), // Checks if [0] is zero, and if so, jump by i16 *0*
    IfLe(i16), // Checks if [0] is less than 0, and if so jump by i16 *0*
    IfCmpGE(i16), // Checks if [0] is greater on equal to [1], and if so jump by i16 *0*
    AConstNull,
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
                1=>OpCode::AConstNull,
                2..=8=>OpCode::ConstIntVal((opCode as i8 - 3) as i32),
                16=>{
                    code_length -= 1;
                    OpCode::PushByte(read_u8(f))
                },
                17=>{
                    code_length -= 2;
                    OpCode::PushShort(read_u16_be(f))
                },
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
                21=>{
                    let index = read_u8(f);
                    code_length -= 1;
                    OpCode::ILoad(index)
                },
                25=>{
                    code_length -= 1;
                    OpCode::ALoad(read_u8(f))
                },
                26..=29=>OpCode::ILoad(opCode - 26),
                30..=33=>OpCode::LLoad(opCode - 30),
                42..=45=>OpCode::ALoad(opCode - 42),
                54=>{
                    let index = read_u8(f);
                    code_length -= 1;
                    OpCode::IStore(index)
                },
                58=>{
                    code_length -= 1;
                    OpCode::AStore(read_u8(f))
                },
                59..=63=>OpCode::IStore(opCode - 59),
                74..=78=>OpCode::AStore(opCode - 74),
                84=>OpCode::Ba,
                87=>OpCode::Pop,
                89=>OpCode::Dup,
                104=>OpCode::IMul,
                153=>{
                    code_length -= 2;
                    let offset = read_i16_be(f);
                    OpCode::IfEq(offset)
                },
                158=>{
                    code_length -= 2;
                    let offset = read_i16_be(f);
                    OpCode::IfLe(offset)
                },
                162=>{
                    code_length -= 2;
                    let offset = read_i16_be(f);
                    OpCode::IfCmpGE(offset)
                },
                173=>OpCode::RetLong,
                176=>OpCode::RetA,
                177=>OpCode::RetVoid,
                178=>{
                    code_length -= 2;
                    Self::get_static(read_u16_be(f),constant_items)
                },
                179=>{
                    let static_field_index = read_u16_be(f);
                    code_length -= 2;
                    OpCode::PutStatic(static_field_index)
                },
                181=>{
                    let field_index = read_u16_be(f);
                    code_length -= 2;
                    OpCode::PutField(field_index)
                },
                180=>{
                    let field_index = read_u16_be(f);
                    code_length -= 2;
                    OpCode::GetField(field_index)
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
                184=>{
                    let method_index = read_u16_be(f);
                    code_length -= 2;
                    let method = method_from_index(method_index,constant_items);
                    OpCode::InvokeStatic(method.0,method.1)
                },
                185=>{
                    let method_index = read_u16_be(f);
                    let count = read_u8(f);
                    let _align = read_u8(f);
                    code_length -= 2;
                    let method = interface_method_from_index(method_index,constant_items);
                    OpCode::InvokeInterface(method.0,method.1)
                },
                /*
                186=>{
                    let method_index = read_u16_be(f);
                    code_length -= 2;
                    let method = interface_method_from_index(method_index,constant_items);
                    OpCode::InvokeDynamic(method.0,method.1)
                },
                */
                187=>{
                    let class_index = read_u16_be(f);
                    code_length -= 2;
                    OpCode::New(class_name_from_index(class_index,constant_items))
                },
                189=>{
                    let class_index = read_u16_be(f);
                    code_length -= 2;
                    OpCode::ANewArray(class_name_from_index(class_index,constant_items))
                },
                190=>OpCode::ArrayLen,
                192=>{
                    let class_index = read_u16_be(f);
                    code_length -= 2;
                    OpCode::CheckCast(class_name_from_index(class_index,constant_items))
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
pub struct Exception{
    start_pc:u16,
    end_pc:u16, // does not include opcode at end_pc!
    handler_pc:u16,
    catch_type:String,
}
impl Exception{
   pub fn read(f:&mut File,constant_items:&[ConstantItem])->Self{
        let start_pc = read_u16_be(f);
        let end_pc = read_u16_be(f);
        let handler_pc = read_u16_be(f);
        let catch_type = class_name_from_index(read_u16_be(f),constant_items);
        Self{start_pc,end_pc,handler_pc,catch_type}
   }
}
#[derive(Debug)]
pub struct Code{
    pub max_stack:u16,
    pub max_locals:u16,
    pub code:Box<[OpCode]>,
    // Exceptions
    pub exceptions:Box<[Exception]>,
    pub attributes:Box<[Attribute]>,
}
impl Code{
   pub fn read(f:&mut File,constant_items:&[ConstantItem])->Self{
        let max_stack = read_u16_be(f);
        let max_locals = read_u16_be(f);
        let code_length = read_u32_be(f);
        let code = OpCode::read_opcodes(f,code_length,constant_items);
        println!("code:{code:?}");
        let exception_table_length = read_u16_be(f);
        let mut excepiton_table:Vec<Exception> = Vec::with_capacity(exception_table_length as usize);
        for i in 0..exception_table_length{
            excepiton_table.push(Exception::read(f,constant_items));
        }
        let exceptions = excepiton_table.into();
        let attributes = read_attributes(read_u16_be(f) as usize,f,constant_items);
        Self{max_stack,max_locals,code,attributes,exceptions}
    }
} 
