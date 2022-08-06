////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{HeadData, JceFieldErr, JceType, JString, STRING1, STRING4};

impl JceType<JString> for JString {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        let l = self.len();
        if l <= 255 {
            HeadData::new(STRING1, tag).format(b, l);
            b.put_u8(l as u8);
        } else {
            HeadData::new(STRING4, tag).format(b, l);
            b.put_i32(l as i32);
        };
        b.put_slice(self.as_ref());
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> Result<JString, JceFieldErr> {
        let len = match r#type {
            STRING1 => Ok(b.get_u8() as usize),
            STRING4 => Ok(b.get_i32() as usize),
            _ => Err(JceFieldErr { expectation: STRING1, result: r#type }),
        }?;
        let r = String::from_utf8(b.slice(..len).to_vec());
        b.advance(len);

        match r { // 默认情况下以 utf-8 通讯
            Ok(r) => Ok(r),
            Err(_) => Err(JceFieldErr { expectation: 255, result: 102 })
        }
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
            JString::from_bytes(&mut Bytes::from(vec![6, 229, 165, 189, 232, 128, 182]), STRING1).unwrap(),
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
            JString::from_bytes(&mut Bytes::from(vec![0, 0, 1, 17, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129]), STRING4).unwrap(),
            JString::from("好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！"),
        );
    }
}
