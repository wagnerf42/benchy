use itertools::Itertools;
use std::io::prelude::*;

pub struct Bencher<'a, S, P, I, R> {
    setup: S,
    postprocess: P,
    algorithms: Vec<(&'static str, &'a dyn Fn(I) -> R)>,
    balgorithms: Vec<(&'static str, &'a dyn Fn(&I) -> R)>,
    phantom_input: std::marker::PhantomData<I>,
    phantom_output: std::marker::PhantomData<R>,
}

impl<'a> Bencher<'a, fn(()), fn(()), (), ()> {
    pub fn new() -> Self {
        Bencher {
            setup: |_| (),
            postprocess: |_| (),
            algorithms: Vec::new(),
            balgorithms: Vec::new(),
            phantom_input: Default::default(),
            phantom_output: Default::default(),
        }
    }
}

impl<'a, S, P, I, R> Bencher<'a, S, P, I, R> {
    pub fn setup<NI, NS>(self, setup: NS) -> Bencher<'a, NS, fn(()), NI, ()>
    where
        NS: Fn() -> NI,
    {
        Bencher {
            setup,
            postprocess: |_| (),
            algorithms: Vec::new(),
            balgorithms: Vec::new(),
            phantom_input: Default::default(),
            phantom_output: Default::default(),
        }
    }
    pub fn postprocess<NR, NP>(self, postprocess: NP) -> Bencher<'a, S, NP, I, NR>
    where
        NP: Fn(NR),
    {
        Bencher {
            setup: self.setup,
            postprocess,
            algorithms: Vec::new(),
            balgorithms: Vec::new(),
            phantom_input: Default::default(),
            phantom_output: Default::default(),
        }
    }
    pub fn algorithm(mut self, name: &'static str, algo: &'a dyn Fn(I) -> R) -> Self {
        self.algorithms.push((name, algo));
        self
    }
    pub fn balgorithm(mut self, name: &'static str, algo: &'a dyn Fn(&I) -> R) -> Self {
        self.balgorithms.push((name, algo));
        self
    }
}

impl<'a, S, P, I, R> Bencher<'a, S, P, I, R>
where
    S: Fn() -> I,
    P: Fn(R),
{
    pub fn run<F: AsRef<std::path::Path>>(
        self,
        runs_number: usize,
        output: F,
    ) -> Result<(), std::io::Error> {
        let mut output_file = std::fs::File::create(output)?;
        writeln!(
            output_file,
            "#{}",
            self.balgorithms.iter().map(|(label, _)| *label).join(",")
        )?;
        for _ in 0..runs_number {
            let input = (self.setup)();
            let out = self
                .balgorithms
                .iter()
                .map(|a| {
                    let start = std::time::Instant::now();
                    let output = (a.1)(&input);
                    let end = start.elapsed();
                    (self.postprocess)(output);
                    end.as_nanos().to_string()
                })
                .join(",");
            writeln!(output_file, "{}", out)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
