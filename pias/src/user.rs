use crate::utils::{get_random_scalar, hash_to_g1_point, hash_to_scalar, G2_BASEPOINT};
use bls12_381::{pairing, G1Affine, G1Projective, G2Affine, G2Projective};

pub fn user_verify_signature(
    pk_x: &G2Projective,
    pk_y: &G2Projective,
    message: &[u8],
    sig1: &G1Projective,
    sig2: &G1Projective,
) -> bool {
    let gt_1 = pairing(&G1Affine::from(sig2), &G2Affine::from(*G2_BASEPOINT));
    let message_hash = hash_to_scalar(message);
    let part2 = pk_x + pk_y * message_hash;
    let gt_2 = pairing(&G1Affine::from(sig1), &G2Affine::from(part2));
    if gt_1.eq(&gt_2) {
        return true;
    }
    false
}
//
// pub fn user_aggregate_public_key() {
//
// }

pub fn user_aggregate_signature(sig2_list: &[G1Projective]) -> G1Projective {
    // TODO: check all length
    // let h_point = hash_to_g1_point(aux_id);
    let mut sig2_result = G1Projective::default();
    for sig2 in sig2_list {
        sig2_result += sig2;
    }
    sig2_result
}

pub fn user_random_signature(aux_id: &[u8], sig2: &G1Projective) -> (G1Projective, G1Projective) {
    let blinding = get_random_scalar();
    let h_point = hash_to_g1_point(aux_id);
    let cert1 = h_point * blinding;
    let cert2 = sig2 * blinding;
    (cert1, cert2)
}

// pub fn user_aggregate_signature(aux_id: &[u8],  pk_x_list: &[G2Projective], pk_y_list: &[G2Projective], message: &Vec<Vec<u8>>, sig2_list: &[G1Projective]) -> G1Projective {
//     // TODO: check all length
//     let h_point = hash_to_g1_point(aux_id);
//     let mut sig2_result = G1Projective::default();
//     for i in 0..pk_x_list.len() {
//         sig2_result += sig2_list[i];
//     }
//     sig2_result
// }
