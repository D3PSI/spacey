use criterion::{criterion_group, criterion_main, Criterion};
use spacey::Parser;
use std::error::Error;

pub fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        b.iter(|| -> Result<(), Box<dyn Error>> {
            let mut parser = Parser::new("ws/quine.ws")?;
            parser.into_iter().for_each(|_instr| {});

            Ok(())
        })
    });
}

criterion_group!(parser, parse_benchmark,);
criterion_main!(parser);
