#[cfg(feature = "use-serde")]
use serde::{Deserialize, Serialize};

use yaxpeax_arch::{AddressDiff, Arch, Decoder, LengthedInstruction, Reader, StandardDecodeError};

mod display;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    word: u32,
    operands: [OperandSpec; 3],
    opcode: Opcode,
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            word: 0,
            operands: [
                OperandSpec::Nothing,
                OperandSpec::Nothing,
                OperandSpec::Nothing,
            ],
            opcode: Opcode::Invalid,
        }
    }
}

impl Instruction {
    fn field(&self, f: FieldSpec) -> u32 {
        match f {
            FieldSpec::Rs1 => (self.word >> 15) & 0b11111,
            FieldSpec::Rs2 => (self.word >> 20) & 0b11111,
            FieldSpec::Rd => (self.word >> 7) & 0b11111,
            FieldSpec::Shamt => (self.word >> 20) & 0b11111,
            FieldSpec::Imm12I => (self.word as i32 >> 20) as u32,
            FieldSpec::Imm12S => {
                let a = ((self.word >> 7) & 0b11111) as i32;
                let b = ((self.word >> 20) & !0b11111) as i32;

                (a | b) as u32
            }
            FieldSpec::Imm12B => {
                let a = (self.word as i32 >> 7) & 0b11110;
                let b = (self.word as i32 >> 20) & !0b01111;
                let c =
                    (((self.word & 0x8000_0000) >> 19) | ((self.word & 0x0000_0080) << 4)) as i32;

                (a | b | c) as u32
            }
            FieldSpec::Imm20U => (self.word & 0xFFFF_F000u32) as u32,
            FieldSpec::Imm20J => todo!(),
        }
    }

    fn operand(&self, op: &OperandSpec) -> Option<Operand> {
        match op {
            OperandSpec::Nothing => None,
            OperandSpec::Rs1 => Some(Operand::Reg(self.field(FieldSpec::Rs1) as u8)),
            OperandSpec::Rs2 => Some(Operand::Reg(self.field(FieldSpec::Rs2) as u8)),
            OperandSpec::Rd => Some(Operand::Reg(self.field(FieldSpec::Rd) as u8)),
            OperandSpec::Shamt => Some(Operand::Shift(self.field(FieldSpec::Imm12I) as u8)),
            OperandSpec::Imm12I => Some(Operand::Imm(self.field(FieldSpec::Imm12I) as i32)),
            OperandSpec::Imm12S => Some(Operand::Imm(self.field(FieldSpec::Imm12S) as i32)),
            OperandSpec::Imm12B => Some(Operand::JOffset(self.field(FieldSpec::Imm12B) as i32)),
            OperandSpec::Imm20U => Some(Operand::Imm(self.field(FieldSpec::Imm20U) as i32)),
            OperandSpec::Imm20J => todo!(),
            OperandSpec::BaseOffsetRs1I => Some(Operand::BaseOffset(
                self.field(FieldSpec::Rs1) as u8,
                self.field(FieldSpec::Imm12I) as i16,
            )),
            OperandSpec::BaseOffsetRs1S => Some(Operand::BaseOffset(
                self.field(FieldSpec::Rs1) as u8,
                self.field(FieldSpec::Imm12S) as i16,
            )),
        }
    }

    pub fn opcode(&self) -> &Opcode {
        &self.opcode
    }

    pub fn operands(&self) -> &[Operand; 3] {
        &self.operands
    }

    pub fn word(&self) -> &u32 {
        &self.word
    }
}

impl yaxpeax_arch::Instruction for Instruction {
    fn well_defined(&self) -> bool {
        // TODO: this is inaccurate
        true
    }
}

impl LengthedInstruction for Instruction {
    type Unit = AddressDiff<u32>;
    fn min_size() -> Self::Unit {
        AddressDiff::from_const(4)
    }

    fn len(&self) -> Self::Unit {
        AddressDiff::from_const(4)
    }
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Invalid,

    // RV32I Base Instruction Set
    LUI,
    AUIPC,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    FENCE,
    ECALL,
    EBREAK,

    // RV64I Base Instruction Set
    LWU,
    LD,
    SD,
    // SLLI,
    // SRLI,
    // SRAI,
    ADDIW,
    SLLIW,
    SRLIW,
    SRAIW,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldSpec {
    Rs1,
    Rs2,
    Rd,
    /// Shift amount (occupies rs2 slot)
    Shamt,
    /// I-type 12-bit immediate
    Imm12I,
    /// S-type 12-bit immediate
    Imm12S,
    /// B-type 12-bit immediate
    Imm12B,
    /// U-type 20-bit immediate
    Imm20U,
    /// J-type 20-bit immediate
    Imm20J,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperandSpec {
    Nothing = 0,
    Rs1,
    Rs2,
    Rd,
    /// Base offset Rs1+Imm12I
    BaseOffsetRs1I,
    /// Base offset Rs1+Imm12S
    BaseOffsetRs1S,
    /// Shift amount (occupies rs2 slot)
    Shamt,
    /// I-type 12-bit immediate
    Imm12I,
    /// S-type 12-bit immediate
    Imm12S,
    /// B-type 12-bit immediate
    Imm12B,
    /// U-type 20-bit immediate
    Imm20U,
    /// J-type 20-bit immediate
    Imm20J,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    /// GPR operand
    Reg(u8),
    /// Immediate
    Imm(i32),
    /// Base(offset)
    BaseOffset(u8, i16),
    Shift(u8),
    LongImm(u32),
    JOffset(i32),
}

#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct RISCV;

impl Arch for RISCV {
    type Address = u32;
    type Word = yaxpeax_arch::U16le;
    type Instruction = Instruction;
    type DecodeError = StandardDecodeError;
    type Decoder = RiscVDecoder;
    type Operand = Operand;
}

#[derive(Default, Debug)]
pub struct RiscVDecoder {}

impl RiscVDecoder {
    fn decode32_into(
        &self,
        instruction: &mut Instruction,
        word: u32,
    ) -> Result<(), <RISCV as Arch>::DecodeError> {
        let opc = word & 0b111_1111;

        match opc {
            0b011_0111 => {
                instruction.opcode = Opcode::LUI;
                instruction.operands = [OperandSpec::Rd, OperandSpec::Imm20U, OperandSpec::Nothing];
            }
            0b001_0111 => {
                instruction.opcode = Opcode::AUIPC;
                instruction.operands = [OperandSpec::Rd, OperandSpec::Imm20U, OperandSpec::Nothing];
            }
            0b110_1111 => {
                instruction.opcode = Opcode::JAL;
                instruction.operands = [OperandSpec::Rd, OperandSpec::Imm20J, OperandSpec::Nothing];
            }
            0b110_0111 => {
                instruction.opcode = Opcode::JALR;
                instruction.operands = [OperandSpec::Rd, OperandSpec::Rs1, OperandSpec::Imm12I];
            }
            0b110_0011 => {
                // Bxx opcode group
                let funct3 = (word >> 12) & 0b111;

                instruction.operands = [OperandSpec::Rs1, OperandSpec::Rs2, OperandSpec::Imm12B];
                match funct3 {
                    0b000 => instruction.opcode = Opcode::BEQ,
                    0b001 => instruction.opcode = Opcode::BNE,
                    0b100 => instruction.opcode = Opcode::BLT,
                    0b101 => instruction.opcode = Opcode::BGE,
                    0b110 => instruction.opcode = Opcode::BLTU,
                    0b111 => instruction.opcode = Opcode::BGEU,
                    _ => Err(StandardDecodeError::InvalidOpcode)?,
                }
            }
            0b000_0011 => {
                // Lx opcode group
                let funct3 = (word >> 12) & 0b111;

                instruction.operands = [
                    OperandSpec::Rd,
                    OperandSpec::BaseOffsetRs1I,
                    OperandSpec::Nothing,
                ];
                match funct3 {
                    0b000 => instruction.opcode = Opcode::LB,
                    0b001 => instruction.opcode = Opcode::LH,
                    0b010 => instruction.opcode = Opcode::LW,
                    0b100 => instruction.opcode = Opcode::LBU,
                    0b101 => instruction.opcode = Opcode::LHU,
                    _ => Err(StandardDecodeError::InvalidOpcode)?,
                }
            }
            0b010_0011 => {
                // Sx opcode group
                let funct3 = (word >> 12) & 0b111;

                instruction.operands = [
                    OperandSpec::Rs2,
                    OperandSpec::BaseOffsetRs1S,
                    OperandSpec::Nothing,
                ];
                match funct3 {
                    0b000 => instruction.opcode = Opcode::SB,
                    0b001 => instruction.opcode = Opcode::SH,
                    0b010 => instruction.opcode = Opcode::SW,
                    _ => Err(StandardDecodeError::InvalidOpcode)?,
                }
            }
            0b001_0011 => {
                // ALU immediate opcode group
                let funct3 = (word >> 12) & 0b111;
                let funct7 = (word >> 25) & 0b111_1111;

                instruction.operands = [OperandSpec::Rd, OperandSpec::Rs1, OperandSpec::Imm12I];
                match funct3 {
                    0b000 => instruction.opcode = Opcode::ADDI,
                    0b010 => instruction.opcode = Opcode::SLTI,
                    0b011 => instruction.opcode = Opcode::SLTIU,
                    0b100 => instruction.opcode = Opcode::XORI,
                    0b110 => instruction.opcode = Opcode::ORI,
                    0b111 => instruction.opcode = Opcode::ANDI,
                    0b001 | 0b101 => {
                        instruction.operands =
                            [OperandSpec::Rd, OperandSpec::Rs1, OperandSpec::Shamt];

                        match funct3 {
                            0b001 => instruction.opcode = Opcode::SLLI,
                            0b101 => match funct7 {
                                0b000_0000 => instruction.opcode = Opcode::SRLI,
                                0b010_0000 => instruction.opcode = Opcode::SRAI,
                                _ => Err(StandardDecodeError::InvalidOpcode)?,
                            },
                            _ => Err(StandardDecodeError::InvalidOpcode)?,
                        };
                    }
                    _ => Err(StandardDecodeError::InvalidOpcode)?,
                }
            }
            0b011_0011 => {
                // ALU opcode group
                let funct3 = (word >> 12) & 0b111;
                let funct7 = (word >> 25) & 0b111_1111;

                instruction.operands = [OperandSpec::Rd, OperandSpec::Rs1, OperandSpec::Rs2];
                match funct3 {
                    0b000 => match funct7 {
                        0b000_0000 => instruction.opcode = Opcode::ADD,
                        0b010_0000 => instruction.opcode = Opcode::SUB,
                        _ => Err(StandardDecodeError::InvalidOpcode)?,
                    },
                    0b001 => instruction.opcode = Opcode::SLL,
                    0b010 => instruction.opcode = Opcode::SLT,
                    0b011 => instruction.opcode = Opcode::SLTU,
                    0b100 => instruction.opcode = Opcode::XOR,
                    0b101 => match funct7 {
                        0b000_0000 => instruction.opcode = Opcode::SRL,
                        0b010_0000 => instruction.opcode = Opcode::SRA,
                        _ => Err(StandardDecodeError::InvalidOpcode)?,
                    },
                    0b110 => instruction.opcode = Opcode::OR,
                    0b111 => instruction.opcode = Opcode::AND,
                    _ => Err(StandardDecodeError::InvalidOpcode)?,
                }
            }
            0b000_1111 => {
                // FENCE opcode group
                todo!()
            }
            0b111_0011 => match (opc >> 20) & 0b1111_1111_1111 {
                0b0000_0000_0000 => instruction.opcode = Opcode::ECALL,
                0b0000_0000_0001 => instruction.opcode = Opcode::EBREAK,
                _ => Err(StandardDecodeError::InvalidOpcode)?,
            },
            _ => Err(StandardDecodeError::InvalidOpcode)?,
        }

        Ok(())
    }
}

impl Decoder<RISCV> for RiscVDecoder {
    fn decode_into<T: Reader<<RISCV as Arch>::Address, <RISCV as Arch>::Word>>(
        &self,
        instruction: &mut Instruction,
        words: &mut T,
    ) -> Result<(), <RISCV as Arch>::DecodeError> {
        let word0 = words.next()?.0;

        // Determine the instruction length first.
        // Also note: RISC-V instructions are _always_ little-endian.
        let opc = word0 & 0b111_1111;
        match opc {
            opc if opc & 0b11 != 0b11 => {
                // 16-bit instruction set space.
                // Unimplemented.
                Err(StandardDecodeError::InvalidOpcode)
            }
            opc if opc & 0b1_1100 != 0b1_1100 => {
                // 32-bit instruction set space.
                let word1 = words.next()?.0;
                let word = ((word1 as u32) << 16u32) | word0 as u32;

                instruction.word = word;
                self.decode32_into(instruction, word)
            }
            _ => Err(StandardDecodeError::InvalidOpcode),
        }?;

        Ok(())
    }
}
