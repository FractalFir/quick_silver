use crate::preprocessing::{method::Method,field::Field as PField};
use inkwell::{types::StructType,context::Context};
use crate::field::Field;
use crate::JavaClass;
pub struct Class<'a>{
    parrent:u64,
    this:u64,
    fields:Box<[Field]>,
    static_fields:Box<[Field]>,
    
    static_methods:Box<[Method]>,
    methods:Box<[Method]>,
    virtual_methods:Box<[Method]>,
    
    llvm_type:Option<StructType<'a>>
}
impl<'a> Class<'a>{
    pub(crate) fn get_llvm_type(&mut self,ctx:&'a Context)->StructType<'a>{
        if self.this != self.parrent{
            todo!("Derived Objects not supported yet!");
        }
        todo!("Class type creation not supported yet!");
    }
    pub (crate) fn get_from_java_class(jc:&JavaClass,ctx:&'a Context)->Self{
        let parrent = get_hash(&jc.super_class);
        let this = get_hash(&jc.this_class);
        let mut fields:Vec<(String,Field)> = Vec::with_capacity(jc.fields.len());
        let mut static_fields:Vec<(String,Field)> = Vec::with_capacity(jc.fields.len());
        for field in jc.fields.iter(){
            let llvm_field = Field::from_java_field(field);
            if field.is_static(){
                static_fields.push(llvm_field);
            }
            else {
                fields.push(llvm_field);
            }
        }
        todo!();
    }
}
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
pub fn get_hash<T:Hash>(t:&T)->u64{
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}
