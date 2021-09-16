use std::fmt::Display;

pub enum BiOp {
    Sum,
    Sub,
    Mul,
    Div,
    And,
    Or,
}

pub enum UnOp {
    Sum,
    Sub,
    Not,
    LShift,
    RShift,
    Cast(String),
}

pub enum RlOp {
    Gte,
    Gt,
    Lte,
    Lt,
    Eq,
    Ne,
}
pub enum AdOp {
    DerRef,
    Ref,
}

pub struct Instruction(Option<usize>, InstructionOp);

pub enum InstructionOp {
    NoOp,
    OpBinary(String, String, BiOp, String),
    OpUnary(String, UnOp, String),
    Copy(String, String),
    IndexedCopy(String, Option<String>, String, Option<String>),
    AddressCopy(bool, String, Option<AdOp>, String),
    IncondBranch(usize),
    CondBranch(usize, String, RlOp, String),
    FuncCall(String, String, Vec<String>),
}

impl Display for BiOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BiOp::Sum => write!(f, "+"),
            BiOp::Sub => write!(f, "-"),
            BiOp::Mul => write!(f, "*"),
            BiOp::Div => write!(f, "/"),
            BiOp::And => write!(f, "and"),
            BiOp::Or => write!(f, "or"),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnOp::Sum => write!(f, "+"),
            UnOp::Sub => write!(f, "-"),
            UnOp::Not => write!(f, "not"),
            UnOp::LShift => write!(f, "<<"),
            UnOp::RShift => write!(f, ">>"),
            UnOp::Cast(type_name) => write!(f, "{}", type_name),
        }
    }
}

impl Display for RlOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RlOp::Gte => write!(f, ">="),
            RlOp::Gt => write!(f, ">"),
            RlOp::Lte => write!(f, "<="),
            RlOp::Lt => write!(f, "<"),
            RlOp::Eq => write!(f, "=="),
            RlOp::Ne => write!(f, "!="),
        }
    }
}

impl Display for AdOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdOp::DerRef => write!(f, "*"),
            AdOp::Ref => write!(f, "&"),
        }
    }
}

impl Display for InstructionOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionOp::NoOp => write!(f, ""),
            InstructionOp::OpBinary(dst, s1, op, s2) => write!(f, "{} = {} {} {}", dst, s1, op, s2),
            InstructionOp::OpUnary(dst, op, src) => write!(f, "{} = {} {}", dst, op, src),
            InstructionOp::Copy(dst, src) => write!(f, "{} = {}", dst, src),
            InstructionOp::IndexedCopy(dst, ixd, src, ixs) => write!(
                f,
                "{}{} = {}{}",
                dst,
                ixd.as_ref()
                    .and_then(|i| { Some(format!("[{}]", i)) })
                    .unwrap_or_default(),
                src,
                ixs.as_ref()
                    .and_then(|i| { Some(format!("[{}]", i)) })
                    .unwrap_or_default()
            ),
            InstructionOp::AddressCopy(dst_deref, dst, op, src) => write!(
                f,
                "{}{} = {}{}",
                if *dst_deref { "*" } else { "" },
                dst,
                op.as_ref()
                    .and_then(|op| { Some(format!("{}", op)) })
                    .unwrap_or_default(),
                src
            ),
            InstructionOp::IncondBranch(label) => write!(f, "goto {}", label),
            InstructionOp::CondBranch(label, r1, op, r2) => {
                write!(f, "if {} {} {} goto {}", r1, op, r2, label)
            }
            InstructionOp::FuncCall(dst, name, params) => {
                for (idx, param) in params.iter().enumerate() {
                    if idx != 0 {
                        writeln!(f, "{:>9}param {}", "",param)?;
                    } else {
                        writeln!(f, "param {}", param)?;
                    }
                }
                write!(f, "{:>9}{} = call {}, {}", "", dst, name, params.len())
            }
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>8}{}{}",
            self.0
                .and_then(|l| { Some(l.to_string()) })
                .unwrap_or_default(),
            if self.0.is_some() { ":" } else { " " },
            self.1
        )
    }
}

#[test]
fn test_gen_ir_basic() {
    let code_insts = vec![
        Instruction(
            None,
            InstructionOp::OpBinary("t1".to_owned(), "y".to_owned(), BiOp::Mul, "x".to_owned()),
        ),
        Instruction(
            None,
            InstructionOp::OpBinary("t2".to_owned(), "x".to_owned(), BiOp::Sum, "t1".to_owned()),
        ),
        Instruction(
            None,
            InstructionOp::OpBinary("t3".to_owned(), "y".to_owned(), BiOp::Mul, "x".to_owned()),
        ),
        Instruction(
            None,
            InstructionOp::OpBinary("t4".to_owned(), "t3".to_owned(), BiOp::Sum, "z".to_owned()),
        ),
        Instruction(
            None,
            InstructionOp::CondBranch(1usize, "t2".to_owned(), RlOp::Lte, "t4".to_owned()),
        ),
        Instruction(None, InstructionOp::Copy("a".to_owned(), "0".to_owned())),
        Instruction(
            Some(1),
            InstructionOp::FuncCall("ax".to_owned(), "batatinha".to_owned(), vec!["a".to_owned(), "b".to_owned(), "c".to_owned()])
        )
    ];
    for inst in code_insts {
        println!("{}", inst)
    }
}
