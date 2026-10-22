
use tauri::http::version;

use crate::thing::{self, Thing, ThingTransform, ThingType};
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Seek, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const MAGIC_BYTES: &[u8; 6] = b"[RMAP]";
const HEADER_MARKER: &[u8; 8] = b"[HEADER]";
const SETTINGS_MARKER: &[u8; 10] = b"[SETTINGS]";
const THINGS_MARKER: &[u8; 8] = b"[THINGS]";
const AREAS_MARKER: &[u8; 7] = b"[AREAS]";

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub things: Vec<Thing>,
    next_id: u32,
    next_layer_id: u16,
    layers: Vec<Layer>,
    pub metadata: MapMetadata,
    pub settings: MapSettings,
}

pub struct Layer {
    pub id: u16,
    pub name: String,
}

pub struct MapMetadata {
    pub name: String,
    pub create_at: SystemTime,
    pub last_edit: SystemTime,
    pub game_version: String,
}

pub struct MapSettings {
    pub background_color: (u8, u8, u8),
    pub grid_enabled: bool,
    pub printer_friendly: bool,
    pub grid_size: (u32, u32),
}

impl Map {
    pub fn new(
        size: (u32, u32),
        name: String,
        version: String,
    ) -> Self {
        let now = SystemTime::now();

        let mut map = Self {
            width: size.0,
            height: size.1,
            things: Vec::new(),
            next_id: 0,
            next_layer_id: 0,
            layers: Vec::new(),
            metadata: MapMetadata {
                name,
                create_at: now,
                last_edit: now,
                game_version: version,
            },
            settings: MapSettings {
                background_color: (255, 255, 255),
                grid_enabled: false,
                printer_friendly: true,
                grid_size: (32, 32),
            },
        };

        map.create_layer("Default".to_string());

        map
    }

    // Map editing functions

    pub fn add_thing(
        &mut self,
        thing_transform: ThingTransform,
        thing_type: ThingType,
        thing_layer: u16,
        name: String,
    ) -> u32 {

        let id = self.next_id;
        self.next_id += 1;

        let added = Thing::new(
            thing_transform,
            thing_type,
            id,
            thing_layer,
            name,
        );

        self.things.push(added);

        id
    }

    pub fn create_layer(&mut self, layer_name: String) -> u16 {
        let id = self.next_layer_id;
        self.next_layer_id += 1;

        self.layers.push(Layer {
            id,
            name: layer_name,
        });

        id
    }

    pub fn get_layer(&self, layer_id: u16) -> Option<&Layer> {
        self.layers.iter().find(|layer| layer.id == layer_id)
    }

    pub fn get_layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    // Map saving functions

    pub fn close_editing(&mut self) {
        self.metadata.last_edit = SystemTime::now();
        self.save_map();
    }

    pub fn save_map(&mut self) {
        // TODO: Definir o caminho do arquivo e chamar encode_map().
    }

    pub fn encode_map(&self, path: String) -> Result<(), String> {
        let maps_dir =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("maps");

        fs::create_dir_all(&maps_dir)
            .map_err(|e| format!("Não foi possível criar o diretório de mapas: {e}"))?;

        let file_path = maps_dir.join(path);
        
        let file = File::create(&file_path)
            .map_err(|e| format!("Não foi possível criar o arquivo: {e}"))?;

        let mut writer = BufWriter::new(file);

        // Magic bytes
        writer
            .write_all(MAGIC_BYTES)
            .map_err(|e| format!("Não foi possível escrever o identificador: {e}"))?;

        // Header
        writer
            .write_all(HEADER_MARKER)
            .map_err(|e| format!("Não foi possível escrever o cabeçalho: {e}"))?;


        //Definindo tamanho maximo das dimensões do mapa como 4 bytes

        let mut width_buf = [0u8;4];
        &width_buf[..self.width.to_le_bytes().len()].copy_from_slice(&self.width.to_le_bytes());
        
        writer
            .write_all(&width_buf)
            .map_err(|e| format!("Não foi possível escrever a largura: {e}"))?;

        let mut height_buf = [0u8;4];
        &height_buf[..self.height.to_le_bytes().len()].copy_from_slice(&self.height.to_le_bytes());
        


        writer
            .write_all(&height_buf)
            .map_err(|e| format!("Não foi possível escrever a altura: {e}"))?;

        

        writer.seek(io::SeekFrom::Current((1)));

        write_string(&mut writer, &self.metadata.name)
            .map_err(|e| format!("Não foi possível escrever o nome do mapa: {e}"))?;

        writer.seek(io::SeekFrom::Current(1));

        write_string(&mut writer, &self.metadata.game_version)
            .map_err(|e| format!("Não foi possível escrever a versão do jogo: {e}"))?;

        let created_at = self
            .metadata
            .create_at
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let last_edit = self
            .metadata
            .last_edit
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        writer
            .write_all(&created_at.to_le_bytes())
            .map_err(|e| format!("Não foi possível escrever a data de criação: {e}"))?;

        writer
            .write_all(&last_edit.to_le_bytes())
            .map_err(|e| format!("Não foi possível escrever a data de edição: {e}"))?;

        // Settings
        writer
            .write_all(SETTINGS_MARKER)
            .map_err(|e| format!("Não foi possível escrever o marcador de configurações: {e}"))?;

        let (r, g, b) = self.settings.background_color;

        writer
            .write_all(&[r, g, b])
            .map_err(|e| format!("Não foi possível escrever a cor de fundo: {e}"))?;

        writer
            .write_all(&[
                u8::from(self.settings.grid_enabled),
                u8::from(self.settings.printer_friendly),
            ])
            .map_err(|e| format!("Não foi possível escrever as configurações booleanas: {e}"))?;

        let (grid_width, grid_height) = self.settings.grid_size;

        writer
            .write_all(&grid_width.to_le_bytes())
            .map_err(|e| format!("Não foi possível escrever a largura da grade: {e}"))?;

        writer
            .write_all(&grid_height.to_le_bytes())
            .map_err(|e| format!("Não foi possível escrever a altura da grade: {e}"))?;

        // Things
        writer
            .write_all(THINGS_MARKER)
            .map_err(|e| format!("Não foi possível escrever o marcador de objetos: {e}"))?;

        let thing_count = u32::try_from(self.things.len())
            .map_err(|_| "Quantidade de objetos excede o limite do formato.".to_string())?;

        writer
            .write_all(&thing_count.to_le_bytes())
            .map_err(|e| format!("Não foi possível escrever a quantidade de objetos: {e}"))?;

        for thing in &self.things {
            if thing.name.len() > thing::THING_NAME_LENGTH {
                return Err(format!(
                    "Nome de objeto muito longo: {:?}, ID: {}",
                    thing.name,
                    thing.get_id()
                ));
            }

            thing
                .write_to(&mut writer)
                .map_err(|e| {
                    format!(
                        "Não foi possível serializar o objeto {}: {e}",
                        thing.get_id()
                    )
                })?;
        }

        // Areas — marcador reservado para implementação futura.
        writer
            .write_all(AREAS_MARKER)
            .map_err(|e| format!("Não foi possível escrever o marcador de áreas: {e}"))?;

        writer
            .flush()
            .map_err(|e| format!("Não foi possível finalizar a escrita do arquivo: {e}"))?;

        Ok(())
    }

    pub fn decode_map(filename: String) -> Result<String, String> {

        let maps_dir =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("maps");
        
        let mut filedir = maps_dir.join(format!("{}.rmap",filename));
 
        
        let file = File::open(filedir)
            .map_err(|e| format!("Não foi possível abrir o arquivo: {e}"))?;

        let mut _reader = io::BufReader::new(file);

        // TODO:
        // 1. Validar MAGIC_BYTES.
        // 2. Ler o cabeçalho na mesma ordem usada por encode_map().
        // 3. Ler as configurações.
        // 4. Reconstruir os objetos.
        // 5. Ler as áreas quando essa seção for implementada.

        //Validating Magic bytes
        let mut buffer = [0u8; 6];

        _reader.read_exact(&mut buffer)
           .map_err(|e| format!("Não foi possível ler os magic bytes: {e}"))?;

        if &buffer != MAGIC_BYTES
        {
            return Err("Arquivo corrompido ou mal formado".to_string())
        }
        
        let mut buffer = [0u8;8];
        _reader.read_exact(&mut buffer)
            .map_err(|e| format!("Não foi possivel encontrar os bits de HEADER"));

        if &buffer != HEADER_MARKER
        {

            return Err("Arquivo corrompido ou mal formado".to_string())
            
        };

        eprintln!("reader at position: {:?}",_reader.stream_position());

        let mut width_buf = [0u8;4];
        let bytes_read = _reader.read_exact(&mut width_buf);

        
        let mut height_buf = [0u8;4];
        let bytes_read1 = _reader.read_exact(&mut height_buf);        

        return Err(format!("width: {:?} | height: {:?} | Reading bytes : {:?},{:?}",width_buf,height_buf, bytes_read, bytes_read1));
        
        Err("A desserialização ainda não foi implementada.".to_string())
    }

}

/// Escreve uma string como:
/// [tamanho em bytes: u32][conteúdo UTF-8]
fn write_string<W: Write>(
    writer: &mut W,
    value: &str,
) -> io::Result<()> {
    let bytes = value.as_bytes();

    let length = u32::try_from(bytes.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "String grande demais para o formato .rmap.",
        )
    })?;

    writer.write_all(&length.to_le_bytes())?;
    writer.write_all(bytes)?;

    Ok(())
}
fn return_section_header(file:File,buffer:&[u8])
{
    println!("{:?}",buffer);
}



#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn get_section_header()
    {
        
    }
    #[test]
    fn decode_test()
    {
        let decoded_map = Map::decode_map("test".to_string())
        .map_err(|e| format!("Decoder error thrown : {:?}", e));
        
        println!("{:?}",decoded_map)
    }    

}

