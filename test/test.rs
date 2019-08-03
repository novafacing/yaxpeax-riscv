extern crate yaxpeax_arch;
extern crate yaxpeax_mips;

use yaxpeax_arch::{Arch, Decoder, U8Reader};
use yaxpeax_mips::{Instruction, RISCV}; //, Opcode};

#[allow(dead_code)]
fn test_decode(data: [u8; 4], expected: Instruction) {
    let mut reader = U8Reader::new(&data[..]);
    let instr = <RISCV as Arch>::Decoder::default()
        .decode(&mut reader)
        .unwrap();
    assert!(
        instr == expected,
        "decode error for {:02x}{:02x}{:02x}{:02x}:\n  decoded: {:?}\n expected: {:?}\n",
        data[0],
        data[1],
        data[2],
        data[3],
        instr,
        expected
    );
}

fn test_display(data: [u8; 4], expected: &'static str) {
    let mut reader = U8Reader::new(&data[..]);
    let instr = <RISCV as Arch>::Decoder::default()
        .decode(&mut reader)
        .unwrap();
    let text = format!("{}", instr);
    assert!(
        text == expected,
        "display error for {:02x}{:02x}{:02x}{:02x}:\n  decoded: {:?}\n displayed: {}\n expected: {}\n",
        data[0], data[1], data[2], data[3],
        instr,
        text, expected
    );
}

#[test]
fn test_arithmetic() {
    test_display([0x13, 0x06, 0xc6, 0xfb], "addi a2, a2, -0x44");
    test_display([0x13, 0x01, 0x01, 0xed], "addi sp, sp, -0x130");
    test_display([0x97, 0x31, 0x88, 0x02], "auipc gp, 0x2883");
    test_display([0x33, 0x65, 0xb5, 0x00], "or a0, a0, a1");
    test_display([0x13, 0x56, 0xc5, 0x00], "srli a2, a0, 0xc");
    test_display([0x13, 0x97, 0x27, 0x00], "slli a4, a5, 0x2");
}
#[test]
fn test_br() {
    test_display([0x63, 0x1a, 0xf7, 0x00], "bne a4, a5, $+0x14");
    test_display([0x63, 0x08, 0xf7, 0x00], "beq a4, a5, $+0x10");
    test_display([0xe3, 0x96, 0x07, 0xfe], "bne a5, zero, $-0x14");
    // test_display([0xef, 0x20, 0x40, 0x59], "jal $+0x29f0");
}
#[test]
#[ignore]
fn test_cmp() {
    // slt
    // sltu
    // slti
    test_display([0x07, 0x00, 0xe1, 0x2d], "sltiu at, t7, 0x7");
}
#[test]
fn test_mov() {
    test_display([0xb7, 0x05, 0x00, 0xc0], "lui a1, 0xc0000");
    test_display([0xb7, 0xc7, 0x29, 0x00], "lui a5, 0x29c");
    test_display([0x13, 0x85, 0x07, 0x00], "mv a0, a5");
    test_display([0x23, 0x20, 0xa1, 0x18], "sw a0, 0x180(sp)");
    test_display([0x03, 0x26, 0x01, 0x18], "lw a2, 0x180(sp)");
}
#[test]
fn test_misc() {
    test_display([0x13, 0x00, 0x00, 0x00], "nop");
}
