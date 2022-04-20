use crate::{
    get_random_g1_point, get_random_g2_point, get_random_scalar, A_G1_add,
    B_G2_add, E_G1_mul, F_G2_mul, L_aggregate, P_paring, Y_scalar_mul, Z_scalar_add, hash_to_g1_point
};

pub fn test_key_gen(user_name: usize) {
    let num = get_random_scalar();
    let point = get_random_g2_point();
    for _ in 0..user_name {
        let _ = F_G2_mul(&num, &point);
    }
}

pub fn test_sign(user_number: usize) {
    let num = get_random_scalar();
    let num2 = get_random_scalar();
    let point1 = get_random_g1_point();
    let message = "test".as_bytes();
    for _ in 0..user_number {
        let _ = hash_to_g1_point(message);
        let _ = E_G1_mul(&num, &point1);
        let _ = Y_scalar_mul(&num, &num2);
        let _ = Z_scalar_add(&num, &num2);
    }
}

pub fn test_verify(user_number: usize) {
    let num = get_random_scalar();
    // let num2 = get_random_scalar();
    let point1 = get_random_g1_point();
    let point2 = get_random_g2_point();
    // let message = "test".as_bytes();
    for _ in 0..user_number {
        let _ = B_G2_add(&point2, &point2);
        let _ = F_G2_mul(&num, &point2);
        let _ = P_paring(&point1, &point2);
        let _ = P_paring(&point1, &point2);
    }
}

pub fn test_aggregate_sign(user_number: usize) {
    let point1 = get_random_g1_point();
    for _ in 0..user_number {
        let _ = A_G1_add(&point1, &point1);
    }
}

pub fn test_aggregate_key(user_number: usize) {
    for _ in 0..user_number {
        let message_vec = vec![format!("123"), format!("223")];
        let _ = L_aggregate(&message_vec);
        let _ = L_aggregate(&message_vec);
    }
}

pub fn test_random_aggregate(user_number: usize) {
    let num = get_random_scalar();
    let point1 = get_random_g1_point();
    for _ in 0..user_number {
        let _ = E_G1_mul(&num, &point1);
        let _ = E_G1_mul(&num, &point1);
    }
}

pub fn test_aggregate_verify(user_number: usize) {
    let num = get_random_scalar();
    let point1 = get_random_g1_point();
    let point2 = get_random_g2_point();
    let _ = P_paring(&point1, &point2);
    let _ = P_paring(&point1, &point2);

    for _ in 0..user_number {
        let _ = B_G2_add(&point2, &point2);
        let _ = F_G2_mul(&num, &point2);
    }
}
