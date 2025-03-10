use criterion::{black_box, criterion_group, criterion_main, Criterion};
use io::packet::Packet;

fn benchmark_packet_p1(c: &mut Criterion) {
    c.bench_function("packet_p1", |b| {
        let mut packet = Packet::new(10000);
        b.iter(|| {
            for i in 0..10000 {
                packet.p1(black_box(i % 256));
            }
        });
    });
}

fn benchmark_packet_p2(c: &mut Criterion) {
    c.bench_function("packet_p2", |b| {
        let mut packet = Packet::new(10000);
        b.iter(|| {
            for i in 0..5000 {
                packet.p2(black_box(i % 65536));
            }
        });
    });
}

fn benchmark_packet_g1(c: &mut Criterion) {
    c.bench_function("packet_g1", |b| {
        let mut packet = Packet::from(vec![0; 10000]);
        b.iter(|| {
            for _ in 0..10000 {
                black_box(packet.g1());
            }
        });
    });
}

criterion_group!(
    benches,
    benchmark_packet_p1,
    benchmark_packet_p2,
    benchmark_packet_g1
);
criterion_main!(benches);
