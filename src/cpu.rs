use crate::display::Display;
use rand::Rng;


pub struct Cpu {
    ram: [u8; 4096],
    v_register: [u8; 16],
    i_register: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    display: Display,
    key: [u8; 16],
}

impl Cpu {
    pub fn new(display: Display) -> Cpu {
        Cpu {
            ram: [0; 4096],
            v_register: [0; 16],
            i_register: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            display,
            key: [0; 16],
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (i, &byte) in rom.iter().enumerate() {
            self.ram[0x200 + i] = byte;
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = (self.ram[self.pc as usize] as u16) << 8 | self.ram[self.pc as usize + 1] as u16;
        self.execute_opcode(opcode);
    }

    pub fn display_update(&mut self) {
        self.display.update();
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
                self.pc = self.stack[self.sp as usize]
            },
            _ => panic!("Unknown opcode: {:#X}", opcode)
        };
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
        self.v_register[x as usize] = (self.v_register[x as usize] + kk) & 0xFF;
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
                self.v_register[x as usize] = (self.v_register[x as usize] - self.v_register[y as usize]) as u8;
            },
            6 => {
                self.v_register[0xF] = self.v_register[x as usize] & 0x1;
                self.v_register[x as usize] >>= 1;
            },
            7 => {
                self.v_register[0xF] = if self.v_register[y as usize] > self.v_register[x as usize] { 1 } else { 0 };
                self.v_register[x as usize] = (self.v_register[y as usize] - self.v_register[x as usize]) as u8;
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
        self.v_register[x as usize] = kk & (rand::thread_rng().gen_range(0..=255) as u8);
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
            _ => panic!("Unknown opcode: {:#X}", opcode)
        }
    }
}

