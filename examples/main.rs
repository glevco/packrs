use packrs::{pack, pack_into, unpack, BigEndian, Pack, Unpack};

#[derive(Debug)]
struct Inner<'a> {
    byte_array: [u8; 2],
    byte_slice: &'a [u8],
}

#[derive(Debug)]
struct Outer<'a> {
    vec: Vec<Inner<'a>>,
}

impl<'a> Unpack<'a> for Inner<'a> {
    type Error = anyhow::Error;

    fn unpack(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
        unpack!(buf, {
            let byte_array: [u8; 2];
            let byte_slice_len: BigEndian<u8> as usize;
            let byte_slice: &[u8], byte_slice_len;
        });

        Ok(Inner {
            byte_array,
            byte_slice,
        })
    }
}

impl<'a> Pack for Inner<'a> {
    fn pack_into(&self, buf: &mut Vec<u8>) {
        pack_into!(
            buf,
            self.byte_array,
            BigEndian::from(self.byte_slice.len() as u8),
            self.byte_slice,
        );
    }
}

impl<'a> Unpack<'a> for Outer<'a> {
    type Error = anyhow::Error;

    fn unpack(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
        unpack!(buf, {
            let vec_len: BigEndian<u8> as usize;
            let vec: Vec<Inner>, vec_len;
        });

        Ok(Outer { vec })
    }
}

impl<'a> Pack for Outer<'a> {
    fn pack_into(&self, buf: &mut Vec<u8>) {
        BigEndian::from(self.vec.len() as u8).pack_into(buf);
        self.vec.pack_into(buf);
    }
}

fn main() -> anyhow::Result<()> {
    let value = Outer {
        vec: vec![Inner {
            byte_array: [1, 2],
            byte_slice: &[1, 2, 3],
        }],
    };
    println!("value: {:?}", value);
    let packed = pack!(value);
    println!("packed: {:?}", packed);
    let unpacked = Outer::unpack(&mut &packed[..])?;
    println!("unpacked: {:?}", unpacked);
    Ok(())
}
