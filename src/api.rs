use crate::return_codes::ReturnCodes;
use crate::mul::run_mul;
use std::slice;

#[no_mangle]
pub extern "C" fn scalar_mul(
    buf: *mut cty::uint8_t,
    max_len: cty::c_int,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::return_codes::ReturnCodes;

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
        let res = scalar_mul(buffer.as_mut_ptr(), 96);
        assert_eq!(res, ReturnCodes::Success as i32);
        assert_eq!(buffer[..64], expected);
    }
}