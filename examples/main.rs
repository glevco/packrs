use packrs::{pack, unpack, unpack3, BigEndian, LittleEndian, UnpackError};

fn main() -> Result<(), UnpackError> {
    let a = 1u8;
    let b = 2i16;
    let c = [1u8, 2, 3];
    let packed: Vec<u8> = pack!(BigEndian::from(a), LittleEndian::from(b), c);
    let buf = &mut &packed[..];
    let unpacked: Result<(BigEndian<u8>, LittleEndian<i16>, [u8; 3]), UnpackError> = unpack3(buf);
    let unpacked = unpacked.unwrap();

    assert_eq!((*unpacked.0, *unpacked.1, unpacked.2), (a, b, c));

    let buf = &mut &packed[..];
    let unpacked = unpack!(buf, (BigEndian<u8>, LittleEndian<i16>, [u8; 3]));
    assert_eq!((*unpacked.0, *unpacked.1, unpacked.2), (a, b, c));

    Ok(())
}
