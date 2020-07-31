use kvik::prelude::*;
use rand::prelude::*;

const SIZE_CAP: usize = 100_000;

fn main() {
    let output = std::env::args().nth(1).expect("we need a filename");
    let threads:usize = output.parse::<usize>().unwrap() + 1;
    benchy::Bencher::new()
        .parameters(vec![100_000_000usize])
        // .parameters(vec![100_000_000usize, 200_000_000, 300_000_000])
        //.setup(|s| ((0..s).collect::<Vec<usize>>(), s))
        //.setup(|s| ((0..s).collect::<Vec<usize>>(), s/2-1))
        .setup(|s| {
            let mut input = vec![true ; s];
            *input.choose_mut(&mut rand::thread_rng()).unwrap() = false;
            input
        })
        .postprocess(|r:bool| assert!(!r))
        .balgorithm("seq", &|input| {
            input.iter().all(|x| *x)
        })
        .balgorithm("no_blocks", &|input| {
            input.par_iter().size_limit(SIZE_CAP).all(|x| *x)
        })
        .balgorithm("blocks", &|input| {
            input
                .par_iter()
                .by_blocks(std::iter::successors(Some(SIZE_CAP * threads), |s| {
                    Some(s.saturating_mul(2))
                }))
                .size_limit(SIZE_CAP)
                .all(|x| *x)
        })
        .run(100, output)
        .expect("failed to save logs");
}
