use std::fs::File;
use crate::*;
use access_flags::*;
use constant_item::*;
use access_flags::*;
use attribute::*;
#[derive(Debug)]
pub struct Field{
    pub access_flags:FieldAccessFlags,
    pub name:String,
    pub descriptor:String,
    pub attributes:Box<[Attribute]>,
}
impl Field{
     pub fn read(f:&mut File,constant_items:&[ConstantItem])->Field{
        let access_flags = FieldAccessFlags::from_u16(read_u16_be(f));
        let name = crate::constant_item::name_from_index(read_u16_be(f),constant_items);
        let descriptor = crate::constant_item::name_from_index(read_u16_be(f),constant_items);
        let attribute_count = read_u16_be(f);
        let attributes = read_attributes(attribute_count as usize,f,constant_items);
        Self{access_flags,name,descriptor,attributes}
     }
     pub fn is_static(&self)->bool{
        self.access_flags.is_static()
     }
     pub(crate) fn write_to_asm<T:Write>(&self,file:&mut T,mappings:&TypeMappings)->std::io::Result<()>{
        let access = if self.access_flags.is_public(){"public"} else {""}; //TODO: support all field attributes
        let r#type = descriptor_to_cli_name(&self.descriptor,mappings);
        let name = &self.name;
        writeln!(file,"\t.field {access} {type} {name}")
     }
}
fn descriptor_to_cli_name(desc:&str,mappings:&TypeMappings)->String{
    let mut chars = desc.chars();
    match chars.nth(0).expect("Filed type descriptor can't be less than 1 charcters!"){
        'B'=>"int8".to_owned(),
        'S'=>"int16".to_owned(),
        'C'=>"char".to_owned(),
        'I'=>"int32".to_owned(),
        'F'=>"float32".to_owned(),
        'Z'=>"bool".to_owned(),
        'L'=>{
            chars.next_back(); //remove ; from the end of class name
            format!("class {}",mappings.map_class(chars.as_str()))
        },
        _=>todo!("unhandled descriptor:{desc}"),
    }
}
