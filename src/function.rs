use crate::TypeId;

struct Function {
    name: String,
    args: Vec<(String, Option<TypeId>)>,
    ret: Option<TypeId>,
    body: Block,
}

type Block = Vec<Instruction>;

pub enum Instruction {
    Call(String),
    Ret(Value)
}


