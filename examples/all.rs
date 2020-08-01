//! all bench with random target
use kvik::prelude::*;
use rand::prelude::*;

const SIZE_CAP: usize = 100_000;

// return rayon's initial counter for a given number of threads
fn log(t: usize) -> usize {
    (t as f64).log2().ceil() as usize + 1
}

fn main() {
    let threads_string = std::env::args().nth(1).expect("we need a number of threads");
    let threads:usize = threads_string.parse::<usize>().unwrap();
    let output = format!("log_all_{}.csv", threads);
    benchy::Bencher::new()
        .setup(|| {
            let mut input = vec![true ; 100_000_000];
            *input.choose_mut(&mut rand::thread_rng()).unwrap() = false;
            input
        })
        .postprocess(|r:bool| assert!(!r))
        .balgorithm("seq", &|input| {
            input.iter().all(|x| *x)
        })
        .balgorithm("size_limit", &|input| {
            input.par_iter().size_limit(SIZE_CAP).all(|x| *x)
        })
        .balgorithm("rayon", &|input| {
            input.par_iter().rayon(log(threads)).all(|x| *x)
        })
        .balgorithm("adaptive", &|input| {
            input.par_iter().adaptive().all(|x| *x)
        })
        .balgorithm("blocks_size_limit", &|input| {
            input
                .par_iter()
                .by_blocks(std::iter::successors(Some(SIZE_CAP * threads), |s| {
                    Some(s.saturating_mul(2))
                }))
                .size_limit(SIZE_CAP)
                .all(|x| *x)
        })
        .balgorithm("blocks_rayon", &|input| {
            input
                .par_iter()
                .by_blocks(std::iter::successors(Some(SIZE_CAP * threads), |s| {
                    Some(s.saturating_mul(2))
                }))
                .rayon(log(threads))
                .all(|x| *x)
        })
        .balgorithm("blocks_adaptive", &|input| {
            input
                .par_iter()
                .by_blocks(std::iter::successors(Some(SIZE_CAP * threads), |s| {
                    Some(s.saturating_mul(2))
                }))
                .adaptive()
                .all(|x| *x)
        })
        .run(10, output)
        .expect("failed to save logs");
}
