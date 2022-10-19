pub struct ClassAccessFlags{
    flags:u16,
}
pub struct FieldAccessFlags{
    flags:u16,
}
impl ClassAccessFlags{
    pub fn from_u16(flags:u16)->ClassAccessFlags{
        ClassAccessFlags{flags}
    }
    pub fn is_public(&self)->bool{
        (self.flags & 0x1) != 0
    }
    pub fn is_final(&self)->bool{
        (self.flags & 0x10) != 0
    }
    pub fn is_supper(&self)->bool{
        (self.flags & 0x20) != 0
    }
    pub fn is_interface(&self)->bool{
        (self.flags & 0x200) != 0
    }
    pub fn is_abstract(&self)->bool{
        (self.flags & 0x400) != 0
    }
    pub fn is_syntetic(&self)->bool{
        (self.flags & 0x1000) != 0
    }
    pub fn is_annotation(&self)->bool{
        (self.flags & 0x2000) != 0
    }
    pub fn is_enum(&self)->bool{
        (self.flags & 0x4000) != 0
    }
}
impl FieldAccessFlags{
    pub fn from_u16(flags:u16)->Self{
        Self{flags}
    }
    pub fn is_public(&self)->bool{
        (self.flags & 0x1) != 0
    }
    pub fn is_private(&self)->bool{
        (self.flags & 0x2) != 0
    }
    pub fn is_protected(&self)->bool{
        (self.flags & 0x4) != 0
    }
    pub fn is_static(&self)->bool{
        (self.flags & 0x8) != 0
    }
    pub fn is_final(&self)->bool{
        (self.flags & 0x10) != 0
    }
    pub fn is_volatile(&self)->bool{
        (self.flags & 0x40) != 0
    }
    pub fn is_transistent(&self)->bool{
        (self.flags & 0x80) != 0
    }
    pub fn is_synthetic(&self)->bool{
        (self.flags & 0x1000) != 0
    }
    pub fn is_enum(&self)->bool{
        (self.flags & 0x4000) != 0
    }
}
impl std::fmt::Display for FieldAccessFlags{
    fn fmt(&self,f:&mut std::fmt::Formatter)->Result<(), std::fmt::Error>{
        let public = self.is_public();
        let private = self.is_private();
        let protected = self.is_protected();
        let r#static = self.is_static();
        let r#final = self.is_final();
        let volatile = self.is_volatile();
        let transistent = self.is_transistent();
        let synthetic = self.is_synthetic();
        let r#enum = self.is_enum();
        write!(f,"{{public:{public}, private:{private}, protected:{protected}, static:{static}, final:{final}, volatile:{volatile}, transistent:{transistent}, synthetic:{synthetic}}}")
    }
}
impl std::fmt::Display for ClassAccessFlags{
    fn fmt(&self,f:&mut std::fmt::Formatter)->Result<(), std::fmt::Error>{
        let public = self.is_public();
        let r#final = self.is_final();
        let supper = self.is_supper();
        let interface = self.is_interface();
        let r#abstract = self.is_abstract();
        let syntetic = self.is_syntetic();
        let annotation = self.is_annotation();
        let r#enum = self.is_enum();
        write!(f,"{{public:{public}, final:{final}, supper:{supper}, interface:{interface}, abstract:{abstract}, syntetic:{syntetic}, annotation:{annotation}, enum:{enum}}}")
    }
}

use std::fmt::Debug;
impl Debug for ClassAccessFlags{
    fn fmt(&self,f:&mut std::fmt::Formatter)->Result<(), std::fmt::Error>{
        <Self as std::fmt::Display>::fmt(self,f)
    }
}
impl Debug for FieldAccessFlags{
    fn fmt(&self,f:&mut std::fmt::Formatter)->Result<(), std::fmt::Error>{
        <Self as std::fmt::Display>::fmt(self,f)
    }
}
