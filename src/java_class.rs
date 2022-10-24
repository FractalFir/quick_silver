use crate::*;
#[derive(Debug)]
pub struct JavaClass{
    minor_version:u16,
    major_version:u16,
    items:Box<[ConstantItem]>,
    access_flags:ClassAccessFlags,
    this_class:u16,
    super_class:u16,
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
        return Some(Self{major_version,minor_version,items,access_flags,this_class,super_class});
    }
}
