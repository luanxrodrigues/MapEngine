


pub struct Thing {
    pub transform:ThingTransform,
    pub _type:ThingType,
    pub bounding_box:(f32,f32),
    id:u64,
    pub layer_id : u16
}
pub struct ThingTransform{
    position:(f32,f32),
    rotation:(f32,f32,f32),
    scale:f32,
    depth:u32
}
pub enum ThingType{
    Object,
    Lighning,
    Empty,
    Area,
}

impl Thing{
    pub fn new(transform:ThingTransform,_type: ThingType,id:u64,layer_id:u16) -> Self{
        Self{
            _type:_type,
            transform:transform,
            bounding_box:(0.0,0.0),
            id:id,
            layer_id:layer_id
        }
    }
    pub fn get_id(&self) -> u64{
        self.id
    }


    pub fn set_rotation(&mut self,rotation : (f32,f32,f32))
    {
        self.transform.rotation = rotation
    }
    pub fn set_scale(&mut self, new_scale:f32) -> &Thing
    {

        self.transform.scale = new_scale;
        
        self
    }
    pub fn set_position(&mut self,new_position:(f32,f32)) -> &Thing
    {
        self.transform.position = new_position;
        self
    }
    pub fn set_depth(&mut self, new_depth:u32) -> &Thing{
        self.transform.depth = new_depth;

        self
    }
    
}