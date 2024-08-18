pub trait Pack {
    fn pack_into(&self, buf: &mut Vec<u8>);

    fn pack(&self) -> Vec<u8> {
        let mut buf = vec![];
        self.pack_into(&mut buf);
        buf
    }
}

impl Pack for &[u8] {
    fn pack_into(&self, buf: &mut Vec<u8>) {
        buf.extend(self.iter());
    }
}

impl Pack for &str {
    fn pack_into(&self, buf: &mut Vec<u8>) {
        buf.extend(self.bytes());
    }
}

impl Pack for Vec<u8> {
    fn pack_into(&self, buf: &mut Vec<u8>) {
        buf.extend(self);
    }
}

impl Pack for String {
    fn pack_into(&self, buf: &mut Vec<u8>) {
        buf.extend(self.bytes());
    }
}

impl<P: Pack> Pack for Vec<P> {
    fn pack_into(&self, buf: &mut Vec<u8>) {
        for item in self {
            item.pack_into(buf)
        }
    }
}

impl<const N: usize> Pack for [u8; N] {
    fn pack_into(&self, buf: &mut Vec<u8>) {
        buf.extend(self)
    }
}

impl<const N: usize, P: Pack> Pack for [P; N] {
    fn pack_into(&self, buf: &mut Vec<u8>) {
        for item in self {
            item.pack_into(buf)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_byte_slice() {
        let mut buf = vec![1, 2, 3];
        let data = &[4, 5];
        data.pack_into(&mut buf);
        assert_eq!(buf, vec![1, 2, 3, 4, 5])
    }
}
