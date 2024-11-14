use skyline::{hook, install_hooks};

pub static mut PLAYER_ID_TAGS_INDEXES: &'static mut [u8] = &mut [0; 8];

pub fn get_tag_from_save(tag_index: u8) -> String {
    unsafe {
        let tag_address =
            (***(((*((*((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
                .add(0x5314510) as *const u64)) as *const u64))
                + 0x58) as *const *const *const u64)
                + ((tag_index as u64) * 0xF7D8)
                + 0xC) as *const u16;

        let mut tag_length = 0;
        while *tag_address.add(tag_length) != 0 {
            tag_length += 1;
        }

        String::from_utf16_lossy(std::slice::from_raw_parts(tag_address, tag_length))
    }
}

#[hook(offset = 0x19fd0b0)]
pub fn update_tag_for_player(param_1: u64, tag_index: *const u8) {
    unsafe {
        PLAYER_ID_TAGS_INDEXES[*((param_1 + 0x1d4) as *const u8) as usize] = *tag_index;
        call_original!(param_1, tag_index);
    }
}

pub fn install() {
    install_hooks!(update_tag_for_player);
}
