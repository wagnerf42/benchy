use itertools::Itertools;
use std::io::prelude::*;

pub struct Bencher<'a, S, P, I, R, Q> {
    setup: S,
    postprocess: P,
    input_parameters: Q,
    algorithms: Vec<(&'static str, &'a dyn Fn(I) -> R)>,
    balgorithms: Vec<(&'static str, &'a dyn Fn(&I) -> R)>,
    phantom_input: std::marker::PhantomData<I>,
    phantom_output: std::marker::PhantomData<R>,
}

impl<'a> Bencher<'a, fn(()), fn(()), (), (), std::iter::Once<()>> {
    pub fn new() -> Self {
        Bencher {
            setup: |_| (),
            postprocess: |_| (),
            input_parameters: std::iter::once(()),
            algorithms: Vec::new(),
            balgorithms: Vec::new(),
            phantom_input: Default::default(),
            phantom_output: Default::default(),
        }
    }
}

impl<'a, S, P, I, R, Q: IntoIterator> Bencher<'a, S, P, I, R, Q> {
    pub fn parameters<NQ: IntoIterator>(
        self,
        input_parameters: NQ,
    ) -> Bencher<'a, fn(NQ::Item), fn(()), (), (), NQ> {
        Bencher {
            setup: |_| (),
            postprocess: |_| (),
            input_parameters,
            algorithms: Vec::new(),
            balgorithms: Vec::new(),
            phantom_input: Default::default(),
            phantom_output: Default::default(),
        }
    }
    pub fn setup<NI, NS>(self, setup: NS) -> Bencher<'a, NS, fn(()), NI, (), Q>
    where
        NS: Fn(Q::Item) -> NI,
    {
        Bencher {
            setup,
            postprocess: |_| (),
            input_parameters: self.input_parameters,
            algorithms: Vec::new(),
            balgorithms: Vec::new(),
            phantom_input: Default::default(),
            phantom_output: Default::default(),
        }
    }
    pub fn postprocess<NR, NP>(self, postprocess: NP) -> Bencher<'a, S, NP, I, NR, Q>
    where
        NP: Fn(NR),
    {
        Bencher {
            setup: self.setup,
            postprocess,
            input_parameters: self.input_parameters,
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

impl<'a, S, P, I, R, PAR: std::fmt::Display + Copy, Q: IntoIterator<Item = PAR>>
    Bencher<'a, S, P, I, R, Q>
where
    S: Fn(PAR) -> I,
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
            "# parameter run_number {}...",
            self.balgorithms.iter().map(|(label, _)| *label).join(" ")
        )?;
        for parameter in self.input_parameters {
            for run_number in 0..runs_number {
                write!(output_file, "{} {}", parameter, run_number)?;
                let input = (self.setup)(parameter);
                for algorithm in &self.balgorithms {
                    let start = std::time::Instant::now();
                    let output = (algorithm.1)(&input);
                    let end = start.elapsed();
                    (self.postprocess)(output);
                    write!(output_file, " {}", end.as_nanos())?;
                }
                writeln!(output_file, "")?;
            }
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
