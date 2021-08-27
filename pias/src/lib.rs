#[macro_use]
extern crate lazy_static;

pub mod issuer;
pub mod user;
pub mod utils;
pub mod verifier;

#[cfg(test)]
mod tests {
    use super::*;
    use bls12_381::{G1Projective, G2Projective};

    #[test]
    fn test_pias() {
        let did_count = 3;
        let mut pk1_list: Vec<G2Projective> = vec![];
        let mut pk2_list: Vec<G2Projective> = vec![];
        let mut message_list: Vec<Vec<u8>> = vec![];
        // let mut sig1_list: Vec<G1Projective> = vec![];
        let mut sig2_list: Vec<G1Projective> = vec![];
        let user_aux = format!("Id number: 1").as_bytes().to_vec();

        for i in 0..did_count {
            let message = format!("my attribute is {}", i).as_bytes().to_vec();
            let (sk1, sk2, pk1, pk2) = issuer::issuer_key_generate();

            let (h_point, h_aux_point) =
                issuer::issuer_sign_credential(&user_aux, &message, &sk1, &sk2);
            let sig_result =
                user::user_verify_signature(&pk1, &pk2, &message, &h_point, &h_aux_point);
            assert_eq!(sig_result, true);
            let message_false = "my name is 20".as_bytes().to_vec();
            let sig_result_false =
                user::user_verify_signature(&pk1, &pk2, &message_false, &h_point, &h_aux_point);
            assert_eq!(sig_result_false, false);
            pk1_list.push(pk1);
            pk2_list.push(pk2);
            message_list.push(message);
            sig2_list.push(h_aux_point);
        }

        let sig2_show = user::user_aggregate_signature(&sig2_list);
        let (cert1, cert2) = user::user_random_signature(&user_aux, &sig2_show);
        let result = verifier::verifier_verify(&pk1_list, &pk2_list, &cert1, &cert2, &message_list);
        assert_eq!(result, true);
        message_list[did_count - 1] = format!("my name is {}", did_count).as_bytes().to_vec();
        let result_false =
            verifier::verifier_verify(&pk1_list, &pk2_list, &cert1, &cert2, &message_list);
        assert_eq!(result_false, false);
    }
}
