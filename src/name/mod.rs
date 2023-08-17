use skyline::{
    hook,
    hooks::{getRegionAddress, Region},
    install_hooks,
};

static UPDATE_TAG_FOR_PLAYER_OFFSET: usize = 0x19fc5b0; // Offset where the function to update the tag is?
static PLAYER_SAVE_OFFSET: usize = 0x5312510; // Offset where save data is located?
static mut PLAYER_SAVE_ADDRESS: *const u64 = 0x0 as *const u64; // The previous but as address?

pub static mut PLAYER_ID_TAGS: &'static mut [u8] = &mut [0; 8];

pub fn get_tag_from_save(tag_index: u8) -> String {
    unsafe {
        // Address of tag?
        // Why + 0x58 (88)?
        // Is each tag 0xF7D8 (63448)? Hence you multiply that many times?
        // Why + 0xC (12)?
        let addr = (***((*((*PLAYER_SAVE_ADDRESS) as *const u64) + 0x58)
            as *const *const *const u64)
            + ((tag_index as u64) * 0xF7D8)
            + 0xC) as *const u16;

        // Gets length of label by looking for null-terminating character.
        let mut len = 0;
        while *addr.add(len) != 0 {
            println!("[{len}]: {}", *addr.add(len));
            len += 1;
        }

        // Return string by reading address until length?
        return String::from_utf16_lossy(std::slice::from_raw_parts(addr, len));
    }
}

// TODO: Figure out what param_1 is. Could be player #?
#[hook(offset = UPDATE_TAG_FOR_PLAYER_OFFSET)]
pub fn update_tag_for_player(param_1: u64, tag_index: *const u8) {
    unsafe {
        let player_id = (param_1 + 0x1d4) as *const u8;
        PLAYER_ID_TAGS[*player_id as usize] = *tag_index;

        println!("Player #: {}, Tag: {}", *player_id, get_tag_from_save(*tag_index));
        call_original!(param_1, tag_index);
    }
}

// pub fn offset_to_addr(offset: usize) -> *const () {
//     unsafe { (getRegionAddress(Region::Text) as *const u8).add(offset) as _ }
// }

pub fn install() {
    unsafe {
        //PLAYER_SAVE_ADDRESS = offset_to_addr(PLAYER_SAVE_OFFSET) as *const u64;
        PLAYER_SAVE_ADDRESS =
            (getRegionAddress(Region::Text) as *const u8).add(PLAYER_SAVE_OFFSET) as *const u64;
    }
    install_hooks!(update_tag_for_player);
}
