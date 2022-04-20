use crate::{get_random_g1_point, get_random_g2_point, get_random_scalar, A_G1_add, B_G2_add, E_G1_mul, F_G2_mul, L_aggregate, P_paring, Y_scalar_mul, Z_scalar_add, hash_to_g1_point, hash_to_scalar};
use crate::{test_A_G1_add, test_B_G2_add, test_E_G1_mul, test_F_G2_mul, test_hash_to_g1_point, test_hash_to_scalar, test_L_aggregate, test_P_paring, test_Y_scalar_mul, test_Z_scalar_add};
pub fn test_issue_key_gen(user_name: usize) {
    let num = get_random_scalar();
    let point = get_random_g2_point();
    for _ in 0..user_name {
        let _ = F_G2_mul(&num, &point);
        let _ = F_G2_mul(&num, &point);
    }
}

pub fn test_trace_key_gen(user_number: usize) {
    let num = get_random_scalar();
    let point = get_random_g2_point();
    let _ = F_G2_mul(&num, &point);
}


pub fn test_aggregate_key(user_number: usize) {
    for _ in 0..user_number {
        let message_vec = vec![format!("123"), format!("223")];
        let _ = L_aggregate(&message_vec);
        let _ = L_aggregate(&message_vec);
    }
}

pub fn test_cred_request(user_number: usize) {
    let num = get_random_scalar();
    let num2 = get_random_scalar();
    let point1 = get_random_g1_point();
    let point2 = get_random_g2_point();
    let message = "test".as_bytes();
    for _ in 0..user_number {
        let _ = hash_to_g1_point(&message);
        let _ = hash_to_scalar(&message);
        let _ = hash_to_scalar(&message);

        for _ in 0..3 {
            test_E_G1_mul();
        }

        for _ in 0..6 {
            test_F_G2_mul();
        }

        for _ in 0..2 {
            test_B_G2_add();
        }

        for _ in 0..2 {
            test_Y_scalar_mul();
        }

        for _ in 0..2 {
            test_Z_scalar_add();
        }
    }
}


pub fn test_issue(user_number: usize) {
    for _ in 0..user_number {
        for _ in 0..4 {
            test_E_G1_mul();
        }

        for _ in 0..2 {
            test_A_G1_add();
        }

        for _ in 0..3 {
            test_F_G2_mul();
        }
    }
}

pub fn test_cred_aggr(user_number: usize) {
    for _ in 0..user_number {
        test_E_G1_mul();
    }
}

pub fn test_cred_show(user_number: usize) {
    test_hash_to_scalar();
    test_hash_to_g1_point();
    test_Y_scalar_mul();
    test_Z_scalar_add();
    test_P_paring();
    for _ in 0..2 {
        test_E_G1_mul();
    }
    for _ in 0..user_number {
        test_F_G2_mul();
    }

    for _ in 0..user_number-1 {
        test_B_G2_add();
    }
}

pub fn test_cred_verify(user_number: usize) {
    test_A_G1_add();
    for _ in 0..2 {
        test_E_G1_mul();
    }
    for _ in 0..4 {
        test_P_paring();
        test_E_G1_mul();
    }
}

pub fn test_trace(user_number: usize) {
    test_hash_to_g1_point();
    test_F_G2_mul();
    test_B_G2_add();
    for _ in 0..2 {
        test_P_paring();
    }
}

pub fn test_judge(user_number: usize) {
    test_hash_to_g1_point();
    for _ in 0..2 {
        test_P_paring();
    }
}
