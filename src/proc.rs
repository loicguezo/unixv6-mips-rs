use crate::param;

pub struct Proc {}

pub static mut proc: [Option<Proc>; param::NPROC] = [None; param::NPROC];
