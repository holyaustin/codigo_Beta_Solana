use solana_program::pubkey::Pubkey;

use std::ptr;

const VECTOR_OVERSIZE_PANIC: &str = "Vector size is bigger than defined maximum capacity";
const STRING_OVERSIZE_PANIC: &str = "String size is bigger than defined maximum capacity";

pub fn pad_string(s: &String, cap: usize) -> String {
    let mut s_padded: String = s.clone();
    while s_padded.len() < cap {
        s_padded = s_padded + "\0";
    }
    return s_padded;
}
type OffsetType = usize;
pub fn ignore_me<T>(_buf: &[u8], offset: OffsetType) -> (*const T, OffsetType) {
    return (ptr::null(), offset);
}
pub fn check_valid_bool_value(buf: &[u8]) {
    let (_, number) = unpack_u8(buf);
    if number > 1 {
        panic!("{:?}: Invalid boolean value", buf)
    }
}
// Generic Pack

// float
//float-simple-precision
pub fn pack_f32(buf: &mut [u8], data: f32) -> &mut [u8] {
    buf[0..4].copy_from_slice(&data.to_le_bytes());
    &mut buf[4..]
}

//float-double-precision
pub fn pack_f64(buf: &mut [u8], data: f64) -> &mut [u8] {
    buf[0..8].copy_from_slice(&data.to_le_bytes());
    &mut buf[8..]
}

pub fn pack_bool(buf: &mut [u8], data: bool) -> &mut [u8] {
    check_valid_bool_value(buf);
    buf[0] = data as u8;
    &mut buf[1..]
}
pub fn pack_u8(buf: &mut [u8], data: u8) -> &mut [u8] {
    buf[0] = data;
    &mut buf[1..]
}
pub fn pack_i8(buf: &mut [u8], data: i8) -> &mut [u8] {
    buf[0..1].copy_from_slice(&data.to_le_bytes());
    &mut buf[1..]
}
pub fn pack_u16(buf: &mut [u8], data: u16) -> &mut [u8] {
    buf[0..2].copy_from_slice(&data.to_le_bytes());
    &mut buf[2..]
}
pub fn pack_i16(buf: &mut [u8], data: i16) -> &mut [u8] {
    buf[0..2].copy_from_slice(&data.to_le_bytes());
    &mut buf[2..]
}
pub fn pack_u32(buf: &mut [u8], data: u32) -> &mut [u8] {
    buf[0..4].copy_from_slice(&data.to_le_bytes());
    &mut buf[4..]
}
pub fn pack_i32(buf: &mut [u8], data: i32) -> &mut [u8] {
    buf[0..4].copy_from_slice(&data.to_le_bytes());
    &mut buf[4..]
}
pub fn pack_u64(buf: &mut [u8], data: u64) -> &mut [u8] {
    buf[0..8].copy_from_slice(&data.to_le_bytes());
    &mut buf[8..]
}
pub fn pack_i64(buf: &mut [u8], data: i64) -> &mut [u8] {
    buf[0..8].copy_from_slice(&data.to_le_bytes());
    &mut buf[8..]
}
pub fn pack_u128(buf: &mut [u8], data: u128) -> &mut [u8] {
    buf[0..16].copy_from_slice(&data.to_le_bytes());
    &mut buf[16..]
}
pub fn pack_i128(buf: &mut [u8], data: i128) -> &mut [u8] {
    buf[0..16].copy_from_slice(&data.to_le_bytes());
    &mut buf[16..]
}
pub fn pack_string(buf: &mut [u8], data: String, field_size: usize) -> &mut [u8] {
    if data.len() > field_size - 4 {
        panic!("{}", STRING_OVERSIZE_PANIC);
    }

    buf[0..4].copy_from_slice(&(data.len() as u32).to_le_bytes());
    buf[4..4 + data.len()].copy_from_slice(&data.as_bytes());
    &mut buf[field_size..]
}
pub fn pack_option<F, T>(buf: &mut [u8], f: F, data: Option<T>, null_value: T) -> &mut [u8]
where
    F: Fn(&mut [u8], T) -> &mut [u8],
{
    let mut dst = pack_bool(buf, data.is_some());
    if data.is_some() {
        dst = f(dst, data.unwrap());
    } else {
        dst = f(dst, null_value);
    }
    return dst;
}

// Generic Unpack

pub fn unpack_f32(buf: &[u8]) -> (&[u8], f32) {
    let mut data_src: [u8; 4] = [0 as u8; 4];
    data_src.copy_from_slice(&buf[..4 as usize]);
    return (&buf[4..], f32::from_le_bytes(data_src));
}

pub fn unpack_f64(buf: &[u8]) -> (&[u8], f64) {
    let mut data_src: [u8; 8] = [0 as u8; 8];
    data_src.copy_from_slice(&buf[..8 as usize]);
    return (&buf[8..], f64::from_le_bytes(data_src));
}

pub fn unpack_bool(buf: &[u8]) -> (&[u8], bool) {
    check_valid_bool_value(buf);
    return (&buf[1..], buf[0] == 0x01);
}
pub fn unpack_u8(buf: &[u8]) -> (&[u8], u8) {
    let mut data_src: [u8; 1] = [0 as u8; 1];
    data_src.copy_from_slice(&buf[..1 as usize]);
    return (&buf[1..], u8::from_le_bytes(data_src));
}
pub fn unpack_i8(buf: &[u8]) -> (&[u8], i8) {
    let mut data_src: [u8; 1] = [0 as u8; 1];
    data_src.copy_from_slice(&buf[..1 as usize]);
    return (&buf[1..], i8::from_le_bytes(data_src));
}
pub fn unpack_u16(buf: &[u8]) -> (&[u8], u16) {
    let mut data_src: [u8; 2] = [0 as u8; 2];
    data_src.copy_from_slice(&buf[..2 as usize]);
    return (&buf[2..], u16::from_le_bytes(data_src));
}
pub fn unpack_i16(buf: &[u8]) -> (&[u8], i16) {
    let mut data_src: [u8; 2] = [0 as u8; 2];
    data_src.copy_from_slice(&buf[..2 as usize]);
    return (&buf[2..], i16::from_le_bytes(data_src));
}
pub fn unpack_u32(buf: &[u8]) -> (&[u8], u32) {
    let mut data_src: [u8; 4] = [0 as u8; 4];
    data_src.copy_from_slice(&buf[..4 as usize]);
    return (&buf[4..], u32::from_le_bytes(data_src));
}
pub fn unpack_i32(buf: &[u8]) -> (&[u8], i32) {
    let mut data_src: [u8; 4] = [0 as u8; 4];
    data_src.copy_from_slice(&buf[..4 as usize]);
    return (&buf[4..], i32::from_le_bytes(data_src));
}
pub fn unpack_u64(buf: &[u8]) -> (&[u8], u64) {
    let mut data_src: [u8; 8] = [0 as u8; 8];
    data_src.copy_from_slice(&buf[..8 as usize]);
    return (&buf[8..], u64::from_le_bytes(data_src));
}
pub fn unpack_i64(buf: &[u8]) -> (&[u8], i64) {
    let mut data_src: [u8; 8] = [0 as u8; 8];
    data_src.copy_from_slice(&buf[..8 as usize]);
    return (&buf[8..], i64::from_le_bytes(data_src));
}

pub fn unpack_u128(buf: &[u8]) -> (&[u8], u128) {
    let mut data_src: [u8; 16] = [0 as u8; 16];
    data_src.copy_from_slice(&buf[..16 as usize]);
    return (&buf[16..], u128::from_le_bytes(data_src));
}

pub fn unpack_i128(buf: &[u8]) -> (&[u8], i128) {
    let mut data_src: [u8; 16] = [0 as u8; 16];
    data_src.copy_from_slice(&buf[..16 as usize]);
    return (&buf[16..], i128::from_le_bytes(data_src));
}

pub fn state_unpack_option<F, T>(buf: &[u8], f: F, internal_size: usize) -> (&[u8], Option<T>)
where
    F: Fn(&[u8]) -> (&[u8], T),
{
    let (dst, non_null) = unpack_bool(buf);
    if non_null {
        let (xdst, x) = f(dst);
        return (xdst, Some(x));
    } else {
        let xdst = &buf[internal_size + 1..];
        return (xdst, None);
    }
}

pub fn instructions_unpack_option<F, T>(buf: &[u8], f: F) -> (&[u8], Option<T>)
where
    F: Fn(&[u8]) -> (&[u8], T),
{
    let (dst, non_null) = unpack_bool(buf);
    if non_null {
        let (xdst, x) = f(dst);
        return (xdst, Some(x));
    }
    return (dst, None);
}

// File specific Unpack
// Strings
pub fn instructions_unpack_string(buf: &[u8]) -> (&[u8], String) {
    // Get the string length
    let mut data_len_src: [u8; 4] = [0 as u8; 4];
    data_len_src.copy_from_slice(&buf[..4]);
    let data_len = u32::from_le_bytes(data_len_src);
    // Get the string bytes
    let data_src = buf.get(4..4 + data_len as usize).unwrap().to_vec();
    return (
        &buf[(4 + data_len) as usize..],
        String::from_utf8(data_src).unwrap(),
    );
}
pub fn state_unpack_string(buf: &[u8], field_size: usize) -> (&[u8], String) {
    // Get the string length
    let mut data_len_src: [u8; 4] = [0 as u8; 4];
    data_len_src.copy_from_slice(&buf[..4]);
    let data_len = u32::from_le_bytes(data_len_src);
    // Get the string bytes
    let data_src = buf.get(4..4 + data_len as usize).unwrap().to_vec();
    return (&buf[field_size..], String::from_utf8(data_src).unwrap());
}

// Vectors
// Vec

pub fn pack_vec<F, T>(
    buf: &mut [u8],
    cap: usize,
    subtype_size: usize,
    f: F,
    data: Vec<T>,
) -> &mut [u8]
where
    F: Fn(&mut [u8], T) -> &mut [u8],
{
    if data.len() > cap {
        panic!("{}", VECTOR_OVERSIZE_PANIC);
    }
    let mut ptr0 = pack_u32(buf, data.len() as u32);
    for t in data {
        ptr0 = f(ptr0, t);
    }
    &mut buf[(4 + cap * subtype_size) as usize..]
}

pub fn instructions_unpack_vec<F, T>(buf: &[u8], subtype_size: usize, f: F) -> (&[u8], Vec<T>)
where
    F: Fn(&[u8]) -> (&[u8], T),
{
    // Get the vec length
    let (mut new_buf, vec_len) = unpack_u32(buf);
    let mut ret = Vec::with_capacity(vec_len as usize);
    let mut item;

    for _idx in 0..vec_len {
        (new_buf, item) = f(new_buf);
        ret.push(item);
    }

    return (&buf[(4 + vec_len as usize * subtype_size) as usize..], ret);
}

pub fn state_unpack_vec<F, T>(buf: &[u8], cap: usize, subtype_size: usize, f: F) -> (&[u8], Vec<T>)
where
    F: Fn(&[u8]) -> (&[u8], T),
{
    // Get the vec length
    let (mut new_buf, vec_len) = unpack_u32(buf);

    if vec_len as usize > cap {
        panic!("{}", VECTOR_OVERSIZE_PANIC);
    }
    let mut ret = Vec::with_capacity(cap);
    let mut item;

    for _idx in 0..vec_len {
        (new_buf, item) = f(new_buf);
        ret.push(item);
    }

    return (&buf[(4 + cap * subtype_size) as usize..], ret);
}

pub fn bool_to_u8(data: bool) -> u8 {
    let result: u8;
    match data {
        true => result = 1,
        false => result = 0,
    }
    return result;
}

pub fn unpack_pubkey(buf: &[u8]) -> (&[u8], Pubkey) {
    let mut data_src: [u8; 32] = [0 as u8; 32];
    data_src.copy_from_slice(&buf[..32 as usize]);
    return (&buf[32..], Pubkey::new_from_array(data_src));
}

pub fn pack_pubkey(buf: &mut [u8], data: Pubkey) -> &mut [u8] {
    buf[0..32].copy_from_slice(&data.to_bytes());
    &mut buf[32..]
}
