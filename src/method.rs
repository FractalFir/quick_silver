use crate::access_flags::*;
use crate::*;
use java_class::*;
use constant_item::*;
use access_flags::*;
use attribute::*;
#[derive(Debug)]
pub struct Method{
    access_flags:MethodAccessFlags,
    name:String,
    descriptor:String,
    attributes:Box<[Attribute]>,
}
impl Method{
     pub fn read(f:&mut File,constant_items:&[ConstantItem])->Method{
        let access_flags = MethodAccessFlags::from_u16(read_u16_be(f));
        let name = crate::java_class::name_from_index(read_u16_be(f),constant_items);
        let descriptor = crate::java_class::name_from_index(read_u16_be(f),constant_items);
        let attribute_count = read_u16_be(f);
        let attributes = read_attributes(attribute_count as usize,f,constant_items);
        Self{access_flags,name,descriptor,attributes}
     }
}

