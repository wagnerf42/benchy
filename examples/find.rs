use kvik::prelude::*;

const SIZE: usize = 100_000_000;

// return rayon's initial counter for a given number of threads
fn log(t: usize) -> usize {
    (t as f64).log2().ceil() as usize + 1
}

fn bench<T: Fn()->usize>(filename: &str, target_selection: T, threads: usize) {
    benchy::Bencher::new()
        .setup(|| ((0..SIZE).collect::<Vec<usize>>(), target_selection()))
        .postprocess(|(s, f)| assert_eq!(Some(s), f))
        .balgorithm("seq", &|&(ref input, target)| {
            (target, input.iter().find(|&e| *e == target).copied())
        })
        .balgorithm("rayon", &|&(ref input, target): &(
            Vec<usize>,
            usize,
        )| {
            let f = input
                .par_iter()
                .filter(|&e| *e == target)
                .next()
                .rayon(log(threads))
                .reduce_with(|a, _| a)
                .copied();
            (target, f)
        })
        .balgorithm("rayon_blocks", &|&(ref input, target)| {
            let f = input
                .par_iter()
                .filter(|&e| *e == target)
                .next()
                .by_blocks(std::iter::successors(Some(4_000), |old| Some(2 * old)))
                .rayon(log(threads))
                .reduce_with(|a, _| a)
                .copied();

            (target, f)
        })
        .balgorithm("adaptive", &|&(ref input, target): &(
            Vec<usize>,
            usize,
        )| {
            let f = input
                .par_iter()
                .filter(|&e| *e == target)
                .next()
                .adaptive()
                .micro_block_sizes(1_000, 10_000)
                .reduce_with(|a, _| a)
                .copied();
            (target, f)
        })
        .balgorithm("adaptive_blocks", &|&(ref input, target)| {
            let f = input
                .par_iter()
                .filter(|&e| *e == target)
                .next()
                .by_blocks(std::iter::successors(Some(4_000), |old| Some(2 * old)))
                .adaptive()
                .micro_block_sizes(1_000, 10_000)
                .reduce_with(|a, _| a)
                .copied();
            (target, f)
        })
        .run(100, filename)
        .expect("failed to save logs");
}

fn main() {
    let threads_string = std::env::args().nth(1).expect("we need a number of threads");
    let threads:usize = threads_string.parse::<usize>().unwrap();
    bench(&format!("log_find_random_{}.csv", threads), || rand::random::<usize>() % SIZE, threads);
    bench(&format!("log_find_mid_{}.csv", threads), || SIZE/2-1, threads);
}
