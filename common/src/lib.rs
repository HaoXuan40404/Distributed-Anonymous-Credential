use bls12_381::{pairing, G1Affine, G1Projective, G2Affine, G2Projective, Gt, Scalar};
use ff::Field;
use group::Group;
use rand::thread_rng;
use std::convert::TryInto;
use wedpr_l_crypto_hash_sha3::WedprSha3_256;
use wedpr_l_utils::traits::Hash;

pub mod d_cred;
pub mod l_cred;
pub mod m_cred;
pub mod part_as;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref G1_BASEPOINT: G1Projective = G1Projective::generator();
    // static ref G1_BASEPOINT: G1Affine = G1Affine::generator();
    pub static ref G2_BASEPOINT: G2Projective = G2Projective::generator();
    // static ref G2_BASEPOINT: G2Affine = G2Affine::generator();
    pub static ref HASH_SHA3_256: WedprSha3_256 = WedprSha3_256::default();
}

pub fn Y_scalar_mul(num1: &Scalar, num2: &Scalar) -> Scalar {
    return num1 * num2;
}

pub fn Z_scalar_add(num1: &Scalar, num2: &Scalar) -> Scalar {
    return num1 + num2;
}

pub fn E_G1_mul(num: &Scalar, point: &G1Projective) -> G1Projective {
    return *point * num;
}

pub fn F_G2_mul(num: &Scalar, point: &G2Projective) -> G2Projective {
    return *point * num;
}

pub fn A_G1_add(point1: &G1Projective, point2: &G1Projective) -> G1Projective {
    return point1 + point2;
}

pub fn B_G2_add(point1: &G2Projective, point2: &G2Projective) -> G2Projective {
    return point1 + point2;
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

pub fn P_paring(point1: &G1Projective, point2: &G2Projective) -> Gt {
    let paring_point = pairing(&G1Affine::from(point1), &G2Affine::from(point2));
    paring_point
}

pub fn L_aggregate(messages: &Vec<String>) -> String {
    let mut result = "".to_string();
    for message in messages {
        result = format!("{}{}", message, result);
    }
    return result;
}

pub fn get_random_scalar() -> Scalar {
    let rng = thread_rng();
    Scalar::random(rng)
}

pub fn get_random_g1_point() -> G1Projective {
    let rng = thread_rng();
    G1Projective::random(rng)
}

pub fn get_random_g2_point() -> G2Projective {
    let rng = thread_rng();
    G2Projective::random(rng)
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

pub fn test_Y_scalar_mul() {
    let num = get_random_scalar();
    let _= Y_scalar_mul(&num, &num);
}

pub fn test_Z_scalar_add() {
    let num = get_random_scalar();
    let _ = Z_scalar_add(&num, &num);
}

pub fn test_E_G1_mul() {
    let num = get_random_scalar();
    let point = get_random_g1_point();
    let _ = E_G1_mul(&num, &point);
}

pub fn test_F_G2_mul() {
    let num = get_random_scalar();
    let point = get_random_g2_point();
    let _ = F_G2_mul(&num, &point);
}

pub fn test_A_G1_add() {
    let point = get_random_g1_point();
    let _ = A_G1_add(&point, &point);
}

pub fn test_B_G2_add() {
    let point = get_random_g2_point();
    let _ = B_G2_add(&point, &point);
}

pub fn test_hash_to_scalar() {
    let message = "test".as_bytes();
    let _ = hash_to_scalar(message);
}

pub fn test_hash_to_g1_point() {
    let message = "test".as_bytes();
    let _ = hash_to_g1_point(message);
}

pub fn test_P_paring() {
    let point1 = get_random_g1_point();
    let point2 = get_random_g2_point();
    let _ = P_paring(&point1, &point2);
}

pub fn test_L_aggregate() {
    let message_vec = vec![format!("123"), format!("223")];
    let _ = L_aggregate(&message_vec);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let g1_point1 = G1Projective::generator();
        let g1_point2 = G1Projective::generator();
        let g2_point1 = G2Projective::generator();
        let g2_point2 = G2Projective::generator();
        let num1 = get_random_scalar();
        let num2 = get_random_scalar();
        let point1 = get_random_g1_point();
        let point2 = get_random_g2_point();
        let _ = Y_scalar_mul(&num1, &num2);
        let _ = Z_scalar_add(&num1, &num2);
        let _ = E_G1_mul(&num1, &g1_point1);
        let _ = F_G2_mul(&num1, &g2_point1);
        let _ = A_G1_add(&g1_point1, &g1_point1);
        let _ = B_G2_add(&g2_point1, &g2_point2);
        let message = [1u8, 2u8, 3u8];
        let message_vec = vec![format!("123"), format!("223")];
        let _ = L_aggregate(&message_vec);
        let _ = hash_to_scalar(&message);
        let _ = hash_to_g1_point(&message);
        let _ = P_paring(&g1_point1, &g2_point2);
    }
}
