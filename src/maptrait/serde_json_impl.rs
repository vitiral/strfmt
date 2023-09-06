use crate::Map; 
extern crate serde_json; 
use self::serde_json::{ Value, Map as JsonMap};
use crate::DisplayStr; 

impl Map<String, Value> for JsonMap<String, Value>{
    fn get(&self, key : &String) -> Option<&Value> {
        JsonMap::get(self, key)
    }
}


impl DisplayStr for Value{
    fn display_str(&self, f: &mut crate::Formatter) -> crate::Result<()> {
        match self {
            Value::Bool(b) => b.display_str(f),
            Value::Null => "null".display_str(f),
            Value::Number(n) => n.to_string().display_str(f),
            Value::String(s) => s.display_str(f),
            Value::Array(a) => format!("{:?}" , a).display_str(f),
            Value::Object(a) => format!("{:?}", a).display_str(f),
        }
    }
}
