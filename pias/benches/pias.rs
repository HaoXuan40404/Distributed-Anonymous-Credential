#[macro_use]
extern crate criterion;
use criterion::Criterion;

use bls12_381::{G1Projective, G2Projective};
use pias::{issuer, user, verifier};

fn create_issuer_key_generate_helper(c: &mut Criterion) {
    let label = format!("create_issuer_key_generate_helper");
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let (_, _, _, _) = issuer::issuer_key_generate();
        })
    });
}

fn create_issuer_sign_credential_helper(c: &mut Criterion) {
    let label = format!("create_issuer_sign_credential_helper");
    let user_aux = format!("my id is {}", 1).as_bytes().to_vec();
    let message = format!("my attribute is {}", 1).as_bytes().to_vec();
    let (sk1, sk2, _pk1, _pk2) = issuer::issuer_key_generate();

    c.bench_function(&label, move |b| {
        b.iter(|| {
            let (_, _) = issuer::issuer_sign_credential(&user_aux, &message, &sk1, &sk2);
        })
    });
}

fn create_user_verify_signature_helper(c: &mut Criterion) {
    let label = format!("create_user_verify_signature_helper");
    let user_aux = format!("my id is {}", 1).as_bytes().to_vec();
    let message = format!("my attribute is {}", 1).as_bytes().to_vec();
    let (sk1, sk2, pk1, pk2) = issuer::issuer_key_generate();

    let (h_point, h_aux_point) = issuer::issuer_sign_credential(&user_aux, &message, &sk1, &sk2);

    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = user::user_verify_signature(&pk1, &pk2, &message, &h_point, &h_aux_point);
        })
    });
}

fn create_user_aggregate_signature_helper(c: &mut Criterion, did_count: usize) {
    let label = format!(
        "create_user_aggregate_signature_helper, did_count = {}",
        did_count
    );

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
        let sig_result = user::user_verify_signature(&pk1, &pk2, &message, &h_point, &h_aux_point);
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

    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = user::user_aggregate_signature(&sig2_list);
        })
    });
}

fn create_user_random_signature_helper(c: &mut Criterion) {
    let label = format!("create_user_random_signature_helper, did_count = {}", 3);
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
        let sig_result = user::user_verify_signature(&pk1, &pk2, &message, &h_point, &h_aux_point);
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

    c.bench_function(&label, move |b| {
        b.iter(|| {
            let (_, _) = user::user_random_signature(&user_aux, &sig2_show);
        })
    });
}

fn create_verifier_verify_helper(c: &mut Criterion, did_count: usize) {
    let label = format!(
        "create_verifier_verify_helper, did_count = {}",
        did_count
    );

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
        let sig_result = user::user_verify_signature(&pk1, &pk2, &message, &h_point, &h_aux_point);
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

    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = verifier::verifier_verify(&pk1_list, &pk2_list, &cert1, &cert2, &message_list);
        })
    });
}

fn create_user_aggregate_signature_helper_3(c: &mut Criterion) {
    create_user_aggregate_signature_helper(c, 3);
}

fn create_user_aggregate_signature_helper_10(c: &mut Criterion) {
    create_user_aggregate_signature_helper(c, 10);
}

fn create_user_aggregate_signature_helper_20(c: &mut Criterion) {
    create_user_aggregate_signature_helper(c, 20);
}

fn create_user_aggregate_signature_helper_30(c: &mut Criterion) {
    create_user_aggregate_signature_helper(c, 30);
}

fn create_verifier_verify_helper_3(c: &mut Criterion) {
    create_verifier_verify_helper(c, 3);
}

fn create_verifier_verify_helper_10(c: &mut Criterion) {
    create_verifier_verify_helper(c, 10);
}

fn create_verifier_verify_helper_20(c: &mut Criterion) {
    create_verifier_verify_helper(c, 20);
}

fn create_verifier_verify_helper_30(c: &mut Criterion) {
    create_verifier_verify_helper(c, 30);
}

criterion_group! {
    name = init_bench_test;
    config = Criterion::default().sample_size(10);
targets =
create_issuer_key_generate_helper,
    create_issuer_sign_credential_helper,
    create_user_verify_signature_helper,
    create_user_aggregate_signature_helper_3,
    create_user_aggregate_signature_helper_10,
    create_user_aggregate_signature_helper_20,
    create_user_aggregate_signature_helper_30,
    create_user_random_signature_helper,
    create_verifier_verify_helper_3,
    create_verifier_verify_helper_10,
    create_verifier_verify_helper_20,
    create_verifier_verify_helper_30,
}

criterion_main!(init_bench_test);
