use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use nucleotide_converter::CodeConverter;
use rand::{Rng, SeedableRng};

fn generate_code(rng: &mut impl Rng, out: &mut [u8]) {
    for i in 0..out.len() {
        out[i] = b"ATCGatcgNn"[rng.random_range(0..10)];
    }
}

#[derive(Debug, Clone, Copy)]
enum Converter {
    Naive,
    NaiveToLower,
    LUT,
    SSE2,
    SSSE3,
    AVX2,
    AVX512VBMI,
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

    let mut rng = rand::rngs::SmallRng::seed_from_u64(1);
    for n in [3_000_000, 100_000_000] {
        for converter in [
            Converter::NaiveToLower,
            Converter::Naive,
            Converter::LUT,
            Converter::SSE2,
        ]
        .into_iter()
        .chain(
            core::iter::once(Converter::SSSE3)
                .filter(|_| std::arch::is_x86_feature_detected!("ssse3")),
        )
        .chain(
            core::iter::once(Converter::AVX2)
                .filter(|_| std::arch::is_x86_feature_detected!("avx2")),
        )
        .chain(
            core::iter::once(Converter::AVX512VBMI)
                .filter(|_| std::arch::is_x86_feature_detected!("avx512vbmi")),
        ) {
            let input = Input {
                name: converter,
                n: n,
            };
            g.throughput(Throughput::Elements(input.n as u64));
            let mut code = vec![0; input.n];
            g.bench_with_input(
                BenchmarkId::new("code_converter", &input),
                &input,
                |b, input: &Input| {
                    generate_code(&mut rng, &mut code);
                    let mut out = vec![0; code.len()];
                    let mut expected = vec![0; code.len()];
                    nucleotide_converter::NaiveCodeConverter::default()
                        .convert(&code, &mut expected);

                    b.iter(|| {
                        match input.name {
                            Converter::SSE2 => {
                                let converter = nucleotide_converter::SSE2CodeConverter::default();
                                converter.convert(&code, &mut out);
                            }
                            Converter::SSSE3 => {
                                let converter = nucleotide_converter::SSSE3CodeConverter::default();
                                converter.convert(&code, &mut out);
                            }
                            Converter::NaiveToLower => {
                                let converter =
                                    nucleotide_converter::NaiveToLowerCodeConverter::default();
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
                            Converter::AVX512VBMI => {
                                let converter =
                                    nucleotide_converter::AVX512VbmiCodeConverter::default();
                                converter.convert(&code, &mut out);
                            }
                        }
                        core::hint::black_box(&out);

                        if cfg!(debug_assertions) && out != expected {
                            panic!(
                                "out != expected (converter: {:?}) (first 5 nt: {:?}, out: {:?}, expected: {:?}, first mismatch: {:?})",
                                input.name,
                                &code[..5],
                                &out[..5],
                                &expected[..5],
                                out.iter().zip(expected.iter()).enumerate().find_map(|(i, (a, b))| {
                                    if a != b {
                                        Some(i)
                                    } else {
                                        None
                                    }
                                }),
                            );
                        }
                    });
                },
            );
        }
    }
}

criterion_group!(benches, benchmark_code_converter);
criterion_main!(benches);
