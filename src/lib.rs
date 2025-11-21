use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::io::{Seek, SeekFrom};
use std::path::Path;
use structs::{Item, ItemDatabase, ItemFlag};

pub mod structs;

const SECRET: &str = "PBG892FXX982ABC*";

#[test]
fn test_load() {
    let mut items_dat = File::open("items.dat").unwrap();
    let mut buffer = Vec::new();
    items_dat.read_to_end(&mut buffer).unwrap();
    let item_database = load_from_memory(&buffer).unwrap();
    let item = item_database
        .get_item(&(item_database.item_count - 1))
        .unwrap();
    assert!(item.name != "")
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<ItemDatabase, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    load_from_reader(reader)
}

pub fn load_from_memory(data: &[u8]) -> Result<ItemDatabase, std::io::Error> {
    let cursor = Cursor::new(data);
    let reader = BufReader::new(cursor);
    load_from_reader(reader)
}

fn load_from_reader<R: Read + Seek>(
    mut reader: BufReader<R>,
) -> Result<ItemDatabase, std::io::Error> {
    let mut item_database = ItemDatabase::new();

    item_database.version = reader.read_u16::<LittleEndian>()?;
    item_database.item_count = reader.read_u32::<LittleEndian>()?;

    for _ in 0..item_database.item_count {
        let item = read_item(&mut reader, item_database.version)?;
        if item.id != item_database.items.len() as u32 {
            panic!("Item ID mismatch");
        }
        item_database.add_item(item);
    }
    item_database.loaded = true;
    Ok(item_database)
}

fn read_item<R: Read + Seek>(
    reader: &mut BufReader<R>,
    version: u16,
) -> Result<Item, std::io::Error> {
    let mut item = Item::new();
    item.id = reader.read_u32::<LittleEndian>()?;
    let flags = reader.read_u16::<LittleEndian>()?;
    item.flags = ItemFlag::from_bits(flags);
    item.action_type = reader.read_u8()?;
    item.material = reader.read_u8()?;
    item.name = decyper_item_name(reader, item.id);
    item.texture_file_name = read_str(reader);
    item.texture_hash = reader.read_u32::<LittleEndian>()?;
    item.visual_effect = reader.read_u8()?;
    item.cooking_ingredient = reader.read_u32::<LittleEndian>()?;
    item.texture_x = reader.read_u8()?;
    item.texture_y = reader.read_u8()?;
    item.render_type = reader.read_u8()?;
    item.is_stripey_wallpaper = reader.read_u8()?;
    item.collision_type = reader.read_u8()?;
    item.block_health = reader.read_u8()?;
    item.drop_chance = reader.read_u32::<LittleEndian>()?;
    item.clothing_type = reader.read_u8()?;
    item.rarity = reader.read_u16::<LittleEndian>()?;
    item.max_item = reader.read_u8()?;
    item.file_name = read_str(reader);
    item.file_hash = reader.read_u32::<LittleEndian>()?;
    item.audio_volume = reader.read_u32::<LittleEndian>()?;
    item.pet_name = read_str(reader);
    item.pet_prefix = read_str(reader);
    item.pet_suffix = read_str(reader);
    item.pet_ability = read_str(reader);
    item.seed_base_sprite = reader.read_u8()?;
    item.seed_overlay_sprite = reader.read_u8()?;
    item.tree_base_sprite = reader.read_u8()?;
    item.tree_overlay_sprite = reader.read_u8()?;
    item.base_color = reader.read_u32::<LittleEndian>()?;
    item.overlay_color = reader.read_u32::<LittleEndian>()?;
    item.ingredient = reader.read_u32::<LittleEndian>()?;
    item.grow_time = reader.read_u32::<LittleEndian>()?;

    reader.read_u16::<LittleEndian>()?;
    item.is_rayman = reader.read_u16::<LittleEndian>()?;

    item.extra_options = read_str(reader);
    item.texture_path_2 = read_str(reader);
    item.extra_option2 = read_str(reader);

    reader.seek(SeekFrom::Current(80))?;

    if version >= 11 {
        item.punch_option = read_str(reader);
    }
    if version >= 12 {
        reader.seek(SeekFrom::Current(13))?;
    }
    if version >= 13 {
        reader.seek(SeekFrom::Current(4))?;
    }
    if version >= 14 {
        reader.seek(SeekFrom::Current(4))?;
    }
    if version >= 15 {
        reader.seek(SeekFrom::Current(25))?;
        let _ = read_str(reader);
    }
    if version >= 16 {
        let _ = read_str(reader);
    }
    if version >= 17 {
        reader.seek(SeekFrom::Current(4))?;
    }
    if version >= 18 {
        reader.seek(SeekFrom::Current(4))?;
    }
    if version >= 19 {
        reader.seek(SeekFrom::Current(9))?;
    }
    if version >= 21 {
        reader.seek(SeekFrom::Current(2))?;
    }
    if version >= 22 {
        item.description = read_str(reader);
    }
    if version >= 23 {
        // Item recipe, 2 bytes + 2 bytes, needs parsing
        reader.seek(SeekFrom::Current(4))?;
    }

    Ok(item)
}

fn read_str<R: Read>(data: &mut BufReader<R>) -> String {
    let str_len = data.read_u16::<LittleEndian>().unwrap();
    let mut str = String::with_capacity(str_len as usize);
    for _ in 0..str_len {
        str.push(data.read_u8().unwrap() as char);
    }
    str
}

fn decyper_item_name<R: Read>(data: &mut BufReader<R>, item_id: u32) -> String {
    let str_len = data.read_u16::<LittleEndian>().unwrap();
    let mut item_name = String::with_capacity(str_len as usize);
    for i in 0..str_len {
        let char_pos = (i as u32 + item_id) % SECRET.len() as u32;
        let secret_char = SECRET.as_bytes()[char_pos as usize];
        let input_char = data.read_u8().unwrap();
        item_name.push((input_char ^ secret_char) as char);
    }
    item_name
}
