use crate::File;
use crate::preprocessing::*;
use field::Field;
use method::Method;
use constant_item::*;
use access_flags::*;
#[derive(Debug)]
pub struct JavaClass{
    minor_version:u16,
    major_version:u16,
    pub items:Box<[ConstantItem]>,
    pub access_flags:ClassAccessFlags,
    pub this_class:String,
    pub super_class:String,
    pub interfaces:Box<[String]>,
    pub fields:Box<[Field]>,
    pub methods:Box<[Method]>,
}
impl JavaClass{
    fn read_interafeces(interface_count:usize,constant_items:&[ConstantItem],f:&mut File)->Box<[String]>{
        let mut res = Vec::with_capacity(interface_count);
        for i in 0..interface_count{
            res.push(class_name_from_index(read_u16_be(f),constant_items));
        }
        let res:Box<[String]> = res.into();
        res
    }
    fn read_methods(method_count:usize,constant_items:&[ConstantItem],f:&mut File)->Box<[Method]>{
        let mut res = Vec::with_capacity(method_count);
        for i in 0..method_count{
            res.push(Method::read(f,constant_items));
        }
        let res:Box<[Method]> = res.into();
        res
    }
    fn read_fields(field_count:usize,constant_items:&[ConstantItem],f:&mut File)->Box<[Field]>{
        let mut res = Vec::with_capacity(field_count);
        for i in 0..field_count{
            res.push(Field::read(f,constant_items));
        }
        let res:Box<[Field]> = res.into();
        res
    }
    pub fn in_directory(path:&str)->Box<[Self]>{
        let paths = std::fs::read_dir(path).expect("Could not get directory {path}");
        let mut res = Vec::new();
        for path in paths {
            let mut f:File  = File::open(path.unwrap().path()).expect("Could not open file!");
            res.push(Self::from_file(&mut f).expect("Not a vaild Java class!"));
        }
        res.into()
    }
    pub fn from_file(f:&mut File)->Option<Self>{
        let magic:u32 = read_u32_be(f);
        if magic != 0xCAFEBABE{
            return None;
        }
        let minor_version = read_u16_be(f);
        let major_version = read_u16_be(f);
        if major_version != 63{
            println!("WARNING: Unsupported class file major version:({major_version})!");
        }
        let items = read_constant_item_pool(f);
        let access_flags = ClassAccessFlags::from_u16(read_u16_be(f));
        let this_class = constant_item::class_name_from_index(read_u16_be(f),&items);
        let super_class = constant_item::class_name_from_index(read_u16_be(f),&items);
        let interface_count = read_u16_be(f);
        let interfaces = Self::read_interafeces(interface_count as usize,&items,f);
        let field_count = read_u16_be(f);
        let fields = Self::read_fields(field_count as usize,&items,f);
        let method_count = read_u16_be(f);
        let methods = Self::read_methods(method_count as usize,&items,f);
        println!("field_count:{field_count}");
        return Some(Self{major_version,minor_version,items,access_flags,this_class,super_class,interfaces,fields,methods});
    }
}
