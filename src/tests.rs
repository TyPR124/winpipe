use crate as winpipe;
use std::io::{Read, Write};

#[test]
fn basic() {
    let (mut tx, mut rx) = winpipe::unnamed().unwrap();
    let bytes = [1, 2, 3, 4];
    let mut bytes2 = [0, 0, 0, 0];
    assert_eq!(4, tx.write(&bytes[..]).unwrap());
    assert_eq!(4, rx.available_bytes());
    assert_eq!(4, rx.read(&mut bytes2[..]).unwrap());
    assert_eq!(bytes, bytes2);
}
#[test]
fn read_single_bytes() {
    let (mut tx, mut rx) = winpipe::unnamed().unwrap();
    let bytes = [1, 2, 3, 4];
    let mut bytes2 = [0, 0, 0, 0];
    assert_eq!(4, tx.write(&bytes[..]).unwrap());
    for i in 0..4 {
        assert_eq!(4-i as u32, rx.available_bytes());
        assert_eq!(1, rx.read(&mut bytes2[i..=i]).unwrap());
    }
    assert_eq!(0, rx.available_bytes());
    assert_eq!(bytes, bytes2);
}
#[test]
fn peek() {
    let (mut tx, mut rx) = winpipe::unnamed().unwrap();
    let bytes = [1, 2, 3, 4];
    let mut bytes2 = [0, 0, 0, 0];
    assert_eq!(4, tx.write(&bytes[..]).unwrap());
    for i in 0..=4 {
        assert_eq!(i, rx.peek(&mut bytes2[..i]).unwrap());
        assert_eq!(&bytes[..i], &bytes2[..i]);
        bytes2 = [0, 0, 0, 0];
    }
    assert_eq!(4, rx.available_bytes());
    assert_eq!(4, rx.read(&mut bytes2[..]).unwrap());
    assert_eq!(bytes, bytes2);
}
// #[test]
// fn duplicate_tx() {
//     let (tx, mut rx) = winpipe::unnamed().unwrap();
//     let mut tx = tx.try_clone().unwrap();
//     let bytes = [1, 2, 3, 4];
//     let mut bytes2 = [0, 0, 0, 0];
//     assert_eq!(4, tx.write(&bytes[..]).unwrap());
//     assert_eq!(4, rx.read(&mut bytes2[..]).unwrap());
//     assert_eq!(bytes, bytes2);
// }
// #[test]
// fn duplicate_rx() {
//     let (mut tx, rx) = winpipe::unnamed().unwrap();
//     let mut rx = rx.try_clone().unwrap();
//     let bytes = [1, 2, 3, 4];
//     let mut bytes2 = [0, 0, 0, 0];
//     assert_eq!(4, tx.write(&bytes[..]).unwrap());
//     assert_eq!(4, rx.read(&mut bytes2[..]).unwrap());
//     assert_eq!(bytes, bytes2);
// }
