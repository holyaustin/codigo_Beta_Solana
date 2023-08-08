use crate::rust_utils::*;
use solana_program::pubkey::Pubkey;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_pack_unpack_vector_pubkey() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let pubkey1: Pubkey = Pubkey::new_unique();
        let pubkey2: Pubkey = Pubkey::new_unique();
        let pubkey3: Pubkey = Pubkey::new_unique();
        let vec: Vec<Pubkey> = [pubkey1, pubkey2, pubkey3].to_vec();

        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            32,
            |dst: &mut [u8], data: Pubkey| -> &mut [u8] {
                return pack_pubkey(dst, *(&data));
            },
            vec,
        );

        assert_eq!(r.len(), big.len() - (vec_max_cap * 32 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 32, |dst: &[u8]| -> (&[u8], Pubkey) {
                return unpack_pubkey(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [pubkey1, pubkey2, pubkey3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 32));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_bool() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<bool> = [true, true, true, false, true].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], bool) {
                return unpack_bool(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_bool() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<bool> = [true, false, true].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            1,
            |dst: &mut [u8], data: bool| -> &mut [u8] {
                return pack_bool(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 1 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], bool) {
                return unpack_bool(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [true, false, true].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 1));
    }

    #[test]
    fn it_instructions_unpack_vector_bool() {
        let mut vec: Vec<bool> = [true, false, true].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 1];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_bool(ptr, vec[0]);
        ptr = pack_bool(ptr, vec[1]);
        pack_bool(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) =
            instructions_unpack_vec(instructions_buffer, 1, |dst: &[u8]| -> (&[u8], bool) {
                return unpack_bool(dst);
            });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 1));
        assert_eq!(vec, [true, false, true].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_bool_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<bool> = [true, false, true, false, true].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            1,
            |dst: &mut [u8], data: bool| -> &mut [u8] {
                return pack_bool(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 1 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], bool) {
                return unpack_bool(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [true, false, true, false, true].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 1));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_bool() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<bool> =[true, true, true, false, true].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], bool) {
                return unpack_bool(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_bool() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<bool> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            1,
            |dst: &mut [u8], data: bool| -> &mut [u8] {
                return pack_bool(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 1 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], bool) {
                return unpack_bool(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 1));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_u128() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<u128> = [1, 2, 3, 4, 5, 6].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 16, |dst: &[u8]| -> (&[u8], u128) {
                return unpack_u128(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_u128() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u128> = [1, 2, 3].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            16,
            |dst: &mut [u8], data: u128| -> &mut [u8] {
                return pack_u128(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 16 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 16, |dst: &[u8]| -> (&[u8], u128) {
                return unpack_u128(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 16));
    }

    #[test]
    fn it_instructions_unpack_vector_u128() {
        let mut vec: Vec<u128> = [1, 2, 3].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 16];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_u128(ptr, vec[0]);
        ptr = pack_u128(ptr, vec[1]);
        pack_u128(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) =
            instructions_unpack_vec(instructions_buffer, 16, |dst: &[u8]| -> (&[u8], u128) {
                return unpack_u128(dst);
            });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 16));
        assert_eq!(vec, [1, 2, 3].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_u128_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u128> = [1, 2, 3, 4, 5].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            16,
            |dst: &mut [u8], data: u128| -> &mut [u8] {
                return pack_u128(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 16 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 16, |dst: &[u8]| -> (&[u8], u128) {
                return unpack_u128(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3, 4, 5].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 16));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_u128() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<u128> =[1,2,3,4,5,6].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 16, |dst: &[u8]| -> (&[u8], u128) {
                return unpack_u128(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_u128() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u128> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            16,
            |dst: &mut [u8], data: u128| -> &mut [u8] {
                return pack_u128(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 16 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 16, |dst: &[u8]| -> (&[u8], u128) {
                return unpack_u128(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 16));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_i128() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<i128> = [1, 2, 3, 4, 5, 6].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 16, |dst: &[u8]| -> (&[u8], i128) {
                return unpack_i128(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_i128() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i128> = [1, 2, 3].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            16,
            |dst: &mut [u8], data: i128| -> &mut [u8] {
                return pack_i128(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 16 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 16, |dst: &[u8]| -> (&[u8], i128) {
                return unpack_i128(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 16));
    }

    #[test]
    fn it_instructions_unpack_vector_i128() {
        let mut vec: Vec<i128> = [1, 2, 3].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 16];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_i128(ptr, vec[0]);
        ptr = pack_i128(ptr, vec[1]);
        pack_i128(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) =
            instructions_unpack_vec(instructions_buffer, 16, |dst: &[u8]| -> (&[u8], i128) {
                return unpack_i128(dst);
            });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 16));
        assert_eq!(vec, [1, 2, 3].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_i128_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i128> = [1, 2, 3, 4, 5].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            16,
            |dst: &mut [u8], data: i128| -> &mut [u8] {
                return pack_i128(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 16 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 16, |dst: &[u8]| -> (&[u8], i128) {
                return unpack_i128(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3, 4, 5].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 16));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_i128() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<i128> =[1,2,3,4,5,6].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 16, |dst: &[u8]| -> (&[u8], i128) {
                return unpack_i128(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_i128() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i128> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            16,
            |dst: &mut [u8], data: i128| -> &mut [u8] {
                return pack_i128(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 16 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 16, |dst: &[u8]| -> (&[u8], i128) {
                return unpack_i128(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 16));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_u64() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<u64> = [1, 2, 3, 4, 5, 6].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 8, |dst: &[u8]| -> (&[u8], u64) {
                return unpack_u64(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_u64() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u64> = [1, 2, 3].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            8,
            |dst: &mut [u8], data: u64| -> &mut [u8] {
                return pack_u64(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 8 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 8, |dst: &[u8]| -> (&[u8], u64) {
                return unpack_u64(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 8));
    }

    #[test]
    fn it_instructions_unpack_vector_u64() {
        let mut vec: Vec<u64> = [1, 2, 3].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 8];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_u64(ptr, vec[0]);
        ptr = pack_u64(ptr, vec[1]);
        pack_u64(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) =
            instructions_unpack_vec(instructions_buffer, 8, |dst: &[u8]| -> (&[u8], u64) {
                return unpack_u64(dst);
            });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 8));
        assert_eq!(vec, [1, 2, 3].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_u64_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u64> = [1, 2, 3, 4, 5].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            8,
            |dst: &mut [u8], data: u64| -> &mut [u8] {
                return pack_u64(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 8 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 8, |dst: &[u8]| -> (&[u8], u64) {
                return unpack_u64(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3, 4, 5].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 8));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_u64() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<u64> =[1,2,3,4,5,6].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 8, |dst: &[u8]| -> (&[u8], u64) {
                return unpack_u64(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_u64() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u64> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            8,
            |dst: &mut [u8], data: u64| -> &mut [u8] {
                return pack_u64(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 8 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 8, |dst: &[u8]| -> (&[u8], u64) {
                return unpack_u64(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 8));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_i64() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<i64> = [1, 2, 3, 4, 5, 6].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 8, |dst: &[u8]| -> (&[u8], i64) {
                return unpack_i64(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_i64() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i64> = [1, 2, 3].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            8,
            |dst: &mut [u8], data: i64| -> &mut [u8] {
                return pack_i64(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 8 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 8, |dst: &[u8]| -> (&[u8], i64) {
                return unpack_i64(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 8));
    }

    #[test]
    fn it_instructions_unpack_vector_i64() {
        let mut vec: Vec<i64> = [1, 2, 3].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 8];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_i64(ptr, vec[0]);
        ptr = pack_i64(ptr, vec[1]);
        pack_i64(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) =
            instructions_unpack_vec(instructions_buffer, 8, |dst: &[u8]| -> (&[u8], i64) {
                return unpack_i64(dst);
            });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 8));
        assert_eq!(vec, [1, 2, 3].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_i64_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i64> = [1, 2, 3, 4, 5].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            8,
            |dst: &mut [u8], data: i64| -> &mut [u8] {
                return pack_i64(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 8 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 8, |dst: &[u8]| -> (&[u8], i64) {
                return unpack_i64(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3, 4, 5].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 8));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_i64() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<i64> =[1,2,3,4,5,6].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 8, |dst: &[u8]| -> (&[u8], i64) {
                return unpack_i64(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_i64() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i64> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            8,
            |dst: &mut [u8], data: i64| -> &mut [u8] {
                return pack_i64(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 8 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 8, |dst: &[u8]| -> (&[u8], i64) {
                return unpack_i64(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 8));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_u32() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<u32> = [1, 2, 3, 4, 5, 6].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 4, |dst: &[u8]| -> (&[u8], u32) {
                return unpack_u32(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_u32() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u32> = [1, 2, 3].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            4,
            |dst: &mut [u8], data: u32| -> &mut [u8] {
                return pack_u32(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 4 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 4, |dst: &[u8]| -> (&[u8], u32) {
                return unpack_u32(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 4));
    }

    #[test]
    fn it_instructions_unpack_vector_u32() {
        let mut vec: Vec<u32> = [1, 2, 3].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 4];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_u32(ptr, vec[0]);
        ptr = pack_u32(ptr, vec[1]);
        pack_u32(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) =
            instructions_unpack_vec(instructions_buffer, 4, |dst: &[u8]| -> (&[u8], u32) {
                return unpack_u32(dst);
            });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 4));
        assert_eq!(vec, [1, 2, 3].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_u32_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u32> = [1, 2, 3, 4, 5].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            4,
            |dst: &mut [u8], data: u32| -> &mut [u8] {
                return pack_u32(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 4 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 4, |dst: &[u8]| -> (&[u8], u32) {
                return unpack_u32(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3, 4, 5].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 4));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_u32() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<u32> =[1,2,3,4,5,6].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 4, |dst: &[u8]| -> (&[u8], u32) {
                return unpack_u32(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_u32() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u32> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            4,
            |dst: &mut [u8], data: u32| -> &mut [u8] {
                return pack_u32(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 4 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 4, |dst: &[u8]| -> (&[u8], u32) {
                return unpack_u32(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 4));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_i32() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<i32> = [1, 2, 3, 4, 5, 6].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 4, |dst: &[u8]| -> (&[u8], i32) {
                return unpack_i32(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_i32() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i32> = [1, 2, 3].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            4,
            |dst: &mut [u8], data: i32| -> &mut [u8] {
                return pack_i32(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 4 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 4, |dst: &[u8]| -> (&[u8], i32) {
                return unpack_i32(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 4));
    }

    #[test]
    fn it_instructions_unpack_vector_i32() {
        let mut vec: Vec<i32> = [1, 2, 3].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 4];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_i32(ptr, vec[0]);
        ptr = pack_i32(ptr, vec[1]);
        pack_i32(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) =
            instructions_unpack_vec(instructions_buffer, 4, |dst: &[u8]| -> (&[u8], i32) {
                return unpack_i32(dst);
            });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 4));
        assert_eq!(vec, [1, 2, 3].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_i32_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i32> = [1, 2, 3, 4, 5].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            4,
            |dst: &mut [u8], data: i32| -> &mut [u8] {
                return pack_i32(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 4 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 4, |dst: &[u8]| -> (&[u8], i32) {
                return unpack_i32(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3, 4, 5].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 4));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_i32() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<i32> =[1,2,3,4,5,6].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 4, |dst: &[u8]| -> (&[u8], i32) {
                return unpack_i32(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_i32() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i32> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            4,
            |dst: &mut [u8], data: i32| -> &mut [u8] {
                return pack_i32(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 4 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 4, |dst: &[u8]| -> (&[u8], i32) {
                return unpack_i32(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 4));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_u16() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<u16> = [1, 2, 3, 4, 5, 6].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 2, |dst: &[u8]| -> (&[u8], u16) {
                return unpack_u16(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_u16() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u16> = [1, 2, 3].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            2,
            |dst: &mut [u8], data: u16| -> &mut [u8] {
                return pack_u16(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 2 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 2, |dst: &[u8]| -> (&[u8], u16) {
                return unpack_u16(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 2));
    }

    #[test]
    fn it_instructions_unpack_vector_u16() {
        let mut vec: Vec<u16> = [1, 2, 3].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 2];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_u16(ptr, vec[0]);
        ptr = pack_u16(ptr, vec[1]);
        pack_u16(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) =
            instructions_unpack_vec(instructions_buffer, 2, |dst: &[u8]| -> (&[u8], u16) {
                return unpack_u16(dst);
            });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 2));
        assert_eq!(vec, [1, 2, 3].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_u16_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u16> = [1, 2, 3, 4, 5].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            2,
            |dst: &mut [u8], data: u16| -> &mut [u8] {
                return pack_u16(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 2 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 2, |dst: &[u8]| -> (&[u8], u16) {
                return unpack_u16(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3, 4, 5].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 2));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_u16() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<u16> =[1,2,3,4,5,6].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 2, |dst: &[u8]| -> (&[u8], u16) {
                return unpack_u16(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_u16() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u16> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            2,
            |dst: &mut [u8], data: u16| -> &mut [u8] {
                return pack_u16(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 2 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 2, |dst: &[u8]| -> (&[u8], u16) {
                return unpack_u16(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 2));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_i16() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<i16> = [1, 2, 3, 4, 5, 6].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 2, |dst: &[u8]| -> (&[u8], i16) {
                return unpack_i16(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_i16() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i16> = [1, 2, 3].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            2,
            |dst: &mut [u8], data: i16| -> &mut [u8] {
                return pack_i16(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 2 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 2, |dst: &[u8]| -> (&[u8], i16) {
                return unpack_i16(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 2));
    }

    #[test]
    fn it_instructions_unpack_vector_i16() {
        let mut vec: Vec<i16> = [1, 2, 3].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 2];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_i16(ptr, vec[0]);
        ptr = pack_i16(ptr, vec[1]);
        pack_i16(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) =
            instructions_unpack_vec(instructions_buffer, 2, |dst: &[u8]| -> (&[u8], i16) {
                return unpack_i16(dst);
            });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 2));
        assert_eq!(vec, [1, 2, 3].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_i16_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i16> = [1, 2, 3, 4, 5].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            2,
            |dst: &mut [u8], data: i16| -> &mut [u8] {
                return pack_i16(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 2 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 2, |dst: &[u8]| -> (&[u8], i16) {
                return unpack_i16(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3, 4, 5].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 2));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_i16() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<i16> =[1,2,3,4,5,6].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 2, |dst: &[u8]| -> (&[u8], i16) {
                return unpack_i16(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_i16() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i16> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            2,
            |dst: &mut [u8], data: i16| -> &mut [u8] {
                return pack_i16(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 2 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) =
            state_unpack_vec(big, vec_max_cap, 2, |dst: &[u8]| -> (&[u8], i16) {
                return unpack_i16(dst);
            });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 2));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_u8() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<u8> = [1, 2, 3, 4, 5, 6].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], u8) {
                return unpack_u8(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_u8() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u8> = [1, 2, 3].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            1,
            |dst: &mut [u8], data: u8| -> &mut [u8] {
                return pack_u8(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 1 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) = state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], u8) {
            return unpack_u8(dst);
        });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 1));
    }

    #[test]
    fn it_instructions_unpack_vector_u8() {
        let mut vec: Vec<u8> = [1, 2, 3].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 1];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_u8(ptr, vec[0]);
        ptr = pack_u8(ptr, vec[1]);
        pack_u8(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) = instructions_unpack_vec(instructions_buffer, 1, |dst: &[u8]| -> (&[u8], u8) {
            return unpack_u8(dst);
        });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 1));
        assert_eq!(vec, [1, 2, 3].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_u8_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u8> = [1, 2, 3, 4, 5].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            1,
            |dst: &mut [u8], data: u8| -> &mut [u8] {
                return pack_u8(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 1 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) = state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], u8) {
            return unpack_u8(dst);
        });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3, 4, 5].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 1));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_u8() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<u8> =[1,2,3,4,5,6].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], u8) {
                return unpack_u8(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_u8() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<u8> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            1,
            |dst: &mut [u8], data: u8| -> &mut [u8] {
                return pack_u8(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 1 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) = state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], u8) {
            return unpack_u8(dst);
        });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 1));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_unpacking_over_capacity_vector_i8() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        let _vec: Vec<i8> = [1, 2, 3, 4, 5, 6].to_vec();
        // test packing a vector works properly (used in state)
        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], i8) {
                return unpack_i8(dst);
            });
    }

    #[test]
    fn it_pack_unpack_vector_i8() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i8> = [1, 2, 3].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            1,
            |dst: &mut [u8], data: i8| -> &mut [u8] {
                return pack_i8(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 1 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) = state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], i8) {
            return unpack_i8(dst);
        });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 1));
    }

    #[test]
    fn it_instructions_unpack_vector_i8() {
        let mut vec: Vec<i8> = [1, 2, 3].to_vec();
        let instructions_buffer: &mut [u8] = &mut [0; 4 + 3 * 1];
        let mut ptr = pack_u32(instructions_buffer, 3);
        ptr = pack_i8(ptr, vec[0]);
        ptr = pack_i8(ptr, vec[1]);
        pack_i8(ptr, vec[2]);

        let ptr: &[u8];

        (ptr, vec) = instructions_unpack_vec(instructions_buffer, 1, |dst: &[u8]| -> (&[u8], i8) {
            return unpack_i8(dst);
        });

        assert_eq!(ptr.len(), instructions_buffer.len() - (4 + 3 * 1));
        assert_eq!(vec, [1, 2, 3].to_vec());
    }

    #[test]
    fn it_pack_unpack_vector_i8_full_capacity() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i8> = [1, 2, 3, 4, 5].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            1,
            |dst: &mut [u8], data: i8| -> &mut [u8] {
                return pack_i8(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 1 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) = state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], i8) {
            return unpack_i8(dst);
        });

        // verify it unpacked every element
        assert_eq!(my_vector, [1, 2, 3, 4, 5].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 1));
    }

    #[test]
    #[should_panic(expected = "Vector size is bigger than defined maximum capacity")]
    fn it_panics_packing_over_capacity_vector_i8() {
        let big: &mut [u8] = &mut [0; 180];
        big[0] = 6;
        let vec_max_cap = 5;
        // let vec: Vec<i8> =[1,2,3,4,5,6].to_vec();

        let (_ptr, _my_vector) =
            state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], i8) {
                return unpack_i8(dst);
            });
    }

    #[test]
    fn it_pack_unpack_empty_vector_i8() {
        let big: &mut [u8] = &mut [0; 180];
        let vec_max_cap = 5;
        let vec: Vec<i8> = [].to_vec();

        // test packing a vector works properly (used in state)
        let r: &mut [u8] = pack_vec(
            big,
            vec_max_cap,
            1,
            |dst: &mut [u8], data: i8| -> &mut [u8] {
                return pack_i8(dst, *(&data));
            },
            vec,
        );

        // verify it is packing the maximum vector capacity and not only the occupied positions
        assert_eq!(r.len(), big.len() - (vec_max_cap * 1 + 4));

        // test unpacking the vector previously packed works properly
        let (ptr, my_vector) = state_unpack_vec(big, vec_max_cap, 1, |dst: &[u8]| -> (&[u8], i8) {
            return unpack_i8(dst);
        });

        // verify it unpacked every element
        assert_eq!(my_vector, [].to_vec());
        // verify the pointer moves the vector's maximum capacity
        assert_eq!(ptr.len(), big.len() - (4 + vec_max_cap * 1));
    }
}
