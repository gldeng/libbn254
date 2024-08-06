use crate::return_codes::ReturnCodes;
use crate::add::run_add;
use crate::mul::run_mul;
use crate::pair::run_pair;
use std::slice;

#[no_mangle]
pub extern "C" fn bn254_add(
    buf: *mut cty::uint8_t,
    max_len: cty::uint32_t,
) -> cty::c_int {
    if max_len < 128 {
        ReturnCodes::InvalidInput as i32
    } else {
        let input = unsafe { slice::from_raw_parts(buf, 128) };
        match run_add(input) {
            Ok(output) => {
                unsafe {
                    slice::from_raw_parts_mut(buf, 64).copy_from_slice(&output);
                }
                ReturnCodes::Success as i32
            }
            Err(_) => ReturnCodes::Failed as i32,
        }
    }
}

#[no_mangle]
pub extern "C" fn bn254_scalar_mul(
    buf: *mut cty::uint8_t,
    max_len: cty::uint32_t,
) -> cty::c_int {
    if max_len < 96 {
        ReturnCodes::InvalidInput as i32
    } else {
        let input = unsafe { slice::from_raw_parts(buf, 96) };
        match run_mul(input) {
            Ok(output) => {
                unsafe {
                    slice::from_raw_parts_mut(buf, 64).copy_from_slice(&output);
                }
                ReturnCodes::Success as i32
            }
            Err(_) => ReturnCodes::Failed as i32,
        }
    }
}

#[no_mangle]
pub extern "C" fn bn254_pairing(
    buf: *mut cty::uint8_t,
    max_len: cty::uint32_t,
) -> cty::c_int {
    if max_len < 192 {
        ReturnCodes::InvalidInput as i32
    } else {
        let input = unsafe { slice::from_raw_parts(buf, max_len as usize) };
        match run_pair(input) {
            Ok(output) => {
                unsafe {
                    slice::from_raw_parts_mut(buf, 32).copy_from_slice(&output);
                }
                ReturnCodes::Success as i32
            }
            Err(_) => ReturnCodes::Failed as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pair::run_pair;
    use super::*;
    use crate::return_codes::ReturnCodes;

    #[test]
    fn test_add(){
        let mut buffer: Vec<u8> = vec![0; 128];
        let input = hex::decode(
            "\
             18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9\
             063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266\
             07c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed\
             06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7",
        )
            .unwrap();
        buffer[..128].copy_from_slice(&input);
        let expected = hex::decode(
            "\
            2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703\
            301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c915",
        )
            .unwrap();

        let res = bn254_add(buffer.as_mut_ptr(), 128);
        assert_eq!(res, ReturnCodes::Success as i32);
        assert_eq!(buffer[..64], expected);
    }

    #[test]
    fn test_scalar_mul() {
        let mut buffer: Vec<u8> = vec![0; 96];
        let input = hex::decode(
            "\
            2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb7\
            21611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb204\
            00000000000000000000000000000000000000000000000011138ce750fa15c2",
        )
            .unwrap();
        buffer[..96].copy_from_slice(&input);
        let expected = hex::decode(
            "\
            070a8d6a982153cae4be29d434e8faef8a47b274a053f5a4ee2a6c9c13c31e5c\
            031b8ce914eba3a9ffb989f9cdd5b0f01943074bf4f0f315690ec3cec6981afc",
        )
            .unwrap();
        let res = bn254_scalar_mul(buffer.as_mut_ptr(), 96);
        assert_eq!(res, ReturnCodes::Success as i32);
        assert_eq!(buffer[..64], expected);
    }

    #[test]
    fn test_pairing() {
        let mut buffer: Vec<u8> = vec![0; 384];
        let input = hex::decode(
            "\
            1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f59\
            3034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41\
            209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf7\
            04bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a41678\
            2bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d\
            120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550\
            111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c\
            2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411\
            198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2\
            1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed\
            090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b\
            12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
        )
            .unwrap();
        buffer[..384].copy_from_slice(&input);
        let expected =
            hex::decode("0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap();

        let output = run_pair(
            &input
        )
            .unwrap();
        assert_eq!(output.as_slice(), expected);
    }

}