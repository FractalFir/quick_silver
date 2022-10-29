use inkwell::types::FunctionType;
use crate::preprocessing::code::OpCode;
enum MethodImpl<'a>{
    JavaCode(Box<[(u32,OpCode)]>),
    LLVMMethod(FunctionType<'a>),
}
struct Method<'a>{
    args:Box<[u64]>,
    ret:u64,
    code:MethodImpl<'a>,
}
struct Varaibles{
    
}
struct CodeBlock{

}
impl<'a> Method<'a>{
    pub(crate) fn from_llvm(args:Box<[u64]>,ret:u64,llvm_fnc:FunctionType<'a>)->Self{
        Self{args,ret,code:MethodImpl::LLVMMethod(llvm_fnc)}
    }
}
