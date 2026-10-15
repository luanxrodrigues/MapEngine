use crate::thing::{self, Thing,ThingType, ThingTransform};
pub struct Map {
    pub name: String,
    width: u32,
    height: u32,
    pub things:Vec<thing::Thing>,
    next_id : u64,
    next_layer_id:u16,
    layers:Vec<Layer>
}
pub struct Layer {
    pub id:u16,
    pub name:String
}

impl Map{
    pub fn new(size:(u32,u32),name:String) -> Self{
        Self{
            width:size.0,
            height:size.1,
            name:name,
            things:Vec::new(),
            next_id:0,
            next_layer_id:0,
            layers:Vec::new()
        }
    }

    pub fn add_thing(&mut self, thing_transform:ThingTransform,thing_type:ThingType,thing_layer:Layer) -> u64
    {
        let id = self.next_id;
        self.next_id += 1;
        let added : Thing = Thing::new(thing_transform,thing_type,id,thing_layer.id);

        self.things.push(added);

        id
    }

    pub fn create_layer(&mut self,layerName:String) -> u16{
        let id = self.next_layer_id;
        self.next_layer_id += 1;
        let newLayer = Layer{
            id:id,
            name:layerName
        };

        id
    }
}