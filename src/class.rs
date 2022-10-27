use crate::preprocessing::method::Method;
use crate::preprocessing::field::Field;
use inkwell::types::StructType;
use inkwell::context::ContextRef;
struct Class{
    parrent:u64,
    this:u64,
    fields:Box<[Field]>,
    methods:Box<[Method]>,
}
impl Class{
    pub(crate) fn get_llvm_type<'a>(&mut self,ctx:ContextRef<'a>)->StructType<'a>{
       if self.this != self.parrent{
            todo!("Derived Objects not supported yet!");
       }
        todo!("Class type creation not supported yet!");
    }
}
