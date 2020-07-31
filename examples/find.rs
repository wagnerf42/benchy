use kvik::prelude::*;

fn main() {
    let output = std::env::args().nth(1).expect("we need a filename");
    let threads:usize = output.parse().unwrap();
    benchy::Bencher::new()
        .parameters(vec![100_000_000usize])
        // .parameters(vec![100_000_000usize, 200_000_000, 300_000_000])
        .setup(|s| ((0..s).collect::<Vec<usize>>(), rand::random::<usize>() % s))
        .postprocess(|(s, f)| assert_eq!(Some(s), f))
        .balgorithm("no_blocks", &|&(ref input, target): &(
            Vec<usize>,
            usize,
        )| {
            let f = input
                .par_iter()
                .filter(|&e| *e == target)
                .next()
                .rayon(2)
                .reduce_with(|a, _| a)
                .copied();
            (target, f)
        })
        .balgorithm("blocks", &|&(ref input, target)| {
            let f = input
                .par_iter()
                .filter(|&e| *e == target)
                .next()
                .by_blocks(std::iter::successors(Some(4_000), |old| Some(2 * old)))
                .rayon(2)
                .reduce_with(|a, _| a)
                .copied();

            (target, f)
        })
        .balgorithm("no_blocks_adapt", &|&(ref input, target): &(
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
        .balgorithm("blocks_adapt", &|&(ref input, target)| {
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
        .run(100, output)
        .expect("failed to save logs");
}
