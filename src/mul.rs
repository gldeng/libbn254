use bn::AffineG1;
use crate::errors::Errors;
use crate::utilities::{read_point, right_pad};

/// Input length for the multiplication operation.
/// `MUL` takes an uncompressed G1 point (64 bytes) and scalar (32 bytes).
pub const MUL_INPUT_LEN: usize = 64 + 32;

pub fn run_mul(input: &[u8]) -> Result<[u8; 64], Errors> {
    let input = right_pad::<MUL_INPUT_LEN>(input);

    let p = read_point(&input[..64])?;

    // `Fr::from_slice` can only fail when the length is not 32.
    let fr = bn::Fr::from_slice(&input[64..96]).unwrap();

    let mut output = [0u8; 64];
    if let Some(mul) = AffineG1::from_jacobian(p * fr) {
        mul.x().to_big_endian(&mut output[..32]).unwrap();
        mul.y().to_big_endian(&mut output[32..]).unwrap();
    }
    Ok(output)
}


#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::errors::Errors;
    use crate::mul::run_mul;

    #[test]
    fn test_alt_bn128_mul() {
        let input = hex::decode(
            "\
            2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb7\
            21611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb204\
            00000000000000000000000000000000000000000000000011138ce750fa15c2",
        )
            .unwrap();
        let expected = hex::decode(
            "\
            070a8d6a982153cae4be29d434e8faef8a47b274a053f5a4ee2a6c9c13c31e5c\
            031b8ce914eba3a9ffb989f9cdd5b0f01943074bf4f0f315690ec3cec6981afc",
        )
            .unwrap();

        let output = run_mul(&input).unwrap();
        assert_eq!(output.as_slice(), expected);

        // zero multiplication test
        let input = hex::decode(
            "\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000\
            0200000000000000000000000000000000000000000000000000000000000000",
        )
            .unwrap();
        let expected = hex::decode(
            "\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000",
        )
            .unwrap();

        let output = run_mul(&input).unwrap();
        assert_eq!(output.as_slice(), expected);


        // no input test
        let input = [0u8; 0];
        let expected = hex::decode(
            "\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000",
        )
            .unwrap();

        let output = run_mul(&input).unwrap();
        assert_eq!(output.as_slice(), expected);

        // point not on curve fail
        let input = hex::decode(
            "\
            1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111\
            0f00000000000000000000000000000000000000000000000000000000000000",
        )
            .unwrap();

        let res = run_mul(&input);
        assert!(matches!(
            res,
            Err(Errors::Error(Error::Bn128AffineGFailedToCreate))
        ));
    }
}