use std::borrow::Cow;
use bn::{AffineG1, Fq, G1, Group};
use crate::Error;

/// Right-pads the given slice with zeroes until `LEN`.
///
/// Returns the first `LEN` bytes if it does not need padding.
#[inline]
pub fn right_pad<const LEN: usize>(data: &[u8]) -> Cow<'_, [u8; LEN]> {
    if let Some(data) = data.get(..LEN) {
        Cow::Borrowed(data.try_into().unwrap())
    } else {
        let mut padded = [0; LEN];
        padded[..data.len()].copy_from_slice(data);
        Cow::Owned(padded)
    }
}

/// Reads a single `Fq` from the input slice.
///
/// # Panics
///
/// Panics if the input is not at least 32 bytes long.
#[inline]
pub fn read_fq(input: &[u8]) -> Result<Fq, Error> {
    Fq::from_slice(&input[..32]).map_err(|_| Error::Bn128FieldPointNotAMember)
}

/// Reads the `x` and `y` points from the input slice.
///
/// # Panics
///
/// Panics if the input is not at least 64 bytes long.
#[inline]
pub fn read_point(input: &[u8]) -> Result<G1, Error> {
    let px = read_fq(&input[0..32])?;
    let py = read_fq(&input[32..64])?;
    new_g1_point(px, py)
}

/// Creates a new `G1` point from the given `x` and `y` coordinates.
pub fn new_g1_point(px: Fq, py: Fq) -> Result<G1, Error> {
    if px == Fq::zero() && py == Fq::zero() {
        Ok(G1::zero())
    } else {
        AffineG1::new(px, py)
            .map(Into::into)
            .map_err(|_| Error::Bn128AffineGFailedToCreate)
    }
}
