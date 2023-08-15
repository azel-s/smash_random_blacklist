use skyline::{
    hook,
    hooks::{getRegionAddress, Region},
    install_hooks,
};

static UPDATE_TAG_FOR_PLAYER_OFFSET: usize = 0x19fc5b0;
static PLAYER_SAVE_OFFSET: usize = 0x5312510;
static mut PLAYER_SAVE_ADDRESS: *const u64 = 0x0 as *const u64;

pub fn get_tag_from_save(tag_index: u8) -> String {
    unsafe {
        let addr = (***((*((*PLAYER_SAVE_ADDRESS) as *const u64) + 0x58)
            as *const *const *const u64)
            + ((tag_index as u64) * 0xF7D8)
            + 0xC) as *const u16;
        let mut len = 0;
        while *addr.add(len) != 0 {
            len += 1;
        }

        let slice = std::slice::from_raw_parts(addr, len);
        String::from_utf16_lossy(slice)
    }
}

#[hook(offset = UPDATE_TAG_FOR_PLAYER_OFFSET)]
pub fn update_tag_for_player(param_1: u64, tag_index: *const u8) {
    unsafe {
        println!("{}", get_tag_from_save(*tag_index));
        call_original!(param_1, tag_index);
    }
}

pub fn offset_to_addr(offset: usize) -> *const () {
    unsafe { (getRegionAddress(Region::Text) as *const u8).add(offset) as _ }
}

pub fn install() {
    unsafe {
        PLAYER_SAVE_ADDRESS = offset_to_addr(PLAYER_SAVE_OFFSET) as *const u64;
    }
    install_hooks!(update_tag_for_player);
}
