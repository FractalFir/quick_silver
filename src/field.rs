use inkwell::types::AnyTypeEnum;
use inkwell::context::Context;
use crate::preprocessing::*;
#[derive(Debug,Clone)]
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
    Array(Box<Field>),//L TODO: Support array constructors! *0* class index
}
impl Field{
    pub fn get_llvm_type<'a>(&self,ctx:&'a Context)->AnyTypeEnum<'a>{
        match self{
            Field::Byte(_)=>AnyTypeEnum::IntType(ctx.i8_type()),
            Field::Char(_)=>AnyTypeEnum::IntType(ctx.i32_type()),
            Field::Int(_)=>AnyTypeEnum::IntType(ctx.i32_type()),
            Field::Double(_)=>AnyTypeEnum::FloatType(ctx.f64_type()),
            Field::Float(_)=>AnyTypeEnum::FloatType(ctx.f32_type()),
            _=>todo!("Field type {:?} is not supported yet!",self),
        }
    }
    pub fn from_java_field(field:&field::Field)->(String,Self){
        let name = field.name.to_owned();
        let mut desctiptor = field.descriptor.chars();
        let res = match &desctiptor.next().expect("Field descriptor must consist of at least one character!"){
            'B'=>{
                if field.descriptor.len() != 1{
                    panic!("Invalid descriptor:\"{}\". It begins with 'B' which suggest that it describes a byte field, but is longer than 1!",field.descriptor);
                }
                let val = if field.attributes.len() != 0{
                    todo!("Field attributes not supported yet!"); //TODO:Support ConstatntValue attribute to allow for constant values!
                }
                else{0};
                Field::Byte(val)
            },
            'S'=>{
                if field.descriptor.len() != 1{
                    panic!("Invalid descriptor:\"{}\". It begins with 'S' which suggest that it describes a short field, but is longer than 1!",field.descriptor);
                }
                let val = if field.attributes.len() != 0{
                    todo!("Field attributes not supported yet!"); //TODO:Support ConstatntValue attribute to allow for constant values!
                }
                else{0};
                Field::Short(val)
            },
            'C'=>{
                if field.descriptor.len() != 1{
                    panic!("Invalid descriptor:\"{}\". It begins with 'C' which suggest that it describes a char field, but is longer than 1!",field.descriptor);
                }
                let val = if field.attributes.len() != 0{
                    todo!("Field attributes not supported yet!"); //TODO:Support ConstatntValue attribute to allow for constant values!
                }
                else{'\0'};
                Field::Char(val)
            },
            'I'=>{
                if field.descriptor.len() != 1{
                    panic!("Invalid descriptor:\"{}\". It begins with 'I' which suggest that it describes an integer field, but is longer than 1!",field.descriptor);
                }
                let val = if field.attributes.len() != 0{
                    todo!("Field attributes not supported yet!"); //TODO:Support ConstatntValue attribute to allow for constant values!
                }
                else{0};
                Field::Int(val)
            },
            'F'=>{
                if field.descriptor.len() != 1{
                    panic!("Invalid descriptor:\"{}\". It begins with 'F' which suggest that it describes an float field, but is longer than 1!",field.descriptor);
                }
                let val = if field.attributes.len() != 0{
                    todo!("Field attributes not supported yet!"); //TODO:Support ConstatntValue attribute to allow for constant values!
                }
                else{0.0};
                Field::Float(val)
            },
            'L'=>{
                let mut chars = desctiptor.collect::<Vec<char>>();
                chars.pop();
                let obj_name:String = chars.iter().collect();
                let obj_hsh = crate::class::get_hash(&obj_name);
                let val:Option<u64> = if field.attributes.len() != 0{
                    todo!("Field attributes on object references not supported yet!"); //TODO:Support ConstatntValue attribute to allow for constant values!
                }
                else{None};
                Field::Object(obj_hsh)
            },
            'Z'=>{
                if field.descriptor.len() != 1{
                    panic!("Invalid descriptor:\"{}\". It begins with 'Z' which suggest that it describes an bool field, but is longer than 1!",field.descriptor);
                }
                let val = if field.attributes.len() != 0{
                    todo!("Field attributes not supported yet!"); //TODO:Support ConstatntValue attribute to allow for constant values!
                }
                else{false};
                Field::Bool(val)
            },
            _=>panic!("unsuported field descrptor:{}",field.descriptor),
        };
        return (name,res);
    }
}
