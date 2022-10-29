use inkwell::{context::Context,module::Module,types::{IntType,BasicMetadataTypeEnum},AddressSpace,values::FunctionValue};
use crate::JavaClass;
use crate::class::Class;
pub struct QuickSilverContext<'a>{
    llvm_context:&'a Context,
    module:Module<'a>,
    heap_alloc_fnc:FunctionValue<'a>,
}
impl<'a> QuickSilverContext<'a>{
    fn insert_core<'b>(md:&Module<'b>,usize_type:IntType<'b>)->(FunctionValue<'b>){
        let alloc_fn_type = usize_type.ptr_type(AddressSpace::Global).fn_type(&[BasicMetadataTypeEnum::IntType(usize_type)],false);
        let heap_alloc_fnc = md.add_function("heap_alloc",alloc_fn_type,None);
        let stack_alloc_fnc = md.add_function("stack_alloc",alloc_fn_type,None);
        (heap_alloc_fnc)
    }
    fn insert_std(&self){
    
    }
    pub fn create_default_llvm_context()->Context{
       Context::create()
    }
    pub fn create(classes:&[JavaClass],ctx:&'a Context)->Self{
        let md = ctx.create_module("quicksilver_mod");
        
        let usize_type = ctx.i64_type(); // TODO: Add 32 bit support
        
        let core_fncs = Self::insert_core(&md,usize_type);
        
        md.verify().expect("Can't verify llvm module after inserting core functions!");
        println!("Module:{}",&md.to_string());
        let mut res = Vec::with_capacity(classes.len());
        for class in classes{
            res.push(Class::get_from_java_class(class,ctx));
        }
        Self{llvm_context:ctx,module:md,heap_alloc_fnc:core_fncs}
    }
    pub fn get_context_ref(&'a self)->&'a Context{
        self.llvm_context
    }
}
