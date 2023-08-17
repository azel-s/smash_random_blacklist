use rand::prelude::SliceRandom;
use rand::Rng;

use crate::{name, RANDOM_WHITELIST_CONFIG};

static REGULAR_CHARA_HASHES: &[u64] = &[
    smash::hash40("ui_chara_bayonetta"),
    smash::hash40("ui_chara_captain"),
    smash::hash40("ui_chara_chrom"),
    smash::hash40("ui_chara_cloud"),
    smash::hash40("ui_chara_daisy"),
    smash::hash40("ui_chara_dedede"),
    smash::hash40("ui_chara_diddy"),
    smash::hash40("ui_chara_donkey"),
    smash::hash40("ui_chara_duckhunt"),
    smash::hash40("ui_chara_falco"),
    smash::hash40("ui_chara_fox"),
    smash::hash40("ui_chara_gamewatch"),
    smash::hash40("ui_chara_ganon"),
    smash::hash40("ui_chara_gaogaen"),
    smash::hash40("ui_chara_gekkouga"),
    smash::hash40("ui_chara_ice_climber"),
    smash::hash40("ui_chara_ike"),
    smash::hash40("ui_chara_inkling"),
    smash::hash40("ui_chara_kamui"),
    smash::hash40("ui_chara_ken"),
    smash::hash40("ui_chara_kirby"),
    smash::hash40("ui_chara_koopa"),
    smash::hash40("ui_chara_koopajr"),
    smash::hash40("ui_chara_krool"),
    smash::hash40("ui_chara_link"),
    smash::hash40("ui_chara_littlemac"),
    smash::hash40("ui_chara_lucario"),
    smash::hash40("ui_chara_lucas"),
    smash::hash40("ui_chara_lucina"),
    smash::hash40("ui_chara_luigi"),
    smash::hash40("ui_chara_mario"),
    smash::hash40("ui_chara_marth"),
    smash::hash40("ui_chara_metaknight"),
    smash::hash40("ui_chara_mewtwo"),
    smash::hash40("ui_chara_murabito"),
    smash::hash40("ui_chara_ness"),
    smash::hash40("ui_chara_pacman"),
    smash::hash40("ui_chara_palutena"),
    smash::hash40("ui_chara_peach"),
    smash::hash40("ui_chara_pichu"),
    smash::hash40("ui_chara_pikachu"),
    smash::hash40("ui_chara_pikmin"),
    smash::hash40("ui_chara_pit"),
    smash::hash40("ui_chara_pitb"),
    smash::hash40("ui_chara_ptrainer"),
    smash::hash40("ui_chara_purin"),
    smash::hash40("ui_chara_reflet"),
    smash::hash40("ui_chara_richter"),
    smash::hash40("ui_chara_ridley"),
    smash::hash40("ui_chara_robot"),
    smash::hash40("ui_chara_rockman"),
    smash::hash40("ui_chara_rosetta"),
    smash::hash40("ui_chara_roy"),
    smash::hash40("ui_chara_ryu"),
    smash::hash40("ui_chara_samus"),
    smash::hash40("ui_chara_samusd"),
    smash::hash40("ui_chara_sheik"),
    smash::hash40("ui_chara_shizue"),
    smash::hash40("ui_chara_shulk"),
    smash::hash40("ui_chara_simon"),
    smash::hash40("ui_chara_snake"),
    smash::hash40("ui_chara_sonic"),
    smash::hash40("ui_chara_szerosuit"),
    smash::hash40("ui_chara_toonlink"),
    smash::hash40("ui_chara_wario"),
    smash::hash40("ui_chara_wiifit"),
    smash::hash40("ui_chara_wolf"),
    smash::hash40("ui_chara_yoshi"),
    smash::hash40("ui_chara_younglink"),
    smash::hash40("ui_chara_zelda"),
    //smash::hash40("ui_chara_brave"),
    //smash::hash40("ui_chara_buddy"),
    //smash::hash40("ui_chara_demon"),
    //smash::hash40("ui_chara_dolly"),
    //smash::hash40("ui_chara_edge"),
    //smash::hash40("ui_chara_flame_first"),
    //smash::hash40("ui_chara_light_first"),
    //smash::hash40("ui_chara_jack"),
    //smash::hash40("ui_chara_master"),
    //smash::hash40("ui_chara_packun"),
    //smash::hash40("ui_chara_pickel"),
    //smash::hash40("ui_chara_tantan"),
    //smash::hash40("ui_chara_trail")
];

static PT_CHARA_HASHES: &[u64] = &[
    smash::hash40("ui_chara_pzenigame"),
    smash::hash40("ui_chara_plizardon"),
    smash::hash40("ui_chara_pfushigisou"),
];

static mut LAST_FIGHTER_FOUND: u64 = 0x0;
static mut LAST_FIGHTER_SUB_FOUND: u64 = 0x0;

static mut WAS_RANDOM_SELECTION: bool = false;

const HASH_MASK: u64 = 0xFF_FFFFFFFF;
const KEY_MASK: u64 = 0xFFFFFF_0000000000;
const RANDOM_HASH: u64 = 0xfd5f7fa78;

fn is_random(entry: u64) -> bool {
    (entry & HASH_MASK) == RANDOM_HASH
}

fn key(entry: u64) -> u64 {
    entry & KEY_MASK
}

#[skyline::hook(offset = 0x1a13770, inline)]
unsafe fn change_random_early(ctx: &mut skyline::hooks::InlineCtx) {
    let player_id =
        (*(*(ctx.registers[21].x.as_ref() as *const u64) as *const u64) + 0x150) as *const u8;

    let player_tag_index = name::PLAYER_ID_TAGS_INDEXES[*player_id as usize];
    let player_tag = name::get_tag_from_save(player_tag_index);

    let mut hashes: Vec<u64> = Vec::new();

    if RANDOM_WHITELIST_CONFIG.0.contains_key(&player_tag) {
        hashes = RANDOM_WHITELIST_CONFIG
            .0
            .get(&player_tag)
            .unwrap()
            .allow
            .iter()
            .map(|x| smash::hash40(x))
            .collect::<Vec<u64>>();
    }

    if hashes.is_empty() {
        hashes = REGULAR_CHARA_HASHES.to_vec();
    }

    let obj = *ctx.registers[23].x.as_ref() as *mut u64;
    let obj = *(obj as *mut *mut u64).add(1);

    let main_chara: u64 = *obj.add(0x200 / 0x8);
    let sub_chara = *obj.add(0x208 / 0x8);

    if !ninput::any::is_down_any(ninput::Buttons::ZL | ninput::Buttons::ZR)
        && (is_random(main_chara) || is_random(sub_chara))
    {
        let chara_hash = hashes
            .choose(&mut rand::thread_rng())
            .copied()
            .unwrap_or(RANDOM_HASH);

        LAST_FIGHTER_FOUND = chara_hash | key(main_chara);
        LAST_FIGHTER_SUB_FOUND = if chara_hash == smash::hash40("ui_chara_ptrainer") {
            PT_CHARA_HASHES
                .choose(&mut rand::thread_rng())
                .copied()
                .unwrap_or(RANDOM_HASH)
                | key(sub_chara)
        } else {
            chara_hash | key(sub_chara)
        };

        *ctx.registers[24].x.as_mut() = LAST_FIGHTER_FOUND;
        WAS_RANDOM_SELECTION = true;
    } else {
        WAS_RANDOM_SELECTION = false;
    }
}

#[skyline::hook(offset = 0x1a0ca40)]
unsafe fn decide_fighter(arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    if WAS_RANDOM_SELECTION {
        let p_main_chara = (arg1 as *mut u64).add(2);
        let p_sub_chara = (arg1 as *mut u64).add(3);

        if WAS_RANDOM_SELECTION && (is_random(*p_main_chara) || is_random(*p_sub_chara)) {
            *p_main_chara = LAST_FIGHTER_FOUND;
            *p_sub_chara = LAST_FIGHTER_SUB_FOUND;
        }

        WAS_RANDOM_SELECTION = false;
    }

    call_original!(arg1, arg2, arg3, arg4)
}

#[skyline::hook(offset = 0x1a1c030)]
unsafe fn copy_fighter_info2(dest: u64, src: u64) {
    if WAS_RANDOM_SELECTION {
        let src_obj = *(src as *mut *mut u64).add(1);
        let src_obj = src_obj.add(0x1F0 / 8);

        *(src_obj as *mut u32).add(8) = rand::thread_rng().gen::<u32>() % 8;
    }

    call_original!(dest, src);
}

pub fn install() {
    skyline::install_hooks!(change_random_early, decide_fighter, copy_fighter_info2);
}
