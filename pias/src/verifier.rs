use crate::utils::{hash_to_scalar, G2_BASEPOINT};
use bls12_381::{pairing, G1Affine, G1Projective, G2Affine, G2Projective};

pub fn verifier_verify(
    pk_x_list: &[G2Projective],
    pk_y_list: &[G2Projective],
    sig1: &G1Projective,
    sig2: &G1Projective,
    message: &Vec<Vec<u8>>,
) -> bool {
    let mut part2 = G2Projective::default();
    // check length
    if pk_x_list.len() != pk_y_list.len() || pk_x_list.len() != message.len() {
        return false;
    }
    for i in 0..pk_x_list.len() {
        let message_scalar = hash_to_scalar(&message[i]);
        let part2_prepare = pk_x_list[i] + pk_y_list[i] * message_scalar;
        part2 += part2_prepare;
    }
    let left_pairing = pairing(&G1Affine::from(sig2), &G2Affine::from(*G2_BASEPOINT));
    let right_pairing = pairing(&G1Affine::from(sig1), &G2Affine::from(part2));
    if left_pairing.eq(&right_pairing) {
        return true;
    }
    false
}
