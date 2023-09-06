use crate::DisplayStr; 
use std::str::FromStr; 
use std::hash::Hash; 
use std::collections::HashMap; 

pub trait Map<K , V> where K: Hash + Eq + FromStr, V: DisplayStr{
    
    ///Prioritized first 
    fn get(&self, key : &K) -> Option<&V>; 


    ///Returned an owned item instead of a reference 
    ///Prioritized second
    #[allow(unused_variables)]
    fn get_owned(&self, key : &K) -> Option<V>{
        None //This is done because HashMap and serde_json Map can both return references  
    }
}

impl<K,V> Map<K,V> for HashMap<K,V> where K: Hash + Eq + FromStr, V: DisplayStr {
    fn get(&self, key : &K) -> Option<&V>{
        HashMap::get(self, key)
    }
}

impl  Map<String, String> for std::env::Vars {
    fn get(&self, _key : &String) -> Option<&String>{None}
    fn get_owned(&self, key : &String) -> Option<String>{
        std::env::var(key).ok()
    }
}

mod serde_json_impl; 