use std::any::Any;
use std::io::{Read, Seek, SeekFrom};
use std::ptr::null;
use std::{collections::HashMap, io::BufReader};
use std::path::Path;
use std::fs::File;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

const SECRET: &str = "PBG892FXX982ABC*";

struct Item {
    id: u32,
    flag: u16,
    item_type: u8,
    material: u8,
    name: String
}

struct Database {
    version: u16,
    item_count: u32,
    items: Vec<Item>
}

fn main() {
    load_from_file("items.dat");
}

fn load_from_file(path: &str) {
    let file = File::open(Path::new(path)).expect("Cannot read items.dat file.");
    let mut reader = BufReader::new(file);
    let version = reader.read_u16::<LittleEndian>().unwrap();
    let item_count = reader.read_u32::<LittleEndian>().unwrap();
    // let mut items = Vec::new();
    println!("version: {}, item_count: {}", version, item_count);

    let id = reader.read_u32::<LittleEndian>().unwrap();
    let flag = reader.read_u16::<LittleEndian>().unwrap();
    let item_type = reader.read_u8().unwrap();
    let material = reader.read_u8().unwrap();
    let name = decyper_item_name(&mut reader, id);
    let texture_file_name = read_str(&mut reader);
    let texture_hash = reader.read_u32::<LittleEndian>().unwrap();
    let cooking_ingredient = reader.read_u32::<LittleEndian>().unwrap();
    let visual_effect = reader.read_u8().unwrap();
    let texture_x = reader.read_u8().unwrap();
    let texture_y: u8 = reader.read_u8().unwrap();
    let render_type = reader.read_u8().unwrap();
    let is_stripey_wallpaper = reader.read_u8().unwrap();
    let collision_type = reader.read_u8().unwrap();
    let block_health = reader.read_u8().unwrap();
    reader.read_u32::<LittleEndian>().unwrap();
    let clothing_type = reader.read_u8().unwrap();
    let rarity = reader.read_u16::<LittleEndian>().unwrap();
    let max_item = reader.read_u8().unwrap();
    let file_name = read_str(&mut reader);
    let file_hash = reader.read_u32::<LittleEndian>().unwrap();
    let animation_length = reader.read_u16::<LittleEndian>().unwrap();
    read_str(&mut reader);
    read_str(&mut reader);
    read_str(&mut reader);
    read_str(&mut reader);
    let seed_base_sprite = reader.read_u8().unwrap();
    let seed_overlay_sprite = reader.read_u8().unwrap();
    let tree_base_sprite = reader.read_u8().unwrap();
    let tree_overlay_sprite = reader.read_u8().unwrap();
    let base_color = reader.read_u32::<LittleEndian>().unwrap();
    let overlay_color = reader.read_u32::<LittleEndian>().unwrap();
    reader.read_u32::<LittleEndian>().unwrap();
    let tree_grow_time = reader.read_u32::<LittleEndian>().unwrap();

    println!("id: {}, flag: {}, item_type: {}, material: {}, name: {}, texture_file_name: {}, texture_hash: {}, cooking_ingredient: {}, visual_effect: {}, texture_x: {}, texture_y: {}, render_type: {}, is_stripey_wallpaper: {}, collision_type: {}, block_health: {}, clothing_type: {}, rarity: {}, max_item: {}, file_name: {}, file_hash: {}, animation_length: {}, seed_base_sprite: {}, seed_overlay_sprite: {}, tree_base_sprite: {}, tree_overlay_sprite: {}, base_color: {}, overlay_color: {}, tree_grow_time: {}", id, flag, item_type, material, name, texture_file_name, texture_hash, cooking_ingredient, visual_effect, texture_x, texture_y, render_type, is_stripey_wallpaper, collision_type, block_health, clothing_type, rarity, max_item, file_name, file_hash, animation_length, seed_base_sprite, seed_overlay_sprite, tree_base_sprite, tree_overlay_sprite, base_color, overlay_color, tree_grow_time);

//     return Database {
//         version,
//         item_count,
//         items
//     };
}

fn read_str(data: &mut BufReader<File>) -> String {
    let mut str = String::new();
    let mut str_len = data.read_u16::<LittleEndian>().unwrap();
    if str_len == 256 {
        str_len = 0;
    }
    for _ in 0..str_len {
        str.push(data.read_u8().unwrap() as char);
    }
    return str;
}

fn decyper_item_name(data: &mut BufReader<File>, item_id: u32) -> String {
    let mut item_name = String::new();
    let str_len = data.read_u16::<LittleEndian>().unwrap();

    for i in 0..str_len {
        let char_pos = (i as u32 + item_id) % SECRET.len() as u32;
        let secret_char = SECRET.as_bytes()[char_pos as usize];
        let input_char = data.read_u8().unwrap();
        item_name.push((input_char ^ secret_char) as char);
    }

    return item_name;
}