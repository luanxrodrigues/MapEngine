use crate::thing::{self, Thing,ThingType, ThingTransform};
pub struct Map {
    pub name: String,
    width: u32,
    height: u32,
    pub things:Vec<thing::Thing>,
    next_id : u64
}

impl Map{
    pub fn new(size:(u32,u32),name:String) -> Self{
        Self{
            width:size.0,
            height:size.1,
            name:name,
            things:Vec::new(),
            next_id:0
        }
    }

    pub fn add_thing(&mut self, thing_transform:ThingTransform,thing_type:ThingType) -> u64
    {
        let id = self.next_id;
        self.next_id += 1;
        let added : Thing = Thing::new(thing_transform,thing_type,id);

        self.things.push(added);

        id
    }
}