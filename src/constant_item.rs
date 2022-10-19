use crate::*;
#[derive(Debug)]
pub enum ConstantItem{
    MethodRef(u16,u16),
    FieldRef(u16,u16),
    Class(u16),
    NameAndType(u16,u16),
    UTF8(String),
    String(u16),
}
impl ConstantItem{
    pub fn read(f:&mut File)->ConstantItem{
        let const_type = read_u8(f);
        match const_type{
            10=>{
                let class_index = read_u16_be(f);
                let name_and_type_index = read_u16_be(f);
                ConstantItem::MethodRef(class_index,name_and_type_index)
            },
            7=>{
                let name_and_type_index = read_u16_be(f);
                ConstantItem::Class(name_and_type_index)
            },
            12=>{
                let name_index = read_u16_be(f);
                let type_index = read_u16_be(f);
                ConstantItem::NameAndType(name_index,type_index)
            },
            1=>{
                let len = read_u16_be(f);
                let mut bytes = vec![0;len as usize];
                f.read(&mut bytes);
                ConstantItem::UTF8(
                    std::str::from_utf8(&bytes).expect("UTF8 constant pool item not a valid UTF8 string").to_owned()
                )
            },
            9=>{
                let class_index = read_u16_be(f);
                let name_and_type_index = read_u16_be(f);
                ConstantItem::FieldRef(class_index,name_and_type_index)
            },
            8=>{
                let string_index = read_u16_be(f);
                ConstantItem::String(string_index)
            }
            _=>panic!("Unsuported Const Item Type '{const_type}'"),
        }
    }
}
pub fn read_constant_item_pool(f:&mut File)->Box<[ConstantItem]>{
    let len = read_u16_be(f) - 1;
    let mut pool = Vec::with_capacity(len as usize);
    for _ in 0..len{
        pool.push(ConstantItem::read(f));
    }
    return pool.into();
}

