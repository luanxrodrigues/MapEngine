#![allow(unused_variables,unused)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod map;
mod thing;
use thing::{Thing,ThingTransform,ThingType};

fn main() {
    mapengine_lib::run()
}
fn create_test_map() -> map::Map{
    let mut map = map::Map::new((32,32),"test".to_string(),"0.0.1".to_string());

    map
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_map() {
        let mut map = create_test_map();
        assert_eq!(map.width,32);
    }

    #[test]
    fn encode1() {
        let mut map: map::Map = create_test_map();
        map.add_thing(ThingTransform { 
            position: (0.0,0.0),
             rotation: (0.0,0.0,0.0),
              scale: 1.0,
               depth: 1 
            }, ThingType::Object,0, "Mesa".to_string());
        map.add_thing(ThingTransform { 
            position: (0.0,0.0),
             rotation: (0.0,0.0,0.0),
              scale: 1.0,
               depth: 1 
            }, ThingType::Object,0, "Mesa".to_string());
        map.add_thing(ThingTransform { 
            position: (0.0,0.0),
             rotation: (0.0,0.0,0.0),
              scale: 1.0,
               depth: 1 
            }, ThingType::Object,0, "Mesa".to_string());
        map.add_thing(ThingTransform { 
            position: (0.0,0.0),
             rotation: (0.0,0.0,0.0),
              scale: 1.0,
               depth: 1 
            }, ThingType::Object,0, "Mesa".to_string());
        map.add_thing(ThingTransform { 
            position: (0.0,0.0),
             rotation: (0.0,0.0,0.0),
              scale: 1.0,
               depth: 1 
            }, ThingType::Object,0, "Mesa".to_string());
        let _  = map::Map::encode_map(&map, "test.rmap".to_string());

    }
    #[test]
    fn encode() {
        let mut map: map::Map = create_test_map();
        map.add_thing(ThingTransform { 
            position: (1.0,1.0),
             rotation: (20.0,30.0,15.0),
              scale: 1.0,
               depth: 1 
            }, ThingType::Object,0, "Mesa".to_string());
        map.add_thing(ThingTransform { 
            position: (0.0,0.0),
             rotation: (0.0,0.0,0.0),
              scale: 1.0,
               depth: 1 
            }, ThingType::Object,0, "Mesa".to_string());
        map.add_thing(ThingTransform { 
            position: (0.0,0.0),
             rotation: (0.0,0.0,0.0),
              scale: 1.0,
               depth: 1 
            }, ThingType::Object,0, "Mesa".to_string());
        map.add_thing(ThingTransform { 
            position: (0.0,0.0),
             rotation: (0.0,0.0,0.0),
              scale: 1.0,
               depth: 1 
            }, ThingType::Object,0, "Mesa".to_string());
        map.add_thing(ThingTransform { 
            position: (0.0,0.0),
             rotation: (0.0,0.0,0.0),
              scale: 1.0,
               depth: 1 
            }, ThingType::Object,0, "Mesa".to_string());
        let _  = map::Map::encode_map(&map, "test2.rmap".to_string());
        
        
    }

}