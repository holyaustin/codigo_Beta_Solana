use crate::rust_utils::*;
use solana_program::pubkey::Pubkey;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_pack_unpack_option_pubkey() {
        let big: &mut [u8] = &mut [0; 180];
        let my_pubkey = Pubkey::new_unique();
        let op_pubkey: &Option<Pubkey> = &Some(my_pubkey);

        let r: &mut [u8] = pack_option(
            big,
            |dst: &mut [u8], data: Pubkey| -> &mut [u8] {
                return pack_pubkey(dst, *(&data));
            },
            (*op_pubkey).clone(),
            Pubkey::new_from_array([0; 32]),
        );

        assert_eq!(r.len(), big.len() - 33);

        let (ptr, result) = state_unpack_option(
            big,
            |dst: &[u8]| -> (&[u8], Pubkey) {
                return unpack_pubkey(dst);
            },
            32 as usize,
        );

        assert_eq!(result, Some(my_pubkey));
        assert_eq!(ptr.len(), big.len() - 33);
    }

    #[test]
    fn it_unpack_option_string_state() {
        let buffer: &[u8] = &[
            0x01, 0x05, 0x00, 0x00, 0x00, 0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12,
            0x13, 0x14, 0x01, 0x05, 0x00, 0x00, 0x00, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x00, 0x00,
            0x00, 0x00, 0x00,
        ];
        let mut op_string: Option<String>;
        let string_field_size = 14; // 4 for size + 10 for string
        let optional_flag_size = 1;

        let mut new_buf: &[u8] = buffer;
        (new_buf, op_string) = state_unpack_option(
            new_buf,
            |dst: &[u8]| -> (&[u8], String) {
                return state_unpack_string(dst, string_field_size);
            },
            string_field_size as usize,
        );

        let mut buf_length = new_buf.len();
        assert_eq!(op_string, Some("Lorem".to_string()));
        assert_eq!(
            buf_length,
            buffer.len() - string_field_size - optional_flag_size
        );

        (new_buf, op_string) = state_unpack_option(
            new_buf,
            |dst: &[u8]| -> (&[u8], String) {
                return state_unpack_string(dst, string_field_size);
            },
            string_field_size as usize,
        );

        let mut previous_buf_length = buf_length;
        buf_length = new_buf.len();

        assert_eq!(op_string, None);
        assert_eq!(
            buf_length,
            previous_buf_length - string_field_size - optional_flag_size
        );

        (new_buf, op_string) = state_unpack_option(
            new_buf,
            |dst: &[u8]| -> (&[u8], String) {
                return state_unpack_string(dst, string_field_size);
            },
            string_field_size as usize,
        );

        previous_buf_length = buf_length;
        buf_length = new_buf.len();

        assert_eq!(op_string, Some("ipsum".to_string()));
        assert_eq!(
            buf_length,
            previous_buf_length - string_field_size - optional_flag_size
        );
        assert_eq!(buf_length, 0);
    }

    #[test]
    fn it_unpack_option_string_instruction() {
        let client_buffer: &[u8] = &[
            0x01, 0x05, 0x00, 0x00, 0x00, 0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x00,
        ];

        let string_size = 5;
        let mut op_string: Option<String>;
        let mut new_buf: &[u8] = client_buffer;
        (new_buf, op_string) =
            instructions_unpack_option(new_buf, |dst: &[u8]| -> (&[u8], String) {
                return instructions_unpack_string(dst);
            });

        assert_eq!(op_string, Some("Lorem".to_string()));
        assert_eq!(new_buf.len(), client_buffer.len() - (string_size + 4) - 1);

        (new_buf, op_string) =
            instructions_unpack_option(new_buf, |dst: &[u8]| -> (&[u8], String) {
                return instructions_unpack_string(dst);
            });

        assert_eq!(op_string, None);
        assert_eq!(new_buf.len(), 0);
    }

    #[test]
    fn it_pack_option_string() {
        let string_field_size = 24;
        let option_string_to_pack = Some("Lorem".to_string());
        let buffer: &mut [u8] = &mut [0; 30];
        let mut new_dst: &mut [u8] = buffer;

        new_dst = pack_option(
            new_dst,
            |dst: &mut [u8], data: String| -> &mut [u8] {
                return pack_string(dst, (&data).clone(), string_field_size);
            },
            option_string_to_pack.clone(),
            "".to_string(),
        );

        assert_eq!(new_dst.len(), buffer.len() - string_field_size - 1);
        assert_eq!(buffer[0], 0x01);
        assert_eq!(buffer[1..5], [0x05, 0x00, 0x00, 0x00]);
        assert_eq!(buffer[5..10], *("Lorem".to_string().as_bytes()));
    }

    #[test]
    fn it_pack_unpack_bool() {
        let big: &mut [u8] = &mut [0; 180];

        let a = true;
        let b = false;

        let r: &mut [u8] = pack_bool(big, a);
        pack_bool(r, b);
        let (ptr2, value_a) = unpack_bool(big);
        let (_, value_b) = unpack_bool(ptr2);
        assert_eq!(value_a, a);
        assert_eq!(value_b, b);
    }

    #[test]
    fn it_pack_unpack_u128() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::u128(..);
                let b = fastrand::u128(..);

                let r: &mut [u8] = pack_u128(big, a);
                pack_u128(r, b);
                let (ptr2, value_a) = unpack_u128(big);
                let (_, value_b) = unpack_u128(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_i128() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::i128(..);
                let b = fastrand::i128(..);

                let r: &mut [u8] = pack_i128(big, a);
                pack_i128(r, b);
                let (ptr2, value_a) = unpack_i128(big);
                let (_, value_b) = unpack_i128(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_u64() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::u64(..);
                let b = fastrand::u64(..);

                let r: &mut [u8] = pack_u64(big, a);
                pack_u64(r, b);
                let (ptr2, value_a) = unpack_u64(big);
                let (_, value_b) = unpack_u64(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_i64() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::i64(..);
                let b = fastrand::i64(..);

                let r: &mut [u8] = pack_i64(big, a);
                pack_i64(r, b);
                let (ptr2, value_a) = unpack_i64(big);
                let (_, value_b) = unpack_i64(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_u32() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::u32(..);
                let b = fastrand::u32(..);

                let r: &mut [u8] = pack_u32(big, a);
                pack_u32(r, b);
                let (ptr2, value_a) = unpack_u32(big);
                let (_, value_b) = unpack_u32(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_i32() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::i32(..);
                let b = fastrand::i32(..);

                let r: &mut [u8] = pack_i32(big, a);
                pack_i32(r, b);
                let (ptr2, value_a) = unpack_i32(big);
                let (_, value_b) = unpack_i32(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_u16() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::u16(..);
                let b = fastrand::u16(..);

                let r: &mut [u8] = pack_u16(big, a);
                pack_u16(r, b);
                let (ptr2, value_a) = unpack_u16(big);
                let (_, value_b) = unpack_u16(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_i16() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::i16(..);
                let b = fastrand::i16(..);

                let r: &mut [u8] = pack_i16(big, a);
                pack_i16(r, b);
                let (ptr2, value_a) = unpack_i16(big);
                let (_, value_b) = unpack_i16(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_u8() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::u8(..);
                let b = fastrand::u8(..);

                let r: &mut [u8] = pack_u8(big, a);
                pack_u8(r, b);
                let (ptr2, value_a) = unpack_u8(big);
                let (_, value_b) = unpack_u8(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_i8() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::i8(..);
                let b = fastrand::i8(..);

                let r: &mut [u8] = pack_i8(big, a);
                pack_i8(r, b);
                let (ptr2, value_a) = unpack_i8(big);
                let (_, value_b) = unpack_i8(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_string() {
        let big: &mut [u8] = &mut [0; 180];

        let a = String::from("something_not that small");
        let b = String::from("not");

        let r: &mut [u8] = pack_string(big, a.clone(), 30);
        pack_string(r, b.clone(), 30);
        let (ptr2, value_a) = state_unpack_string(big, 30);
        let (_, value_b) = state_unpack_string(ptr2, 30);
        assert_eq!(value_a, a);
        assert_eq!(value_b, b);
    }

    #[test]
    fn it_pack_unpack_f32() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::f32();
                let b = fastrand::f32();

                let r: &mut [u8] = pack_f32(big, a);
                pack_f32(r, b);
                let (ptr2, value_a) = unpack_f32(big);
                let (_, value_b) = unpack_f32(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    fn it_pack_unpack_f64() {
        let big: &mut [u8] = &mut [0; 180];

        for _a in 0..255 {
            for _b in 0..255 {
                let a = fastrand::f64();
                let b = fastrand::f64();

                let r: &mut [u8] = pack_f64(big, a);
                pack_f64(r, b);
                let (ptr2, value_a) = unpack_f64(big);
                let (_, value_b) = unpack_f64(ptr2);
                assert_eq!(value_a, a);
                assert_eq!(value_b, b);
            }
        }
    }

    #[test]
    #[should_panic(expected = "String size is bigger than defined maximum capacity")]
    fn it_pack_invalid_string() {
        let big: &mut [u8] = &mut [0; 180];

        let a = String::from("Something longer than accepted capacity");

        pack_string(big, a.clone(), 30);
    }
}
