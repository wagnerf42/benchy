use kvik::prelude::*;

fn main() {
    let output = std::env::args().nth(1).expect("we need a filename");
    benchy::Bencher::new()
        .parameters(vec![10_000_000usize, 20_000_000, 30_000_000])
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
        .run(100, output)
        .expect("failed to save logs");
}
