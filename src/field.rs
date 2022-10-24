use crate::access_flags::*;
use crate::*;
use java_class::*;
#[derive(Debug)]
pub struct Field{
    access:FieldAccessFlags,
    name:String,
    descriptor:String,
}
impl Field{
     pub fn read(f:&mut File,constant_items:&[ConstantItem])->Field{
        let access = FieldAccessFlags::from_u16(read_u16_be(f));
        let name = crate::java_class::name_from_index(read_u16_be(f),constant_items);
        let descriptor = crate::java_class::name_from_index(read_u16_be(f),constant_items);
        let attribute_count = read_u16_be(f);
        for i in 0..attribute_count{
            todo!();
        }
        Self{access,name,descriptor}
     }
}
