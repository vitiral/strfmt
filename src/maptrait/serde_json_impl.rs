use crate::Map; 
extern crate serde_json; 
use self::serde_json::{ Value, Map as JsonMap};
use crate::DisplayStr;

impl Map<String, Value> for JsonMap<String, Value>{
    fn get(&self, key : &String) -> Option<&Value> {
        JsonMap::get(self, key)
    }
}


///This implementation uses the serde_json standards 
///If index is invalid, return serde_json::Value::Null
///However, if type is not indexable, raise Error 
impl Map<String, Value> for Value {

    fn get(&self, key : &String) -> Option<&Value> {
        
        if !(self.is_array() || self.is_object()){
            return None;
        }        

        if self.is_array(){
            let key : Result<usize, _> = key.parse();
            if key.is_err(){return None; }
            return self.get(key.unwrap()); 
        }    

        self.get(key)
    }

    fn get_owned(&self, _key : &String) -> Option<Value>{
        
        if self.is_array() || self.is_object(){
            return Some(serde_json::Value::Null);  
        }   

        return None;    
    }

}



fn fmt_serde_json_number(n : &serde_json::Number, f : &mut crate::Formatter) -> crate::Result<()> {

    if let Some(v) = n.as_f64(){
        f.f64(v)
    }

    else if let Some(v) = n.as_u64(){
        f.u64(v)
    }

    else if let Some(v) = n.as_i64(){
        f.i64(v)
    }

    else {
        unreachable!("Serde Json Number is not u64, i64 or f64, nothing left to do")
    }
}


impl DisplayStr for Value{
    fn display_str(&self, f: &mut crate::Formatter) -> crate::Result<()> {
        match self {
            Value::Bool(b) => b.display_str(f),
            Value::Null => "null".display_str(f),
            Value::Number(n) => fmt_serde_json_number(n, f),
            Value::String(s) => s.display_str(f),
            Value::Array(a) => format!("{:?}" , a).display_str(f),
            Value::Object(a) => format!("{:?}", a).display_str(f),
        }
    }
}
