use bn::AffineG1;
use crate::errors::Errors;
use crate::utilities::{read_point, right_pad};

/// Input length for the add operation.
/// `ADD` takes two uncompressed G1 points (64 bytes each).
pub const ADD_INPUT_LEN: usize = 64 + 64;

pub fn run_add(input: &[u8]) -> Result<[u8; 64], Errors> {
    let input = right_pad::<ADD_INPUT_LEN>(input);

    let p1 = read_point(&input[..64])?;
    let p2 = read_point(&input[64..])?;

    let mut output = [0u8; 64];
    if let Some(sum) = AffineG1::from_jacobian(p1 + p2) {
        sum.x().to_big_endian(&mut output[..32]).unwrap();
        sum.y().to_big_endian(&mut output[32..]).unwrap();
    }
    Ok(output)
}


#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::errors::Errors;
    use super::*;

    #[test]
    fn test_alt_bn128_add() {
        let input = hex::decode(
            "\
             18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9\
             063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266\
             07c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed\
             06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7",
        )
            .unwrap();
        let expected = hex::decode(
            "\
            2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703\
            301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c915",
        )
            .unwrap();

        let output = run_add(&input).unwrap();
        assert_eq!(output.as_slice(), expected);

        // zero sum test
        let input = hex::decode(
            "\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000",
        )
            .unwrap();
        let expected = hex::decode(
            "\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000",
        )
            .unwrap();

        let output = run_add(&input).unwrap();
        assert_eq!(output.as_slice(), expected);

        // no input test
        let input = [0u8; 0];
        let expected = hex::decode(
            "\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000",
        )
            .unwrap();

        let output = run_add(&input).unwrap();
        assert_eq!(output.as_slice(), expected);

        // point not on curve fail
        let input = hex::decode(
            "\
            1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111",
        )
            .unwrap();

        let res = run_add(&input);
        assert!(matches!(
            res,
            Err(Errors::Error(Error::Bn128AffineGFailedToCreate))
        ));
    }
}
