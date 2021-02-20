#![no_std]
#![feature(start)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod rgb;
mod gba_color;
mod graphics;
mod font;
mod font_def;
mod io;

use rgb::RGB;
use rgb::RGBDef;
use gba_color::GBAColor;
use graphics::Graphics;
use crate::io::key_up_is_pressed;
use crate::io::key_a_is_pressed;


struct object_attributes {
    attribute_zero: u16,
    attribute_one:  u16,
    attribute_two:  u16,
    dummy:          u16,
}



const MEM_IO:       u32 = 0x04000000;   // I/Oレジスタ
const MEM_PAL:      u32 = 0x05000000;   // 1KB カラーパレット
const MEM_VRAM:     u32 = 0x06000000;   // 96KB VRAM (ビデオRAM)
const MEM_OAM:      u32 = 0x07000000;   // 1KB OAM RAM (オブジェクト属性メモリ)

const REG_DISPLAY       : *const u32 = (MEM_IO) as *const u32;
const REG_DISPLAY_VCOUNT: *const u32 = (MEM_IO | 0x00000006) as *const u32;

const oam_memory: *mut object_attributes = MEM_OAM as *mut object_attributes;
const tile_memory: *mut u16 = MEM_VRAM as *mut u16;
const object_palette_memory: *mut u16 = (MEM_PAL | 0x00000200) as *mut u16;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    init_graphic();


    // Plot RGB dot
    let red: RGB = RGB::red();
    let mut offset: u32 = ((80 * 240) + 115) as u32;
    let mut vram: *mut u16 = (MEM_VRAM + (offset * 2)) as *mut u16;
    unsafe {
        *vram = red.convert_u16_color();
    }

    let graphics: Graphics = Graphics::new();
    graphics.draw_string("Hello, World!", 10, 10, &RGB::white());

    let green: RGB = RGB::green();
    let blue: RGB = RGB::blue();
    let white: RGB = RGB::white();
    let mut x: u16 = 100;
    let mut y: u16 = 100;
    offset = ((y * 240) + x) as u32;
    vram = (MEM_VRAM + (offset * 2)) as *mut u16;
    unsafe {
        *vram = white.convert_u16_color();
    }


    let paddle_tile_memory: *mut u16 = tile_memory[4][1] as *mut u16;

    // カラーパレットメモリーの最初の16色パレット(インデックスは0)に、
    // スプライトで使うカラーパレットを書き込みます
    unsafe{
        *object_palette_memory.offset(1) = RGB::white().convert_u16_color();
        *object_palette_memory.offset(2) = RGB::magenta().convert_u16_color();
    }

    // オブジェクト属性をOAMメモリに書き込むことで、スプライトを生成します
    let paddle_attributes = object_attributes {
        attribute_zero: 0x8000,
        attribute_one: 0x4000,
        attribute_two: 1,
        dummy: 0
    };

    let ball_attributes = object_attributes {
        attribute_zero: 0,
        attribute_one: 0,
        attribute_two: 5,
        dummy: 0
    };

    unsafe {
        *oam_memory.offset(0) = paddle_attributes;
        *oam_memory.offset(1) = ball_attributes;
    }



    // IO
    let mut key_state: u32 = 0;
    loop {
        unsafe {
            wait_for_vsync();

            if (key_a_is_pressed()) {
                y += 10;
                offset = ((y * 240) + x) as u32;
                vram = (MEM_VRAM + (offset * 2)) as *mut u16;
                *vram = blue.convert_u16_color();
            }
            else {
                *vram = blue.convert_u16_color();
            }
        }
    }
    0
}

fn init_graphic() {
    let video_mode: *mut u8 = MEM_IO as *mut u8;
    let bg: *mut u8 = (MEM_IO + 1 ) as *mut u8;
    unsafe {
        *video_mode = 0x03; // mode3
        *bg = 0x04; // BG2
    }
}

fn wait_for_vsync() {
    unsafe{
        while (*REG_DISPLAY_VCOUNT >= 160){;}
        while (*REG_DISPLAY_VCOUNT < 160) {;}
    }
}

/*
#![no_std]
#![feature(start)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

use core::cell::UnsafeCell;
use core::ptr::NonNull;
use game::Game;

mod game;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(_: Layout) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

static REG_IE: usize = 0x0400_0200;
static REG_VCOUNT: usize = 0x0400_0006;

extern "C" {
    static mut __bss_start: u8;
    static mut __bss_end: u8;
    static mut __data_start: u8;
    static mut __data_end: u8;
    static __sidata: u8;
    static __wram_end: u8;
}

fn init_heap() {
    unsafe {
        let heap_start = &__bss_end as *const u8 as usize;
        let heap_end = &__wram_end as *const u8 as usize;
        let heap_size = heap_end - heap_start;

        ALLOCATOR.init(heap_start, heap_size);
    }
}

fn wait_for_vsync() {
    unsafe {
        while core::ptr::read_volatile(REG_VCOUNT as *const u16) >= 160 {}
        while core::ptr::read_volatile(REG_VCOUNT as *const u16) < 160 {}
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        let count = &__bss_end as *const u8 as usize - &__bss_start as *const u8 as usize;
        core::ptr::write_bytes(&mut __bss_start as *mut u8, 0, count);
        let count = &__data_end as *const u8 as usize - &__data_start as *const u8 as usize;
        core::ptr::copy_nonoverlapping(&__sidata as *const u8, &mut __data_start as *mut u8, count);
    }

    init_heap();
    let mut game: Game = Game::new(40, 60);
    unsafe {
        (0x400_0000 as *mut u16).write_volatile(0x0403); // BG2 Mode 3
        loop {
            let field = game.next();
            wait_for_vsync();
            for (i, cell) in field.iter().enumerate() {
                let col = i % 60;
                let row = i / 60;
                let color = if *cell { 0x7FFF } else { 0x0000 };
                for j in 0..4 {
                    (0x600_0000 as *mut u16)
                        .offset(((row * 4 + j) * 240 + col * 4) as isize)
                        .write_volatile(color);
                    (0x600_0000 as *mut u16)
                        .offset(((row * 4 + j) * 240 + col * 4 + 1) as isize)
                        .write_volatile(color);
                    (0x600_0000 as *mut u16)
                        .offset(((row * 4 + j) * 240 + col * 4 + 2) as isize)
                        .write_volatile(color);
                    (0x600_0000 as *mut u16)
                        .offset(((row * 4 + j) * 240 + col * 4 + 3) as isize)
                        .write_volatile(color);
                }
            }
        }
    }
}

pub struct Heap {
    heap: Mutex<linked_list_allocator::Heap>,
}

impl Heap {
    pub const fn empty() -> Heap {
        Heap {
            heap: Mutex::new(linked_list_allocator::Heap::empty()),
        }
    }

    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        self.heap.lock(|heap| heap.init(start_addr, size));
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.heap
            .lock(|heap| heap.allocate_first_fit(layout))
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.heap.lock(|heap| heap.deallocate(NonNull::new_unchecked(ptr), layout));
    }
}

pub struct Mutex<T> {
    inner: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Mutex {
            inner: UnsafeCell::new(value),
        }
    }
}

impl<T> Mutex<T> {
    pub fn lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        unsafe {
            let ie = core::ptr::read_volatile(REG_IE as *const u16);
            let ret = f(&mut *self.inner.get());
            (REG_IE as *mut u16).write_volatile(ie);
            ret
        }
    }
}

unsafe impl<T> Sync for Mutex<T> {}
*/
