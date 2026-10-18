#![allow(unused_variables,unused)]

use crate::thing::{self, Thing,ThingType, ThingTransform};
use std::fmt::format;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::fs;

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub things:Vec<thing::Thing>,
    next_id : u32,
    next_layer_id:u16,
    layers:Vec<Layer>,
    pub metadata:MapMetadata,
    pub settings:MapSettings
}
pub struct Layer {
    pub id:u16,
    pub name:String
}
pub struct MapMetadata {
    pub name: String,
    pub create_at:std::time::SystemTime,
    pub last_edit:std::time::SystemTime,
    pub game_version:String
}
pub struct MapSettings
{
    background_color:(u8,u8,u8),
    grid_enabled:bool,
    printer_friendly:bool,
    grid_size:(u32,u32)
}

impl Map{
    pub fn new(size:(u32,u32),name:String,version:String) -> Self{

        let mut map = Map{
            width:size.0,
            height:size.1,
            things:Vec::new(),
            next_id:0,
            next_layer_id:0,
            layers:Vec::new(),
            metadata:MapMetadata{
                name:name,
                create_at:std::time::SystemTime::now(),
                last_edit:std::time::SystemTime::now(),
                game_version:version
            },
            settings:MapSettings{
                background_color:(255,255,255),
                grid_enabled:true,
                printer_friendly:false,
                grid_size:(32,32)
            }
        };
        map.create_layer("Default".to_string());

        map
    }


    //Map editing functions
    pub fn add_thing(&mut self, thing_transform:ThingTransform,thing_type:ThingType,thing_layer:u16,name:String) -> u32
    {
        let id: u32 = self.next_id;
        self.next_id += 1;
        let added : Thing = Thing::new(thing_transform,thing_type,id,thing_layer,name);

        self.things.push(added);

        id
    }

    pub fn create_layer(&mut self,layer_name:String) -> u16{
        let id = self.next_layer_id;
        self.next_layer_id += 1;
        let new_layer = Layer{
            id:id,
            name:layer_name
        };
        self.layers.push(new_layer);

        id
    }
    pub fn get_layer(&self,layer_id:u16) -> Option<&Layer>
    {
        self.layers.iter().find(|layer| layer.id == layer_id)
    }
    pub fn get_layers(&self) -> &Vec<Layer>
    {
        &self.layers
    }

    //Map saving functions
    pub fn close_editing(&mut self){
        self.metadata.last_edit = std::time::SystemTime::now();
        self.save_map();
    }
    pub fn save_map(&mut self){
        //...
        //Desenvolver o sistema de serializaçao personalizado
    }
    

    pub fn encode_map(&self,path:String) -> Result<(),String>
    {

        let maps_dir = PathBuf::from(format!("{}/maps",env!("CARGO_MANIFEST_DIR")));

        let maps = fs::read_dir(maps_dir.clone())
            .map_err(|e| format!("Nao foi possivel ler o diretório de mapas: {e}"))?;
        

        let mut file : File = File::create(&maps_dir.join(&path))
            .map_err(|e| return format!("Nao foi possivel criar o arquivo: {e}"))?;


        //Write the magic bytes
        let mut writer = std::io::BufWriter::new(file);
        const MAGIC_BYTES: &[u8; 6] = b"[RMAP]";

        writer.write(MAGIC_BYTES)
            .map_err(|e| format!("Nao foi possivel escrever no arquivo: {e}"))?;

        
        let mut header_bytes: &[u8; 8] = b"[HEADER]";

        writer.write(header_bytes)
            .map_err(|e| format!("Nao foi possivel escrever no arquivo: {e}"))?;

        
        let mut name_buffer = [0u8; 32];
        let name_bytes = self.metadata.name.as_bytes();

        writer.write_all(name_bytes);
        writer.write_all(&self.metadata.game_version.as_bytes());
        writer.write_all(&self.width.to_le_bytes());
        writer.write_all(&self.height.to_le_bytes());
        

        //created at
        //edit last at

        //SETTINGS HEADER
        //all settings
        let header_bytes  = b"[SETTINGS]";
        writer.write_all(header_bytes);
        

        //THINGS
         let header_bytes  = b"[THINGS]";
        writer.write_all(header_bytes);
        for thing in &self.things
        {
            

            let mut name_buffer = [0u8; 32];

            if thing.name.len() > name_buffer.len()
            {
                return Err(format!("Thing invalid name: {:?} is too long, id: {}",thing.name,thing.get_id()));
            }
            
            name_buffer[..thing.name.len()].copy_from_slice(thing.name.as_bytes());
            writer.write_all(b"[T]");
            writer.write_all(name_buffer.as_slice());
            //Outras propriedades do Thing
            let mut id_buf: [u8; 32] = [0u8; 32];
            id_buf = id_buf[..thing.get_id().to_le_bytes().len()].try_into().unwrap();

            writer.write_all(&id_buf);

            println!("Thing name: {:?}, id : {}",thing.name,thing.get_id());
        }
        let header_bytes  = b"[AREAS]";
        writer.write(header_bytes);



        Ok(())
    }
    pub fn decode_map(path: String) -> Result<Map,String>
    {
        let file  = File::open(path)
        .map_err(|e| format!("Nao foi possivel abrir o arquivo: {e}"))?;
        
        let mut map = Map::new((0,0),"".to_string(),"0.1.1".to_string());
        Ok(map)
    }
}