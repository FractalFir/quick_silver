use crate::*;
#[derive(Debug)]
pub struct JavaClass{
    minor_version:u16,
    major_version:u16,
    items:Box<[ConstantItem]>,
    access_flags:ClassAccessFlags,
    this_class:u16,
    super_class:u16,
    interfaces:Box<[u16]>,
    fields:Box<[FieldInfo]>,
}
impl JavaClass{
    pub fn from_file(f:&mut File)->Option<Self>{
        let magic:u32 = read_u32_be(f);
        if magic != 0xCAFEBABE{
            return None;
        }
        let major_version = read_u16_be(f);
        let minor_version = read_u16_be(f);
        let items = read_constant_item_pool(f);
        let access_flags = ClassAccessFlags::from_u16(read_u16_be(f));
        let this_class = read_u16_be(f);
        let super_class = read_u16_be(f);
        let interfaces_len = read_u16_be(f);
        let mut interfaces = Vec::with_capacity(interfaces_len as usize);
        for _ in 0..interfaces_len{
            interfaces.push(read_u16_be(f));
        }
        let interfaces:Box<[u16]> = interfaces.into();
        let fields_len = read_u16_be(f);
        let mut fields = Vec::with_capacity(fields_len as usize);
        for _ in 0..fields_len{
            fields.push(FieldInfo::read(f));
        }
        let fields:Box<[FieldInfo]> = fields.into();
        return Some(Self{major_version,minor_version,items,access_flags,this_class,super_class,interfaces,fields});
    }
}
