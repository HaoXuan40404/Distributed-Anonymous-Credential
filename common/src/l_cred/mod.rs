
use crate::{test_A_G1_add, test_B_G2_add, test_E_G1_mul, test_F_G2_mul, test_hash_to_g1_point, test_hash_to_scalar, test_L_aggregate, test_P_paring, test_Y_scalar_mul, test_Z_scalar_add};
pub fn test_issue_key_gen(user_number: usize, attribute_number: usize) {
    for _ in 0..user_number {
        for _ in 0..attribute_number+1 {
            let _ = test_F_G2_mul();
        }
    }
}

pub fn test_trace_key_gen(user_number: usize, attribute_number: usize) {
    test_F_G2_mul();
}


pub fn test_aggregate_key(user_number: usize, attribute_number: usize) {
    for _ in 0..user_number {
        for _ in 0..attribute_number+1 {
            test_L_aggregate();
        }
    }
}

pub fn test_cred_request(user_number: usize, attribute_number: usize) {
        test_hash_to_g1_point();

        for _ in 0..2 {
            test_hash_to_scalar()
        }

        for _ in 0..attribute_number+2 {
            test_E_G1_mul();
        }

        for _ in 0..3 {
            test_Y_scalar_mul();
            test_Z_scalar_add();
        }

        for _ in 0..6 {
            test_F_G2_mul();
        }

        for _ in 0..2 {
            test_B_G2_add();
        }
}


pub fn test_issue(user_number: usize, attribute_number: usize) {
        test_hash_to_g1_point();
        test_hash_to_scalar();

        for _ in 0..attribute_number+3 {
            test_E_G1_mul();
        }

        for _ in 0..3 {
            test_B_G2_add();
        }

        for _ in 0..5 {
            test_F_G2_mul();
        }
}

pub fn test_cred_aggr(user_number: usize, attribute_number: usize) {
    for _ in 0..user_number {
        test_E_G1_mul();
    }
}

pub fn test_cred_show(user_number: usize, attribute_number: usize) {
    test_hash_to_scalar();
    test_hash_to_g1_point();
    test_P_paring();
    test_Y_scalar_mul();
    test_Z_scalar_add();

    for _ in 0..2 {
        test_E_G1_mul();
    }


    for _ in 0..user_number {
        test_F_G2_mul();
    }

}

pub fn test_cred_verify(user_number: usize, attribute_number: usize) {
    test_hash_to_scalar();
    test_hash_to_g1_point();
    for _ in 0..user_number-1 {
        for _ in 0..attribute_number {
            test_B_G2_add();
        }
    }

    for _ in 0..2 {
        test_E_G1_mul();
    }

    for _ in 0..attribute_number+1 {
        test_F_G2_mul();
    }

    for _ in 0..4 {
        test_P_paring();
    }

}

pub fn test_trace(user_number: usize, attribute_number: usize) {
    test_hash_to_g1_point();
    for _ in 0..2 {
        test_P_paring();
    }

    test_F_G2_mul();
}

pub fn test_judge(user_number: usize, attribute_number: usize) {
    test_hash_to_g1_point();
    for _ in 0..2 {
        test_P_paring();
    }
}
