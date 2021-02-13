#![no_std]
#![feature(start)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const MEM_IO:       u32 = 0x04000000;

// Keypad Input 
// Reference : http://www.akkit.org/info/gbatek.htm#gbakeypadinput
const KEY_A:        u32 = 0x00000001; // 0  (0 = Pressed, 1 = Released)
const KEY_B:        u32 = 0x00000002; // 1
const KEY_SELECT:   u32 = 0x00000004; // 2
const KEY_START:    u32 = 0x00000008; // 3
const KEY_RIGHT:    u32 = 0x00000010; // 4
const KEY_LEFT:     u32 = 0x00000020; // 5
const KEY_UP:       u32 = 0x00000040; // 6
const KEY_DOWN:     u32 = 0x00000080; // 7
const KEY_R:        u32 = 0x00000100; // 8
const KEY_L:        u32 = 0x00000200; // 9 (10 ~ 15 Not Used)
const KEY_ANY:      u32 = 0x000003FF; // mask


const REG_KEY_INPUT: &u32 = &(MEM_IO + 0x00000130);
const REG_DISPLAY_VCOUNT: &u32 = &(MEM_IO + 0x00000006);


#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut key_state: u32 = 0;

    // Graphic
    // Display Size : 240 * 160 (左上がx = 0, y =0)
    // Color Format : 16bit (? BBBBB GGGGG RRRRR)
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
        // GBAは60fpsで動作

        // 現在のV-Blank，V-DrawSkipが終わるまでスキップ
        while(*REG_DISPLAY_VCOUNT >= 160){};        
        //while(*REG_DISPLAY_VCOUNT < 160){};



        // ゲームループ


        // I/O
        
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
