use crate::display::Display;
use crate::audio::Audio;
use ::rand::thread_rng;
use ::rand::Rng;
use macroquad::prelude::*;

const KEY_MAPPINGS: [KeyCode; 16] = [
    KeyCode::Key0, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3,
    KeyCode::Key4, KeyCode::Key5, KeyCode::Key6, KeyCode::Key7,
    KeyCode::Key8, KeyCode::Key9, KeyCode::A,    KeyCode::B,
    KeyCode::C,    KeyCode::D,    KeyCode::E,    KeyCode::F
];

pub struct Cpu {
    ram: [u8; 4096],
    v_register: [u8; 16],
    i_register: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    display: Display,
    key: [u8; 16],
    delay_timer: u8,
    audio: Audio,
}

impl Cpu {
    pub fn new(display: Display, audio: Audio) -> Cpu {
        const FONT_SPRITES: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0,  // 0
            0x20, 0x60, 0x20, 0x20, 0x70,  // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0,  // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0,  // 3
            0x90, 0x90, 0xF0, 0x10, 0x10,  // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0,  // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0,  // 6
            0xF0, 0x10, 0x20, 0x40, 0x40,  // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0,  // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0,  // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90,  // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0,  // B
            0xF0, 0x80, 0x80, 0x80, 0xF0,  // C
            0xE0, 0x90, 0x90, 0x90, 0xE0,  // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0,  // E
            0xF0, 0x80, 0xF0, 0x80, 0x80   // F
        ];
        let mut ram: [u8; 4096] = [0; 4096];
        for (i, &sprite) in FONT_SPRITES.iter().enumerate() {
            ram[i] = sprite;
        }
        Cpu {
            ram,
            v_register: [0; 16],
            i_register: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            display,
            key: [0; 16],
            delay_timer: 0,
            audio
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let rom_start = 0x200;
        for (i, &byte) in rom.iter().enumerate() {
            self.ram[rom_start + i] = byte;
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = (self.ram[self.pc as usize] as u16) << 8 | self.ram[self.pc as usize + 1] as u16;
        self.execute_opcode(opcode);
    }

    pub fn display_update(&mut self) {
        self.display.update();
    }

    pub fn key_input(&mut self) {
        for i in 0..16 {
            if is_key_down(KEY_MAPPINGS[i]) {
                self.key[i as usize] = 1;
            }
            if is_key_released(KEY_MAPPINGS[i]) {
                self.key[i as usize] = 0;
            }
        }
    }

    fn next_pc(&mut self) {
        self.pc += 2;
    }

    fn execute_category_zero(&mut self, opcode: u16) {
        match opcode & 0x00FF {
            0xE0 => {
                self.display.clear();
            },
            0xEE => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            },
            _ => panic!("Unknown opcode: {:#X}", opcode)
        };
        self.next_pc();
    }

    fn execute_category_one(&mut self, opcode: u16) {
        self.pc = opcode & 0x0FFF;
    }

    fn execute_category_two(&mut self, opcode: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = opcode & 0x0FFF;
    }

    fn execute_category_three(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;
        if self.v_register[x as usize] == kk {
            self.next_pc();
        }
        self.next_pc();
    }

    fn execute_category_four(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;
        if self.v_register[x as usize] != kk {
            self.next_pc();
        }
        self.next_pc();
    }

    fn execute_category_five(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        if self.v_register[x as usize] == self.v_register[y as usize] {
            self.next_pc();
        }
        self.next_pc();
    }

    fn execute_category_six(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;
        self.v_register[x as usize] = kk;
        self.next_pc();
    }

    fn execute_category_seven(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;

        self.v_register[x as usize] = self.v_register[x as usize].wrapping_add(kk);
        self.next_pc();
    }

    fn execute_category_eight(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        match opcode & 0x000F {
            0 => {
                self.v_register[x as usize] = self.v_register[y as usize];
            },
            1 => {
                self.v_register[x as usize] |= self.v_register[y as usize];
            },
            2 => {
                self.v_register[x as usize] &= self.v_register[y as usize];
            },
            3 => {
                self.v_register[x as usize] ^= self.v_register[y as usize];
            },
            4 => {
                let result = (self.v_register[x as usize] as u16) + (self.v_register[y as usize] as u16);
                self.v_register[0xF] = if result > 0xFF { 1 } else { 0 };
                self.v_register[x as usize] = result as u8;
            },
            5 => {
                self.v_register[0xF] = if self.v_register[x as usize] > self.v_register[y as usize] { 1 } else { 0 };
                self.v_register[x as usize] = self.v_register[x as usize].wrapping_sub(self.v_register[y as usize] as u8);
            },
            6 => {
                self.v_register[0xF] = self.v_register[x as usize] & 0x1;
                self.v_register[x as usize] >>= 1;
            },
            7 => {
                self.v_register[0xF] = if self.v_register[y as usize] > self.v_register[x as usize] { 1 } else { 0 };
                self.v_register[x as usize] = self.v_register[y as usize].wrapping_sub(self.v_register[x as usize]);
            }
            0xE => {
                self.v_register[0xF] = (self.v_register[x as usize] & 0x80) >> 7;
                self.v_register[x as usize] = (self.v_register[x as usize] << 1) as u8;
            }
            _ => panic!("Unknown opcode: {:#X}", opcode)
            
        };
        self.next_pc();
    }

    fn execute_category_nine(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        if self.v_register[x as usize] != self.v_register[y as usize] {
            self.next_pc();
        }
        self.next_pc();
    }

    fn execute_category_a(&mut self, opcode: u16) {
        self.i_register = opcode & 0x0FFF;
        self.next_pc();
    }

    fn execute_category_b(&mut self, opcode: u16) {
        self.pc = self.v_register[0] as u16 + opcode & 0x0FFF;
    }

    fn execute_category_c(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;
        let mut rng = thread_rng();
        let num: u8 = rng.gen_range(0..=255);
        self.v_register[x as usize] = kk & num;
        self.next_pc();
    }

    fn execute_category_d(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = opcode & 0x000F;
        let position_x = self.v_register[x as usize] as u8 % self.display.width as u8;
        let position_y = self.v_register[y as usize] as u8 % self.display.height as u8;
        self.v_register[0xF] = 0;
        for height in 0..n {
            let sprites = self.ram[(self.i_register + height) as usize];
            for width in 0..8 {
                let sprite_pixel = (sprites >> (7 - width)) & 0x1;
                let screen_x = (position_x + width) as u8 % self.display.width as u8;
                let screen_y = (position_y + height as u8) as u8 % self.display.height as u8;
                let screen_pixel = self.display.pixels[screen_x as usize][screen_y as usize];
                let new_pixel = screen_pixel ^ sprite_pixel;
                if screen_pixel == 1 && new_pixel == 0 {
                    self.v_register[0xF] = 1;
                }
                self.display.pixels[screen_x as usize][screen_y as usize] = new_pixel;
            }
        }
        self.display.update();
        self.next_pc();
    }
    
    fn execute_category_e(&mut self, opcode: u16) {
        let x = (opcode & 0x0F00) >> 8;
        let nn = opcode & 0x00FF;
        match nn {
            0x9E => {
                if self.key[self.v_register[x as usize] as usize] == 1 {
                    self.next_pc();
                }
            },
            0xA1 => {
                if self.key[self.v_register[x as usize] as usize] != 1 {
                    self.next_pc();
                }
            },
            _ => panic!("Unknown opcode: {:#X}", opcode)
        };
        self.next_pc();
    }

    fn execute_category_f(&mut self, opcode: u16) {
        let x = (opcode & 0x0F00) >> 8;
        match opcode & 0x00FF {
            0x07 => {
                self.v_register[x as usize] = self.delay_timer;
                self.next_pc();
            },
            0x0A => {
                let mut key_pressed = None;
                for (index, &pressed) in self.key.iter().enumerate() {
                    if pressed == 1 {
                        key_pressed = Some(index);
                        break;
                    }
                }
                if let Some(index) = key_pressed {
                    self.v_register[x as usize] = index as u8;
                    self.next_pc();
                }
            },
            0x15 => {
                self.delay_timer = self.v_register[x as usize];
                self.next_pc();
            },
            0x18 => {
                self.audio.play();
                self.next_pc();
            },
            0x1E => {
                self.i_register += self.v_register[x as usize] as u16;
                self.next_pc();
            },
            0x29 => {
                self.i_register = self.v_register[x as usize] as u16 * 5;
                self.next_pc();
            },
            0x33 => {
                self.ram[self.i_register as usize] = self.v_register[x as usize] / 100;
                self.ram[self.i_register as usize + 1] = (self.v_register[x as usize] / 10) % 10;
                self.ram[self.i_register as usize + 2] = self.v_register[x as usize] % 10;
                self.next_pc();
            },
            0x55 => {
                for (i, &data) in self.v_register.iter().take(x as usize + 1).enumerate() {
                    self.ram[self.i_register as usize + i] = data;
                }
                self.i_register += x + 1;
                self.next_pc();
            },
            0x65 => {
                for (i, &data) in self.ram[self.i_register as usize..=(self.i_register + x) as usize].iter().enumerate() {
                    self.v_register[i] = data;
                }
                self.next_pc();
            },
            _ => panic!("Unknown opcode: {:#X}", opcode)
        }
    }

    fn execute_opcode(&mut self, opcode: u16) {
        let category = (opcode & 0xF000) >> 12;
        match category {
            0 => {
                self.execute_category_zero(opcode);
            },
            1 => {
                self.execute_category_one(opcode);
            },
            2 => {
                self.execute_category_two(opcode);
            },
            3 => {
                self.execute_category_three(opcode);
            },
            4 => {
                self.execute_category_four(opcode);
            },
            5 => {
                self.execute_category_five(opcode);
            },
            6 => {
                self.execute_category_six(opcode);
            },
            7 => {
                self.execute_category_seven(opcode);
            },
            8 => {
                self.execute_category_eight(opcode);
            },
            9 => {
                self.execute_category_nine(opcode);
            },
            0xA => {
                self.execute_category_a(opcode);
            },
            0xB => {
                self.execute_category_b(opcode);
            },
            0xC => {
                self.execute_category_c(opcode);
            },
            0xD => {
                self.execute_category_d(opcode);
            },
            0xE => {
                self.execute_category_e(opcode);
            },
            0xF => {
                self.execute_category_f(opcode);
            },
            _ => panic!("Unknown opcode: {:#X}", opcode)
        }
    }
}

