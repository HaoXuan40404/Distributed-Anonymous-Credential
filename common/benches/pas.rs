#[macro_use]
extern crate criterion;
use criterion::Criterion;

use common::part_as;

fn create_test_key_gen_helper(c: &mut Criterion, user_number: usize) {
    let mut label = format!("crate_test_key_gen_helper, user_number: {}", user_number);
    label += "-pas";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = part_as::test_key_gen(user_number);
        })
    });
}


fn create_test_sign_helper(c: &mut Criterion, user_number: usize) {
    let mut label = format!("create_test_sign_helper, user_number: {}", user_number);
    label += "-pas";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = part_as::test_sign(user_number);
        })
    });
}

fn create_test_verify_helper(c: &mut Criterion, user_number: usize) {
    let mut label = format!("create_test_verify_helper, user_number: {}", user_number);
    label += "-pas";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = part_as::test_verify(user_number);
        })
    });
}

fn create_test_aggregate_sign_helper(c: &mut Criterion, user_number: usize) {
    let mut label = format!("test_aggregate_sign, user_number: {}", user_number);
    label += "-pas";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = part_as::test_aggregate_sign(user_number);
        })
    });
}


fn create_test_aggregate_key_helper(c: &mut Criterion, user_number: usize) {
    let mut label = format!("test_aggregate_key, user_number: {}", user_number);
    label += "-pas";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = part_as::test_aggregate_key(user_number);
        })
    });
}

fn create_test_random_aggregate_helper(c: &mut Criterion, user_number: usize) {
    let mut label = format!("test_random_aggregate, user_number: {}", user_number);
    label += "-pas";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = part_as::test_random_aggregate(user_number);
        })
    });
}

fn create_test_aggregate_verify_helper(c: &mut Criterion, user_number: usize) {
    let mut label = format!("test_aggregate_verify, user_number: {}", user_number);
    label += "-pas";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = part_as::test_aggregate_verify(user_number);
        })
    });
}


fn get_user_number(idx: usize) -> usize {
    let user_number;
    if idx == 0 {
        user_number = 1;
    }
    else {
        user_number = idx*10;
    }
    user_number
}
fn create_test_key_gen_helper_1_1000(c: &mut Criterion) {
    for i in 0..101 {
        let user_number = get_user_number(i);
        create_test_key_gen_helper(c, user_number);
    }
}

fn create_test_sign_helper_1_1000(c: &mut Criterion) {
    for i in 0..101 {
        let user_number = get_user_number(i);
        create_test_sign_helper(c, user_number);
    }
}

fn create_test_verify_helper_1_1000(c: &mut Criterion) {
    for i in 0..101 {
        let user_number = get_user_number(i);
        create_test_verify_helper(c, user_number);
    }
}


fn create_test_aggregate_sign_helper_1_1000(c: &mut Criterion) {
    for i in 0..101 {
        let user_number = get_user_number(i);
        create_test_aggregate_sign_helper(c, user_number);
    }
}

fn create_test_aggregate_key_helper_1_1000(c: &mut Criterion) {
    for i in 0..101 {
        let user_number = get_user_number(i);
        create_test_aggregate_key_helper(c, user_number);
    }
}

fn create_test_random_aggregate_helper_1_1000(c: &mut Criterion) {
    for i in 0..101 {
        let user_number = get_user_number(i);
        create_test_random_aggregate_helper(c, user_number);
    }
}

fn create_test_aggregate_verify_helper_1_1000(c: &mut Criterion) {
    for i in 0..101 {
        let user_number = get_user_number(i);
        create_test_aggregate_verify_helper(c, user_number);
    }
}

criterion_group! {
    name = init_bench_test;
    config = Criterion::default().sample_size(10);
targets =
    create_test_key_gen_helper_1_1000,
    create_test_sign_helper_1_1000,
    create_test_verify_helper_1_1000,
    create_test_aggregate_sign_helper_1_1000,
    create_test_aggregate_key_helper_1_1000,
    create_test_random_aggregate_helper_1_1000,
    create_test_aggregate_verify_helper_1_1000,

}

criterion_main!(init_bench_test);
