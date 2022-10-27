use inkwell::types::AnyTypeEnum;
use inkwell::context::ContextRef;
#[derive(Debug,Clone,Copy)]
pub enum Field{
    Byte(i8),  // B
    Char(char), // C
    Int(i32), // I
    Double(f64), // D
    Float(f32), // F
    Long(i64), // J
    Short(i16), // S
    Bool(bool), // Z
    Object(u64),//L TODO: Support object constructors! *0* class index
    Array(u64),//L TODO: Support array constructors! *0* class index
}
impl Field{
    pub fn get_llvm_type<'a>(&self,ctx:ContextRef<'a>)->AnyTypeEnum<'a>{
        match self{
            Field::Byte(_)=>AnyTypeEnum::IntType(ctx.i8_type()),
            Field::Char(_)=>AnyTypeEnum::IntType(ctx.i32_type()),
            Field::Int(_)=>AnyTypeEnum::IntType(ctx.i32_type()),
            Field::Double(_)=>AnyTypeEnum::FloatType(ctx.f64_type()),
            Field::Float(_)=>AnyTypeEnum::FloatType(ctx.f32_type()),
            _=>todo!("Field type {:?} is not supported yet!",self),
        }
    }
}
