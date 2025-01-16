#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ItemDatabase {
    pub version: u16,
    pub item_count: u32,
    pub items: HashMap<u32, Item>,
    pub loaded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ItemFlag {
    pub flippable: bool,
    pub editable: bool,
    pub seedless: bool,
    pub permanent: bool,
    pub dropless: bool,
    pub no_self: bool,
    pub no_shadow: bool,
    pub world_locked: bool,
    pub beta: bool,
    pub auto_pickup: bool,
    pub mod_flag: bool,
    pub random_grow: bool,
    pub public: bool,
    pub foreground: bool,
    pub holiday: bool,
    pub untradeable: bool,
}

impl ItemFlag {
    pub fn from_bits(bits: u16) -> ItemFlag {
        ItemFlag {
            flippable: bits & 0x1 != 0,
            editable: bits & 0x2 != 0,
            seedless: bits & 0x4 != 0,
            permanent: bits & 0x8 != 0,
            dropless: bits & 0x10 != 0,
            no_self: bits & 0x20 != 0,
            no_shadow: bits & 0x40 != 0,
            world_locked: bits & 0x80 != 0,
            beta: bits & 0x100 != 0,
            auto_pickup: bits & 0x200 != 0,
            mod_flag: bits & 0x400 != 0,
            random_grow: bits & 0x800 != 0,
            public: bits & 0x1000 != 0,
            foreground: bits & 0x2000 != 0,
            holiday: bits & 0x4000 != 0,
            untradeable: bits & 0x8000 != 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Item {
    pub id: u32,
    pub flags: ItemFlag,
    pub action_type: u8,
    pub material: u8,
    pub name: String,
    pub texture_file_name: String,
    pub texture_hash: u32,
    pub cooking_ingredient: u32,
    pub visual_effect: u8,
    pub texture_x: u8,
    pub texture_y: u8,
    pub render_type: u8,
    pub is_stripey_wallpaper: u8,
    pub collision_type: u8,
    pub block_health: u8,
    pub drop_chance: u32,
    pub clothing_type: u8,
    pub rarity: u16,
    pub max_item: u8,
    pub file_name: String,
    pub file_hash: u32,
    pub audio_volume: u32,
    pub pet_name: String,
    pub pet_prefix: String,
    pub pet_suffix: String,
    pub pet_ability: String,
    pub seed_base_sprite: u8,
    pub seed_overlay_sprite: u8,
    pub tree_base_sprite: u8,
    pub tree_overlay_sprite: u8,
    pub base_color: u32,
    pub overlay_color: u32,
    pub ingredient: u32,
    pub grow_time: u32,
    pub is_rayman: u16,
    pub extra_options: String,
    pub texture_path_2: String,
    pub extra_option2: String,
    pub punch_option: String,
}

impl ItemDatabase {
    pub fn new() -> ItemDatabase {
        ItemDatabase {
            version: 0,
            item_count: 0,
            items: HashMap::new(),
            loaded: false,
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.insert(item.id.clone(), item);
    }

    pub fn get_item_as_ref(&self, id: &u32) -> Option<&Item> {
        self.items.get(id)
    }

    pub fn get_item(&self, id: &u32) -> Option<Item> {
        match self.items.get(id) {
            Some(item) => Some(item.clone()),
            None => None,
        }
    }
}

impl Item {
    pub fn new() -> Item {
        Item {
            id: 0,
            flags: ItemFlag {
                flippable: false,
                editable: false,
                seedless: false,
                permanent: false,
                dropless: false,
                no_self: false,
                no_shadow: false,
                world_locked: false,
                beta: false,
                auto_pickup: false,
                mod_flag: false,
                random_grow: false,
                public: false,
                foreground: false,
                holiday: false,
                untradeable: false,
            },
            action_type: 0,
            material: 0,
            name: String::new(),
            texture_file_name: String::new(),
            texture_hash: 0,
            cooking_ingredient: 0,
            visual_effect: 0,
            texture_x: 0,
            texture_y: 0,
            render_type: 0,
            is_stripey_wallpaper: 0,
            collision_type: 0,
            block_health: 0,
            drop_chance: 0,
            clothing_type: 0,
            rarity: 0,
            max_item: 0,
            file_name: String::new(),
            file_hash: 0,
            audio_volume: 0,
            pet_name: String::new(),
            pet_prefix: String::new(),
            pet_suffix: String::new(),
            pet_ability: String::new(),
            seed_base_sprite: 0,
            seed_overlay_sprite: 0,
            tree_base_sprite: 0,
            tree_overlay_sprite: 0,
            base_color: 0,
            overlay_color: 0,
            ingredient: 0,
            grow_time: 0,
            is_rayman: 0,
            extra_options: String::new(),
            texture_path_2: String::new(),
            extra_option2: String::new(),
            punch_option: String::new(),
        }
    }
}
