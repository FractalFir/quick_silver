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
                        let res = convert_function_names(class_path,function_name,function_sig);
                        println!("WARNIG: Type Mappings contain a mapping for the namespace parrent class of method at path \"{class_path}\" is in, but do not contain any mapping for this method. This is usualy means that this java standard libray method is not supported and may lead to final CLI code not being able to compile. Assumed path:\"{res}\".\n");
                        todo!();
                        return res;
                    },
                }
            },
            None=>{
               todo!();
            }
        }
    }
}
fn convert_function_names(class_path:&str,name:&str,sig:&str)->String{
    todo!();
}
