use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::BufReader;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use structs::{Item, ItemDatabase};

pub mod structs;

const SECRET: &str = "PBG892FXX982ABC*";

#[test]
fn test_load() {
    let item_database = load_from_file("items.dat").unwrap();
    let item = item_database
        .get_item(&(item_database.item_count - 1))
        .unwrap();
    assert!(item.name != "")
}

pub fn load_from_file(path: &str) -> Result<ItemDatabase, std::io::Error> {
    let mut item_database = ItemDatabase::new();

    let file = File::open(Path::new(path))?;
    let mut reader = BufReader::new(file);

    item_database.version = reader.read_u16::<LittleEndian>()?;
    item_database.item_count = reader.read_u32::<LittleEndian>()?;

    for i in 0..item_database.item_count {
        let mut item = Item::new();
        item.id = reader.read_u32::<LittleEndian>()?;
        item.flags = reader.read_u16::<LittleEndian>()?;
        item.action_type = reader.read_u8()?;
        item.material = reader.read_u8()?;
        item.name = decyper_item_name(&mut reader, item.id);
        item.texture_file_name = read_str(&mut reader);
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
        item.file_name = read_str(&mut reader);
        item.file_hash = reader.read_u32::<LittleEndian>()?;
        item.audio_volume = reader.read_u32::<LittleEndian>()?;
        item.pet_name = read_str(&mut reader);
        item.pet_prefix = read_str(&mut reader);
        item.pet_suffix = read_str(&mut reader);
        item.pet_ability = read_str(&mut reader);
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
        item.extra_options = read_str(&mut reader);
        item.texture_path_2 = read_str(&mut reader);
        item.extra_option2 = read_str(&mut reader);
        reader.seek(SeekFrom::Current(80))?;

        if item_database.version >= 11 {
            item.punch_option = read_str(&mut reader);
        }
        if item_database.version >= 12 {
            reader.seek(SeekFrom::Current(13))?;
        }
        if item_database.version >= 13 {
            reader.seek(SeekFrom::Current(4))?;
        }
        if item_database.version >= 14 {
            reader.seek(SeekFrom::Current(4))?;
        }
        if item_database.version >= 15 {
            reader.seek(SeekFrom::Current(25))?;
            read_str(&mut reader);
        }
        if item_database.version >= 16 {
            read_str(&mut reader);
        }
        if item_database.version >= 17 {
            reader.seek(SeekFrom::Current(4))?;
        }
        if item_database.version >= 18 {
            reader.seek(SeekFrom::Current(4))?;
        }

        if i != item.id {
            panic!("Item id mismatch");
        }
        item_database.add_item(item);
    }
    Ok(item_database)
}

fn read_str(data: &mut BufReader<File>) -> String {
    let str_len = data.read_u16::<LittleEndian>().unwrap();
    let mut str = String::with_capacity(str_len as usize);
    for _ in 0..str_len {
        str.push(data.read_u8().unwrap() as char);
    }
    return str;
}

fn decyper_item_name(data: &mut BufReader<File>, item_id: u32) -> String {
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
