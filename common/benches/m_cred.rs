#[macro_use]
extern crate criterion;
use criterion::Criterion;

use common::l_cred;

fn create_test_issue_keygen_helper(c: &mut Criterion, user_number: usize, attribute_number: usize) {
    let mut label = format!("create_test_issue_keygen_helper, user_number: {}, attribute_number: {}", user_number, attribute_number);
    label += "-lcred";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = l_cred::test_issue_key_gen(user_number, attribute_number);
        })
    });
}


fn create_trace_keygen_helper(c: &mut Criterion, user_number: usize, attribute_number: usize) {
    let mut label = format!("create_trace_keygen_helper, user_number: {}, attribute_number: {}", user_number, attribute_number);
    label += "-lcred";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = l_cred::test_trace_key_gen(user_number, attribute_number);
        })
    });
}

fn create_test_aggr_key_helper(c: &mut Criterion, user_number: usize, attribute_number: usize) {
    let mut label = format!("create_test_aggr_key_helper, user_number: {}, attribute_number: {}", user_number, attribute_number);
    label += "-lcred";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = l_cred::test_aggregate_key(user_number, attribute_number);
        })
    });
}

fn create_test_cred_request_helper(c: &mut Criterion, user_number: usize, attribute_number: usize) {
    let mut label = format!("create_test_cred_request_helper, user_number: {}, attribute_number: {}", user_number, attribute_number);
    label += "-lcred";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = l_cred::test_cred_request(user_number, attribute_number);
        })
    });
}


fn create_test_issue_helper(c: &mut Criterion, user_number: usize, attribute_number: usize) {
    let mut label = format!("create_test_issue_helper, user_number: {}, attribute_number: {}", user_number, attribute_number);
    label += "-lcred";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = l_cred::test_issue(user_number, attribute_number);
        })
    });
}

fn create_test_cred_aggr_helper(c: &mut Criterion, user_number: usize, attribute_number: usize) {
    let mut label = format!("create_test_cred_aggr_helper, user_number: {}, attribute_number: {}", user_number, attribute_number);
    label += "-lcred";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = l_cred::test_cred_aggr(user_number, attribute_number);
        })
    });
}

fn create_test_cred_show_helper(c: &mut Criterion, user_number: usize, attribute_number: usize) {
    let mut label = format!("create_test_cred_show_helper, user_number: {}, attribute_number: {}", user_number, attribute_number);
    label += "-lcred";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = l_cred::test_cred_show(user_number, attribute_number);
        })
    });
}


fn create_test_cred_verify_helper(c: &mut Criterion, user_number: usize, attribute_number: usize) {
    let mut label = format!("create_test_cred_verify_helper, user_number: {}, attribute_number: {}", user_number, attribute_number);
    label += "-lcred";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = l_cred::test_cred_verify(user_number, attribute_number);
        })
    });
}



fn create_test_trace_helper(c: &mut Criterion, user_number: usize, attribute_number: usize) {
    let mut label = format!("create_test_trace_helper, user_number: {}, attribute_number: {}", user_number, attribute_number);
    label += "-lcred";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = l_cred::test_trace(user_number, attribute_number);
        })
    });
}

fn create_test_judge_helper(c: &mut Criterion, user_number: usize, attribute_number: usize) {
    let mut label = format!("create_test_judge_helper, user_number: {}, attribute_number: {}", user_number, attribute_number);
    label += "-lcred";
    c.bench_function(&label, move |b| {
        b.iter(|| {
            let _ = l_cred::test_judge(user_number, attribute_number);
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


fn create_test_issue_keygen_helper_1_1000(c: &mut Criterion) {
    for i in 0..11 {
        let user_number = get_user_number(i);
        create_test_issue_keygen_helper(c, user_number, 20);
    }
}

fn create_trace_keygen_helper_1_1000(c: &mut Criterion) {
    for i in 0..11 {
        let user_number = get_user_number(i);
        create_trace_keygen_helper(c, user_number, 20);
    }
}

fn create_test_aggr_key_helper_1_1000(c: &mut Criterion) {
    for i in 0..11 {
        let user_number = get_user_number(i);
        create_test_aggr_key_helper(c, user_number, 20);
    }
}

fn create_test_cred_request_helper_1_1000(c: &mut Criterion) {
    for i in 0..11 {
        let user_number = get_user_number(i);
        create_test_cred_request_helper(c, user_number, 20);
    }
}


fn create_test_issue_helper_1_1000(c: &mut Criterion) {
    for i in 0..11 {
        let user_number = get_user_number(i);
        create_test_issue_helper(c, user_number, 20);
    }
}


fn create_test_cred_aggr_helper_1_1000(c: &mut Criterion) {
    for i in 0..11 {
        let user_number = get_user_number(i);
        create_test_cred_aggr_helper(c, user_number, 20);
    }
}

fn create_test_cred_show_helper_1_1000(c: &mut Criterion) {
    for i in 0..11 {
        let user_number = get_user_number(i);
        create_test_cred_show_helper(c, user_number, 20);
    }
}

fn create_test_cred_verify_helper_1_1000(c: &mut Criterion) {
    for i in 0..11 {
        let user_number = get_user_number(i);
        create_test_cred_verify_helper(c, user_number, 20);
    }
}

fn create_test_trace_helper_1_1000(c: &mut Criterion) {
    for i in 0..11 {
        let user_number = get_user_number(i);
        create_test_trace_helper(c, user_number, 20);
    }
}

fn create_test_judge_helper_1_1000(c: &mut Criterion) {
    for i in 0..11 {
        let user_number = get_user_number(i);
        create_test_judge_helper(c, user_number, 20);
    }
}


fn create_test_issue_keygen_helper_k_10_100(c: &mut Criterion) {
    for i in 0..11 {
        let attribute_number = i*10;
        let user_number = 20;
        create_test_issue_keygen_helper(c, user_number, attribute_number);
    }
}

fn create_trace_keygen_helper_k_10_100(c: &mut Criterion) {
    for i in 0..11 {
        let attribute_number = i*10;
        let user_number = 20;
        create_trace_keygen_helper(c, user_number, attribute_number);
    }
}

fn create_test_aggr_key_helper_k_10_100(c: &mut Criterion) {
    for i in 0..11 {
        let attribute_number = i*10;
        let user_number = 20;
        create_test_aggr_key_helper(c, user_number, attribute_number);
    }
}

fn create_test_cred_request_helper_k_10_100(c: &mut Criterion) {
    for i in 0..11 {
        let attribute_number = i*10;
        let user_number = 20;
        create_test_cred_request_helper(c, user_number, attribute_number);
    }
}


fn create_test_issue_helper_k_10_100(c: &mut Criterion) {
    for i in 1..11usize {
        let attribute_number = i*10;
        let user_number = 20;
        create_test_issue_helper(c, user_number, attribute_number);
    }
}


fn create_test_cred_aggr_helper_k_10_100(c: &mut Criterion) {
    for i in 1..11usize {
        let attribute_number = i*10;
        let user_number = 20;
        create_test_cred_aggr_helper(c, user_number, attribute_number);
    }
}

fn create_test_cred_show_helper_k_10_100(c: &mut Criterion) {
    for i in 1..11usize {
        let attribute_number = i*10;
        let user_number = 20;
        create_test_cred_show_helper(c, user_number, attribute_number);
    }
}

fn create_test_cred_verify_helper_k_10_100(c: &mut Criterion) {
    for i in 1..11usize {
        let attribute_number = i*10;
        let user_number = 20;
        create_test_cred_verify_helper(c, user_number, attribute_number);
    }
}

fn create_test_trace_helper_k_10_100(c: &mut Criterion) {
    for i in 1..11usize {
        let attribute_number = i*10;
        let user_number = 20;
        create_test_trace_helper(c, user_number, attribute_number);
    }
}

fn create_test_judge_helper_k_10_100(c: &mut Criterion) {
    for i in 1..11usize {
        let attribute_number = i*10;
        let user_number = 20;
        create_test_judge_helper(c, user_number, attribute_number);
    }
}


criterion_group! {
    name = init_bench_test;
    config = Criterion::default().sample_size(10);
targets =
    create_test_issue_keygen_helper_1_1000,
    create_trace_keygen_helper_1_1000,
    create_test_aggr_key_helper_1_1000,
    create_test_cred_request_helper_1_1000,
    create_test_issue_helper_1_1000,
    create_test_cred_aggr_helper_1_1000,
    create_test_cred_show_helper_1_1000,
    create_test_cred_verify_helper_1_1000,
    create_test_trace_helper_1_1000,
    create_test_judge_helper_1_1000,
    create_test_issue_keygen_helper_k_10_100,
    create_trace_keygen_helper_k_10_100,
    create_test_aggr_key_helper_k_10_100,
    create_test_cred_request_helper_k_10_100,
    create_test_issue_helper_k_10_100,
    create_test_cred_aggr_helper_k_10_100,
    create_test_cred_show_helper_k_10_100,
    create_test_cred_verify_helper_k_10_100,
    create_test_trace_helper_k_10_100,
    create_test_judge_helper_k_10_100,
}

criterion_main!(init_bench_test);
