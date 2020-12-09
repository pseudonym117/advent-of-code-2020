use crate::parser::OpCode;

pub trait Machine {
    fn next(&mut self) -> Result<(), i32>;
    fn reset(&mut self);
}

pub struct GameJoy {
    pub accumulator: i32,
    pub instruction_pointer: usize,
    loaded_program: Vec<OpCode>,
    pub error: Option<i32>,
}

impl GameJoy {
    pub fn new(program: Vec<OpCode>) -> GameJoy {
        GameJoy {
            accumulator: 0,
            instruction_pointer: 0,
            loaded_program: program,
            error: None,
        }
    }
}

impl Machine for GameJoy {
    fn next(&mut self) -> Result<(), i32> {
        if let Some(code) = self.error {
            return Err(code);
        }

        let op = self.loaded_program.get(self.instruction_pointer);
        if op.is_none() {
            if self.instruction_pointer == self.loaded_program.len() {
                return Err(0);
            } else {
                self.error = Some(2);
                return Err(2);
            }
        }

        let mut new_ip = self.instruction_pointer + 1;
        match op.unwrap() {
            OpCode::Acc(acc) => self.accumulator += acc,
            OpCode::Jmp(rel) => {
                let tmp_ip: i32 = self.instruction_pointer as i32 + rel;
                if tmp_ip < 0 {
                    self.error = Some(1);
                    return Err(1);
                }

                new_ip = tmp_ip as usize;
            }
            _ => (),
        }

        self.instruction_pointer = new_ip;
        Ok(())
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.instruction_pointer = 0;
        self.error = None;
    }
}
