extern crate hashbrown; 
use crate::{Map, DisplayStr};
use std::hash::Hash; 
use std::str::FromStr; 

impl<K,V> Map<K, V> for hashbrown::HashMap<K, V> where K: Hash + Eq + FromStr , V : DisplayStr{
    fn get(&self, key : &K) -> Option<&V>{
        hashbrown::HashMap::get(self, key)
    }
}
