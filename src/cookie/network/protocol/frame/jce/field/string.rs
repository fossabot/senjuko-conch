use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{HeadData, JceType, JString, STRING1, STRING4, TYPE_ERR};

impl JceType<JString> for JString {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        let l = self.len();
        if l <= 255 {
            HeadData::new(STRING1, tag, l as u32).format(b);
            b.put_u8(l as u8);
        } else {
            HeadData::new(STRING4, tag, l as u32).format(b);
            b.put_i32(l as i32);
        };
        b.put_slice(self.as_ref());
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> JString {
        let len = match r#type {
            STRING1 => b.get_u8() as usize,
            STRING4 => b.get_i32() as usize,
            _ => panic!("{}", TYPE_ERR),
        };
        let a = String::from_utf8(b.slice(..len).to_vec());
        b.advance(len);
        a.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{JceType, JString, STRING1, STRING4};

    #[test]
    fn to_bytes1() {
        let mut b = BytesMut::new();
        JString::from("好耶").to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![6, 6, 229, 165, 189, 232, 128, 182]);
    }

    #[test]
    fn from_bytes1() {
        assert_eq!(
            JString::from_bytes(&mut Bytes::from(vec![6, 229, 165, 189, 232, 128, 182]), STRING1),
            JString::from("好耶"),
        );
    }

    #[test]
    fn to_bytes2() {
        let mut b = BytesMut::new();
        JString::from("好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！").to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![7, 0, 0, 1, 17, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129]);
    }

    #[test]
    fn from_bytes2() {
        assert_eq!(
            JString::from_bytes(&mut Bytes::from(vec![0, 0, 1, 17, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129]), STRING4),
            JString::from("好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！"),
        );
    }
}
