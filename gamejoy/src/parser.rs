use std::borrow::Cow;

pub enum OpCode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Clone for OpCode {
    fn clone(&self) -> Self {
        match self {
            OpCode::Nop(nop) => OpCode::Nop(*nop),
            OpCode::Acc(acc) => OpCode::Acc(*acc),
            OpCode::Jmp(jmp) => OpCode::Jmp(*jmp),
        }
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            OpCode::Nop(nop) => write!(f, "nop {}", nop),
            OpCode::Acc(acc) => write!(f, "acc {}", acc),
            OpCode::Jmp(jmp) => write!(f, "jmp {}", jmp),
        }
    }
}

fn make_op(op: &str, arg: i32) -> Option<OpCode> {
    match op.to_lowercase().as_str() {
        "nop" => Some(OpCode::Nop(arg)),
        "acc" => Some(OpCode::Acc(arg)),
        "jmp" => Some(OpCode::Jmp(arg)),
        _ => None,
    }
}

pub fn parse(program: &Cow<str>) -> Option<Vec<OpCode>> {
    Some(program.lines().filter_map(to_op).collect())
}

fn to_op(line: &str) -> Option<OpCode> {
    let mut split = line.split_ascii_whitespace().take(2);
    let op_str = split.next()?;
    let arg: i32 = split.next()?.parse().ok()?;

    make_op(op_str, arg)
}
