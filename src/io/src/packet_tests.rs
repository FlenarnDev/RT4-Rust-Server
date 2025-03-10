use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crate::packet::Packet;

#[test]
fn test_packet_creation() {
    let packet = Packet::new(10);
    assert_eq!(packet.len(), 0);
    assert_eq!(packet.remaining(), 0);
}

#[test]
fn test_packet_from_vec() {
    let data = vec![1, 2, 3, 4];
    let packet = Packet::from(data.clone());
    assert_eq!(packet.data, data);
    assert_eq!(packet.len(), 4);
    assert_eq!(packet.remaining(), 4);
}

#[test]
fn test_p1() {
    let mut packet = Packet::new(10);
    packet.p1(255);
    assert_eq!(packet.data, vec![255]);
}

#[test]
fn test_p2() {
    let mut packet = Packet::new(10);
    packet.p2(513);
    assert_eq!(packet.data, vec![2, 1]);
}

#[test]
fn test_p4() {
    let mut packet = Packet::new(10);
    packet.p4(16909060);
    assert_eq!(packet.data, vec![1, 2, 3, 4]);
}

#[test]
fn test_g1() {
    let mut packet = Packet::from(vec![42]);
    assert_eq!(packet.g1(), 42);
}

#[test]
fn test_g2() {
    let mut packet = Packet::from(vec![0x12, 0x34]);
    assert_eq!(packet.g2(), 0x1234);
}

#[test]
fn test_g4() {
    let mut packet = Packet::from(vec![0x01, 0x02, 0x03, 0x04]);
    assert_eq!(packet.g4(), 0x01020304);
}