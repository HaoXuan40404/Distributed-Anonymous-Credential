use bls12_381::{G1Projective, G2Projective, Scalar};
use ff::Field;
use group::Group;
use rand::thread_rng;
use std::convert::TryInto;
use wedpr_l_crypto_hash_sha3::WedprSha3_256;
use wedpr_l_utils::traits::Hash;

lazy_static! {
    pub static ref G1_BASEPOINT: G1Projective = G1Projective::generator();
    // static ref G1_BASEPOINT: G1Affine = G1Affine::generator();
    pub static ref G2_BASEPOINT: G2Projective = G2Projective::generator();
    // static ref G2_BASEPOINT: G2Affine = G2Affine::generator();
    pub static ref HASH_SHA3_256: WedprSha3_256 = WedprSha3_256::default();
}

fn convert_array(vector: &[u8]) -> [u8; 64] {
    let vector_double: Vec<u8> = [vector.clone(), vector].concat();
    vector_double
        .try_into()
        .unwrap_or_else(|vector_double: Vec<u8>| {
            panic!(
                "Expected a Vec of length {} but it was {}",
                64,
                vector_double.len()
            )
        })
}

pub fn hash_to_scalar(message: &[u8]) -> Scalar {
    let hash_bytes = HASH_SHA3_256.hash(message);
    let hash_array = convert_array(&hash_bytes);
    Scalar::from_bytes_wide(&hash_array)
}

pub fn hash_to_g1_point(message: &[u8]) -> G1Projective {
    let num = hash_to_scalar(message);
    *G1_BASEPOINT * num
}

pub fn get_random_g1_point() -> G1Projective {
    let rng = thread_rng();
    G1Projective::random(rng)
}

pub fn get_random_scalar() -> Scalar {
    let rng = thread_rng();
    Scalar::random(rng)
}

// pub fn hash_to_g1_point(message: &[u8]) -> G1Affine {
//     let num = hash_to_scalar(message);
//     let point = *G1_BASEPOINT * num;
// }
