use std::fmt;

use crate::{Instruction, Opcode, Operand};

const REG_NAMES: [&'static str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.opcode == Opcode::ADDI {
            match (
                self.operand(&self.operands[0]),
                self.operand(&self.operands[1]),
                self.operand(&self.operands[2]),
            ) {
                (Some(a), Some(b), Some(Operand::Imm(0))) => {
                    if a == b {
                        return write!(f, "nop");
                    } else {
                        return write!(f, "mv {}, {}", a, b);
                    }
                }
                _ => {}
            }
        } else if self.opcode == Opcode::BEQ {
            match (
                self.operand(&self.operands[0]),
                self.operand(&self.operands[1]),
                self.operand(&self.operands[2]),
            ) {
                (Some(Operand::Reg(a)), Some(Operand::Reg(b)), Some(Operand::Imm(offs))) => {
                    if a == b {
                        if offs < 0 {
                            return write!(f, "beq $-{:#x}", offs.wrapping_neg());
                        } else {
                            return write!(f, "beq $+{:#x}", offs);
                        }
                    }
                }
                _ => {}
            }
        } else if self.opcode == Opcode::SLL {
            match (
                self.operand(&self.operands[0]),
                self.operand(&self.operands[1]),
                self.operand(&self.operands[2]),
            ) {
                (Some(Operand::Reg(dest)), Some(Operand::Reg(src)), Some(Operand::Shift(0))) => {
                    // TODO: should this also test for dest == `zero`?
                    if dest == src {
                        return write!(f, "nop");
                    }
                }
                _ => {}
            }
        }

        fn display_operand(f: &mut fmt::Formatter, opcode: &Opcode, op: &Operand) -> fmt::Result {
            if *opcode == Opcode::LUI || *opcode == Opcode::AUIPC {
                // we show the immediate of LUI as an unsigned integer, because the docs say so.
                if let Operand::Imm(imm) = op {
                    return write!(f, "{:#x}", (*imm as u32) >> 12);
                }
            } else if let Operand::Imm(imm) = op {
                if *imm < 0 {
                    return write!(f, "-{:#x}", imm.wrapping_neg());
                } else {
                    return write!(f, "{:#x}", imm);
                }
            }

            write!(f, "{}", op)
        }
        write!(f, "{}", self.opcode)?;

        let mut wrote_operand = false;
        for op in self.operands.iter() {
            match self.operand(op) {
                Some(op) => {
                    if wrote_operand {
                        write!(f, ", ")?;
                    } else {
                        write!(f, " ")?;
                        wrote_operand = true;
                    }
                    display_operand(f, &self.opcode, &op)?;
                }
                _ => {
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Reg(reg) => {
                let name = REG_NAMES[*reg as usize];
                write!(f, "{}", name)
            }
            Operand::Imm(imm) => {
                write!(f, "{:#x}", imm)
            }
            Operand::BaseOffset(reg, offs) => {
                let name = REG_NAMES[*reg as usize];
                if *offs == 0 {
                    write!(f, "({})", name)
                } else {
                    if *offs < 0 {
                        write!(f, "-{:#x}({})", offs.wrapping_neg(), name)
                    } else {
                        write!(f, "{:#x}({})", offs, name)
                    }
                }
            }
            Operand::Shift(sa) => {
                write!(f, "{:#x}", sa)
            }
            Operand::LongImm(imm) => {
                write!(f, "{:#x}", imm)
            }
            Operand::JOffset(offs) => {
                if *offs < 0 {
                    write!(f, "$-{:#x}", offs.wrapping_neg())
                } else {
                    write!(f, "$+{:#x}", offs)
                }
            }
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Invalid => panic!("attempt to disassemble invalid opcode"),
            Opcode::LUI => write!(f, "lui"),
            Opcode::AUIPC => write!(f, "auipc"),
            Opcode::JAL => write!(f, "jal"),
            Opcode::JALR => write!(f, "jalr"),
            Opcode::BEQ => write!(f, "beq"),
            Opcode::BNE => write!(f, "bne"),
            Opcode::BLT => write!(f, "blt"),
            Opcode::BGE => write!(f, "bge"),
            Opcode::BLTU => write!(f, "bltu"),
            Opcode::BGEU => write!(f, "bgeu"),
            Opcode::LB => write!(f, "lb"),
            Opcode::LH => write!(f, "lh"),
            Opcode::LW => write!(f, "lw"),
            Opcode::LBU => write!(f, "lbu"),
            Opcode::LHU => write!(f, "lhu"),
            Opcode::SB => write!(f, "sb"),
            Opcode::SH => write!(f, "sh"),
            Opcode::SW => write!(f, "sw"),
            Opcode::ADDI => write!(f, "addi"),
            Opcode::SLTI => write!(f, "slti"),
            Opcode::SLTIU => write!(f, "sltiu"),
            Opcode::XORI => write!(f, "xori"),
            Opcode::ORI => write!(f, "ori"),
            Opcode::ANDI => write!(f, "andi"),
            Opcode::SLLI => write!(f, "slli"),
            Opcode::SRLI => write!(f, "srli"),
            Opcode::SRAI => write!(f, "srai"),
            Opcode::ADD => write!(f, "add"),
            Opcode::SUB => write!(f, "sub"),
            Opcode::SLL => write!(f, "sll"),
            Opcode::SLT => write!(f, "slt"),
            Opcode::SLTU => write!(f, "sltu"),
            Opcode::XOR => write!(f, "xor"),
            Opcode::SRL => write!(f, "srl"),
            Opcode::SRA => write!(f, "sra"),
            Opcode::OR => write!(f, "or"),
            Opcode::AND => write!(f, "and"),
            Opcode::FENCE => write!(f, "fence"),
            Opcode::ECALL => write!(f, "ecall"),
            Opcode::EBREAK => write!(f, "ebreak"),
            Opcode::LWU => write!(f, "lwu"),
            Opcode::LD => write!(f, "ld"),
            Opcode::SD => write!(f, "sd"),
            Opcode::ADDIW => write!(f, "addiw"),
            Opcode::SLLIW => write!(f, "slliw"),
            Opcode::SRLIW => write!(f, "srliw"),
            Opcode::SRAIW => write!(f, "sraiw"),
            Opcode::ADDW => write!(f, "addw"),
            Opcode::SUBW => write!(f, "subw"),
            Opcode::SLLW => write!(f, "sllw"),
            Opcode::SRLW => write!(f, "srlw"),
            Opcode::SRAW => write!(f, "sraw"),
        }
    }
}
