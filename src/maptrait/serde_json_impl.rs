use crate::Map; 
extern crate serde_json; 
use self::serde_json::{ Value, Map as JsonMap};
use crate::DisplayStr; 

impl Map<String, Value> for JsonMap<String, Value>{
    fn get(&self, key : &String) -> Option<&Value> {
        JsonMap::get(self, key)
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
