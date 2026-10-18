#![allow(unused_variables,unused)]

use std::io::{self, Write};

pub const THING_NAME_LENGTH: usize = 32;

pub struct Thing {
    pub transform:ThingTransform,
    pub _type:ThingType,
    pub bounding_box:(f32,f32),
    id:u32,
    pub layer_id : u16,
    pub name : String
}
pub struct ThingTransform{
    pub position:(f32,f32),
    pub rotation:(f32,f32,f32),
    pub scale:f32,
    pub depth:u32
}
pub enum ThingType{
    Object,
    Lighning,
    Empty,
    Area,
}

impl ThingType {
    fn to_byte(&self) -> u8 {
        match self {
            Self::Object => 0,
            Self::Lighning => 1,
            Self::Empty => 2,
            Self::Area => 3,
        }
    }
}

impl Thing{
    pub fn new(transform:ThingTransform,_type: ThingType,id:u32,layer_id:u16,name:String) -> Self{
        Self{
            _type:_type,
            transform:transform,
            bounding_box:(0.0,0.0),
            id:id,
            layer_id:layer_id,
            name:name
        }
    }
    pub fn get_id(&self) -> u32{
        self.id
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let name = self.name.as_bytes();
        if name.len() > THING_NAME_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "thing name exceeds 32 bytes",
            ));
        }

        let mut name_bytes = [0u8; THING_NAME_LENGTH];
        name_bytes[..name.len()].copy_from_slice(name);

        writer.write_all(b"[T]")?;
        writer.write_all(&name_bytes)?;
        writer.write_all(&self.id.to_le_bytes())?;
        writer.write_all(&self.layer_id.to_le_bytes())?;
        writer.write_all(&[self._type.to_byte()])?;
        writer.write_all(&self.bounding_box.0.to_le_bytes())?;
        writer.write_all(&self.bounding_box.1.to_le_bytes())?;
        writer.write_all(&self.transform.position.0.to_le_bytes())?;
        writer.write_all(&self.transform.position.1.to_le_bytes())?;
        writer.write_all(&self.transform.rotation.0.to_le_bytes())?;
        writer.write_all(&self.transform.rotation.1.to_le_bytes())?;
        writer.write_all(&self.transform.rotation.2.to_le_bytes())?;
        writer.write_all(&self.transform.scale.to_le_bytes())?;
        writer.write_all(&self.transform.depth.to_le_bytes())
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

#[cfg(test)]
mod tests {
    use super::{Thing, ThingTransform, ThingType};

    #[test]
    fn writes_all_thing_fields_in_little_endian_order() {
        let thing = Thing::new(
            ThingTransform {
                position: (1.0, -2.0),
                rotation: (3.0, 4.0, 5.0),
                scale: 6.0,
                depth: 7,
            },
            ThingType::Area,
            0x1234_5678,
            0x9abc,
            "Map".to_string(),
        );

        let mut bytes = Vec::new();
        thing.write_to(&mut bytes).unwrap();

        assert_eq!(&bytes[..3], b"[T]");
        assert_eq!(&bytes[3..35], b"Map\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
        assert_eq!(&bytes[35..41], &[0x78, 0x56, 0x34, 0x12, 0xbc, 0x9a]);
        assert_eq!(bytes[41], 3);
        assert_eq!(&bytes[42..], &[
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0x80, 0x3f,
            0, 0, 0, 0xc0,
            0, 0, 0x40, 0x40,
            0, 0, 0x80, 0x40,
            0, 0, 0xa0, 0x40,
            0, 0, 0xc0, 0x40,
            7, 0, 0, 0,
        ]);
    }

    #[test]
    fn rejects_names_longer_than_the_fixed_field() {
        let thing = Thing::new(
            ThingTransform {
                position: (0.0, 0.0),
                rotation: (0.0, 0.0, 0.0),
                scale: 1.0,
                depth: 0,
            },
            ThingType::Object,
            0,
            0,
            "a".repeat(33),
        );

        let error = thing.write_to(&mut Vec::new()).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    }
}