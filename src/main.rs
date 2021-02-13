#![no_std]
#![feature(start)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const MEM_IO:   u32 = 0x04000000;
const KEY_UP:   u32 = 0x00000040;
const KEY_DOWN: u32 = 0x00000080;
const KEY_ANY:  u32 = 0x000003FF;   // マスク用
const REG_KEY_INPUT: &u32 = &(MEM_IO + 0x00000130);
const REG_DISPLAY_VCOUNT: &u32 = &(MEM_IO + 0x00000006);

/*
    #define REG_KEY_INPUT      (*((volatile uint32 *)(MEM_IO + 0x0130)))
*/

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut key_state: u32 = 0;

    /*
        // 現在のキー入力状態の確認(REG_KEY_INPUTは反転された値を格納しています)
        key_states = ~REG_KEY_INPUT & KEY_ANY;
    */

    // RGBの表示
    unsafe {
        (0x400_0000 as *mut u16).write_volatile(0x0403); // BG2 Mode 3
        (0x600_0000 as *mut u16)
            .offset((80 * 240 + 115) as isize)  // x = 115, y = 80
            .write_volatile(0x001F);    // 0000 0000 0001 1111 = R

        (0x600_0000 as *mut u16)
            .offset((80 * 240 + 120) as isize)  // x = 120, y = 80
            .write_volatile(0x03E0);    // 0000 0011 1110 0000 = G
        
        (0x600_0000 as *mut u16)
            .offset((80 * 240 + 125) as isize)  // x = 125, y = 80
            .write_volatile(0x7C00);    // 0111 1100 0000 0000 = R
    }
    
    loop {
        // 現在のV-Blank，V-DrawSkipが終わるまでスキップ
        while(*REG_DISPLAY_VCOUNT >= 160){};

        // クロスコンパイルしたときのデバッグどうすればええのん？？
        
        //while(*REG_DISPLAY_VCOUNT < 160){};

        key_state = (!(*REG_KEY_INPUT)) & KEY_ANY;
        if ((key_state & KEY_UP).count_ones() == 1) {
            unsafe {
                (0x600_0000 as *mut u16)
                .offset((90 * 240 + 115) as isize)  // x = 115, y = 80
                .write_volatile(0x03E0);    // 0000 0000 0001 1111 = G
            }
        }
    }

    0
}
