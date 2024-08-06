use crate::utilities::new_g1_point;
use bn::{AffineG2, Fq, Fq2, G2, Gt, Group};

use crate::errors::{Errors, Error};

/// Pair element length.
/// `PAIR` elements are composed of an uncompressed G1 point (64 bytes) and an uncompressed G2 point
/// (128 bytes).
pub(crate) const PAIR_ELEMENT_LEN: usize = 64 + 128;

pub(crate) fn run_pair(
    input: &[u8]
) -> Result<[u8; 32], Errors> {
    if input.len() % PAIR_ELEMENT_LEN != 0 {
        return Err(Error::Bn128PairLength.into());
    }

    let success = if input.is_empty() {
        true
    } else {
        let elements = input.len() / PAIR_ELEMENT_LEN;

        let mut points = Vec::with_capacity(elements);

        // read points
        for idx in 0..elements {
            let read_fq_at = |n: usize| {
                debug_assert!(n < PAIR_ELEMENT_LEN / 32);
                let start = idx * PAIR_ELEMENT_LEN + n * 32;
                // SAFETY: We're reading `6 * 32 == PAIR_ELEMENT_LEN` bytes from `input[idx..]`
                // per iteration. This is guaranteed to be in-bounds.
                let slice = unsafe { input.get_unchecked(start..start + 32) };
                Fq::from_slice(slice).map_err(|_| Error::Bn128FieldPointNotAMember)
            };
            let ax = read_fq_at(0)?;
            let ay = read_fq_at(1)?;
            let bay = read_fq_at(2)?;
            let bax = read_fq_at(3)?;
            let bby = read_fq_at(4)?;
            let bbx = read_fq_at(5)?;

            let a = new_g1_point(ax, ay)?;
            let b = {
                let ba = Fq2::new(bax, bay);
                let bb = Fq2::new(bbx, bby);
                // TODO: check whether or not we need these zero checks
                if ba.is_zero() && bb.is_zero() {
                    G2::zero()
                } else {
                    G2::from(AffineG2::new(ba, bb).map_err(|_| Error::Bn128AffineGFailedToCreate)?)
                }
            };

            points.push((a, b));
        }

        let mul = bn::pairing_batch(&points);

        mul == Gt::one()
    };
    let mut output = [0u8; 32];
    if success {
        output[31] = 1;
    }
    Ok(output)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alt_bn128_pair() {
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
        let expected =
            hex::decode("0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap();

        let output = run_pair(
            &input
        )
            .unwrap();
        assert_eq!(output.as_slice(), expected);

        // no input test
        let input = [0u8; 0];
        let expected =
            hex::decode("0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap();

        let output = run_pair(
            &input
        )
            .unwrap();
        assert_eq!(output.as_slice(), expected);

        // point not on curve fail
        let input = hex::decode(
            "\
            1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111",
        )
            .unwrap();

        let res = run_pair(
            &input
        );
        assert!(matches!(
            res,
            Err(Errors::Error(Error::Bn128AffineGFailedToCreate))
        ));

        // invalid input length
        let input = hex::decode(
            "\
            1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111\
            111111111111111111111111111111\
        ",
        )
            .unwrap();

        let res = run_pair(
            &input
        );
        assert!(matches!(
            res,
            Err(Errors::Error(Error::Bn128PairLength))
        ));
    }
}

