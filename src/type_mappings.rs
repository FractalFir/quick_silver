type VariableMappings = String;
type MethodMappings = String;
use serde::*;
use std::collections::HashMap;
#[derive(Serialize,Deserialize,Debug)]
enum TypeMappingsNode{
    Namespace(HashMap<String,TypeMappingsNode>),
    Class(String,HashMap<String,VariableMappings>,HashMap<String,HashMap<String,MethodMappings>>),
}
impl TypeMappingsNode{
    fn map_class(&self,path:&mut std::str::Split<char>)->Option<String>{
        match path.next(){
            Some(next)=>{
                match self{
                    Self::Namespace(space)=>{
                        space.get(next.into())?.map_class(path)
                    },
                    _=>return None,
                }
            },
            None=>{
                match self{
                    Self::Namespace(_)=>panic!("Invalid class path in type Mappings. Expected item at path to be a class, but it was a namespace."),
                    Self::Class(Mappings,_,_)=>return Some(Mappings.to_owned()),
                }
            }
        }
    }
    fn map_method(&self,path:&mut std::str::Split<char>,function_name:&str,function_sig:&str)->Option<String>{
        match path.next(){
            Some(next)=>{
                match self{
                    Self::Namespace(space)=>{
                        space.get(next.into())?.map_method(path,function_name,function_sig)
                    },
                    _=>return None,
                }
            },
            None=>{
                match self{
                    Self::Namespace(_)=>panic!("Invalid class path in type mappings. Expected item at path to be a class, but it was a namespace."),
                    Self::Class(_,_,mappings)=>{
                        let method = mappings.get(function_name.into())?;
                        let signature_match = method.get(function_sig.into())?;
                        return Some(signature_match.to_owned());
                    },
                }
            }
        }
    }
    fn map_field(&self,path:&mut std::str::Split<char>,function_name:&str,function_sig:&str)->Option<(String,String)>{
        match path.next(){
            Some(next)=>{
                match self{
                    Self::Namespace(space)=>{
                        space.get(next.into())?.map_field(path,function_name,function_sig)
                    },
                    _=>return None,
                }
            },
            None=>{
                match self{
                    Self::Namespace(_)=>panic!("Invalid class path in type mappings. Expected item at path to be a class, but it was a namespace."),
                    Self::Class(_,mappings,_)=>{
                        todo!();
                    },
                }
            }
        }
    }
}
use serde_json::Value;
#[derive(Serialize,Deserialize,Debug)]
pub struct TypeMappings(HashMap<String,TypeMappingsNode>);
impl TypeMappings{
    pub fn from_file(path:&str)->Self{
        let s = std::fs::read_to_string(path).expect("Could not load JSON file containing type Mappingss!");
        serde_json::from_str(&s).expect("Could not deserialize JSON file containing type Mappingss!")
    }
    pub fn map_class(&self,class_path:&str)->String{
        let mut path = class_path.split('/');
        let root = path.next().expect("Empty class path!");
        match self.0.get(root.into()){
            Some(node)=>{
                match node.map_class(&mut path){
                    Some(res)=>return res,
                    None=>{
                        let mut path = class_path.split('/');
                        let mut res = String::new();
                        res.push_str(root);
                        path.next();
                        for part in path{
                            res.push('.');
                            res.push_str(part);
                        }
                        println!("WARNIG: Type Mappings contain a mapping for the namespace class at path \"{class_path}\" is in, but do not contain any Mappings for this class. This is usualy means that this java class is not supported and may lead to final CLI code not being able to compile. Assumed path:\"{res}\".\n");
                        return res;
                    },
                }
            },
            None=>{
                let mut res = String::new();
                res.push_str(root);
                for part in path{
                    res.push('.');
                    res.push_str(part);
                }
                return res;
            }
        }
        todo!("class_path:{class_path:?}");
    }
    pub fn map_method(&self,class_path:&str,function_name:&str,function_sig:&str)->String{
        let mut path = class_path.split('/');
        let root = path.next().expect("Empty class path!");
        match self.0.get(root.into()){
            Some(node)=>{
                match node.map_method(&mut path,function_name,function_sig){
                    Some(res)=>return res,
                    None=>{
                        //let res = convert_function_names(class_path,function_name,function_sig);
                        //println!("WARNIG: Type Mappings contain a mapping for the namespace parrent class of method at path \"{class_path}\" is in, but do not contain any mapping for this method. This is usualy means that this java standard libray method is not supported and may lead to final CLI code not being able to compile. Assumed path:\"{res}\".\n");
                        todo!();
                        //return res;
                    },
                }
            },
            None=>{
                let class = self.map_class(class_path);
                let sig = map_fn_sig(function_sig,self);
                println!("class:{class}");
                let function_name = convert_special_function_names(function_name);
                format!(" {} {class}::{function_name}({})",sig.0,sig.1)
            }
        }
    }
    pub fn map_field(&self,class_path:&str,field_name:&str,field_desc:&str)->(String,String){
        let mut path = class_path.split('/');
        let root = path.next().expect("Empty class path!");
        match self.0.get(root.into()){
            Some(node)=>{
                match node.map_field(&mut path,field_name,field_desc){
                    Some(res)=>return res,
                    None=>{
                        let res = (class_path,field_name,field_desc);
                        println!("WARNIG: Type Mappings contain a mapping for the namespace parrent class of method at path \"{class_path}\" is in, but do not contain any mapping for this method. This is usualy means that this java standard libray method is not supported and may lead to final CLI code not being able to compile. Assumed path:\"{res:?}\".\n");
                        todo!();
                    },
                }
            },
            None=>{
               let tpe = crate::field::field_descriptor_to_cli_name(field_desc,self);
               let mut field_path = self.map_class(class_path);
               field_path.push_str("::");
               field_path.push_str(field_name);
               println!("tpe:{tpe} field_path:{field_path}");
               return (tpe,field_path);
            }
        }
    }
}
fn map_fn_sig(sig:&str,mappings:&TypeMappings)->(String,String){
    let mut index = 0;
    for c in sig.chars(){
        if c == ')'{break};
        index+=1;
    }
    let (arg,ret) = sig.split_at(index);
    let (_,arg) = arg.split_at(1);
    let mut res = String::with_capacity(arg.len());
    let mut arg = arg.split(',');
    let first = arg.next();
    match first{
        Some(first)=>if(first != ""){
            let first = crate::field::field_descriptor_to_cli_name(first,mappings);
            res.push_str(&first);
        },
        None=>(),
    };
    for s in arg{
        res.push(',');
        let s = crate::field::field_descriptor_to_cli_name(s,mappings);
        res.push_str(&s);
    }
    let (_,ret) = ret.split_at(1);
    let ret = crate::field::field_descriptor_to_cli_name(ret,mappings);
    (ret,res)
}
fn convert_special_function_names<'a>(name:&'a str)->&'a str{
    match name{
        "<init>"=>".ctor",
        _=>name,
    }
}
