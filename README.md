<br/>
<div align="center">
<h3 align="center">GTitem-r</h3>
<p align="center">
Items.dat parser for Growtopia in Rust
</p>
</div>

## Usage

```rust
use gtitem_r::parse_from_file;

fn main() {
  let item_database = parse_from_file("items.dat").unwrap();
  let item = item_database.get_item(0).unwrap();
  println!("{:?}", item)
}
```

## Property

### ItemDatabase

- version - items.dat version
- item_count - all item in the database
- items - HashMap<String, Item>

### Item

- id: u32,
- flags: u16,
- action_type: u8,
- material: u8,
- name: String,
- texture_file_name: String,
- texture_hash: u32,
- cooking_ingredient: u32,
- visual_effect: u8,
- texture_x: u8,
- texture_y: u8,
- render_type: u8,
- is_stripey_wallpaper: u8,
- collision_type: u8,
- block_health: u8,
- drop_chance: u32,
- clothing_type: u8,
- rarity: u16,
- max_item: u8,
- file_name: String,
- file_hash: u32,
- audio_volume: u32,
- pet_name: String,
- pet_prefix: String,
- pet_suffix: String,
- pet_ability: String,
- seed_base_sprite: u8,
- seed_overlay_sprite: u8,
- tree_base_sprite: u8,
- tree_overlay_sprite: u8,
- base_color: u32,
- overlay_color: u32,
- ingredient: u32,
- grow_time: u32,
- is_rayman: u16,
- extra_options: String,
- texture_path_2: String,
- extra_option2: String,
- punch_option: String,
