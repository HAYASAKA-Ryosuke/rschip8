use crate::display::Display;


pub struct Cpu {
    ram: [u8; 4096],
    v_register: [u8; 16],
    i_register: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    display: Display,
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

    fn execute_category_zero(&mut self, opcode: u16) {
        match opcode & 0x00FF {
            0xE0 => {
                self.display.clear();
            },
            0xEE => {
                self.pc = self.stack[self.sp as usize]
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
            _ => panic!("Unknown opcode: {:#X}", opcode)
        }
    }
}

