use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

use yaxpeax_arch::{Arch, Decoder, U8Reader};
use yaxpeax_mips::{Instruction, RISCV}; //, Opcode};

fn decode_single(insn: u32) -> Instruction {
    let insn = insn.to_le_bytes();

    let mut reader = U8Reader::new(&insn[..]);
    <RISCV as Arch>::Decoder::default()
        .decode(&mut reader)
        .unwrap()
}

fn decode_multi(insns: &[u32]) {
    for insn in insns {
        decode_single(*insn);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");
    group.throughput(Throughput::Elements(1));

    group.bench_function("decode srli", |b| {
        b.iter(|| {
            decode_single(black_box(0x00c55613));
        })
    });
    
    group.throughput(Throughput::Elements(2));
    group.bench_function("decode sw/srli", |b| {
        b.iter(|| {
            decode_single(black_box(0x00c55613));
            decode_single(black_box(0x02a12423));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
