use crate::utils::{hash_to_g1_point, hash_to_scalar, G2_BASEPOINT};
use bls12_381::{G1Projective, G2Projective, Scalar};
use ff::Field;
use rand::thread_rng;

pub fn issuer_key_generate() -> (Scalar, Scalar, G2Projective, G2Projective) {
    let rng = thread_rng();
    let sk1 = Scalar::random(rng.clone());
    let sk2 = Scalar::random(rng);
    let pk1 = *G2_BASEPOINT * sk1;
    let pk2 = *G2_BASEPOINT * sk2;
    (sk1, sk2, pk1, pk2)
}

pub fn issuer_sign_credential(
    aux_id: &[u8],
    message: &[u8],
    sk1: &Scalar,
    sk2: &Scalar,
) -> (G1Projective, G1Projective) {
    // let mut rng = thread_rng();
    // let h_point = G1Projective::random(rng);
    let h_point = hash_to_g1_point(aux_id);
    let message_hash = hash_to_scalar(&message);
    let h_aux_point = h_point * (sk1 + sk2 * message_hash);
    (h_point, h_aux_point)
}
