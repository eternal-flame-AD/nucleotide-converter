use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use nucleotide_converter::CodeConverter;
use rand::{Rng, SeedableRng};

fn generate_code(n: usize) -> Vec<u8> {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(1);
    let mut code = Vec::new();
    for _ in 0..n {
        code.push(b"ATCGatcg"[rng.random_range(0..8)]);
    }
    code
}

#[derive(Debug, Clone, Copy)]
enum Converter {
    Naive,
    LUT,
    SSE2,
    AVX2,
}

struct Input {
    name: Converter,
    n: usize,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({} nt)", self.name, self.n)
    }
}

fn benchmark_code_converter(c: &mut Criterion) {
    let mut g = c.benchmark_group("code_converter");

    for converter in [
        Converter::Naive,
        Converter::LUT,
        Converter::SSE2,
        Converter::AVX2,
    ] {
        let input = Input {
            name: converter,
            n: 3_000_000,
        };
        g.throughput(Throughput::Elements(input.n as u64));
        g.bench_with_input(
            BenchmarkId::new("code_converter", &input),
            &input,
            |b, input: &Input| {
                let code = generate_code(input.n);
                let mut out = vec![0; code.len()];
                let mut expected = vec![0; code.len()];
                nucleotide_converter::NaiveCodeConverter::default().convert(&code, &mut expected);

                b.iter(|| {
                    match input.name {
                        Converter::SSE2 => {
                            let converter = nucleotide_converter::SSE2CodeConverter::default();
                            converter.convert(&code, &mut out);
                        }
                        Converter::LUT => {
                            let converter = nucleotide_converter::LUTCodeConverter::default();
                            converter.convert(&code, &mut out);
                        }
                        Converter::Naive => {
                            let converter = nucleotide_converter::NaiveCodeConverter::default();
                            converter.convert(&code, &mut out);
                        }
                        Converter::AVX2 => {
                            let converter = nucleotide_converter::AVX2CodeConverter::default();
                            converter.convert(&code, &mut out);
                        }
                    }
                    core::hint::black_box(&out);
                    debug_assert_eq!(out, expected);
                });
            },
        );
    }
}

criterion_group!(benches, benchmark_code_converter);
criterion_main!(benches);
