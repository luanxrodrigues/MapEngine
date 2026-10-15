pub struct Thing {
    x:f32,
    y:f32,
    rotation:(u16,u16,u16),
    scale:u8,
    _type:ThingType
}
pub enum ThingType{
    Object,
    Lighning,
    Empty,
}
