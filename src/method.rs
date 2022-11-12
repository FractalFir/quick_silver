use crate::*;
use access_flags::*;
use java_class::*;
use constant_item::*;
use access_flags::*;
use attribute::*;
use std::fs::File;
use crate::code::Code;
#[derive(Debug)]
pub struct Method{
    access_flags:MethodAccessFlags,
    name:String,
    descriptor:String,
    attributes:Box<[Attribute]>,
}
fn map_java_name_to_cli_name<'a>(input:&'a str)->&'a str{
    match input{
        "<init>"=>".ctor",
        "<clinit>"=>".cctor",
        _=>input,
    }
}
impl Method{
    pub fn get_code<'a>(&'a self)->Option<&'a Code>{
        for attribute in self.attributes.iter(){
            match &attribute{
                Attribute::Code(c)=>return Some(c),
                _=>(),
            }
        }
        None
    }
     pub fn read(f:&mut File,constant_items:&[ConstantItem])->Method{
        let access_flags = MethodAccessFlags::from_u16(read_u16_be(f));
        let name = crate::constant_item::name_from_index(read_u16_be(f),constant_items);
        let descriptor = crate::constant_item::name_from_index(read_u16_be(f),constant_items);
        let attribute_count = read_u16_be(f);
        let attributes = read_attributes(attribute_count as usize,f,constant_items);
        Self{access_flags,name,descriptor,attributes}
     }
     pub(crate) fn write_to_asm<T:Write>(&self,file:&mut T,mappings:&TypeMappings)->std::io::Result<()>{
        let access = if self.access_flags.is_public(){"public"} else if  self.access_flags.is_private(){"private"} else if self.access_flags.is_protected(){"protected"} else {""};
        let sig = ("void","");
        let name = map_java_name_to_cli_name(&self.name);
         // TODO: handle more access flags
        let code = self.get_code().expect(&format!("Method {} must have code!",self.name));
         write!(file,"\t.method {access} {} {name}({}){{\n",sig.0,sig.1)?;
         write!(file,"\t.max_stack {}\n",code.max_stack)?;
         for (op,index) in code.code.iter(){
            op.write_to_asm(file,*index,mappings);
         }
         write!(file,"}}\n")
     }
}

