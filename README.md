Demo:

```
cargo run --release -- ATCGatcgATCGatcg
[nucleotide_converter::NaiveCodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]
[nucleotide_converter::LUTCodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]
[nucleotide_converter::SSE2CodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]
[nucleotide_converter::AVX2CodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]
```

Benchmark:

```
cargo bench

code_converter/code_converter/Naive (3000000 nt)
                        time:   [235.91 µs 236.03 µs 236.19 µs]
                        thrpt:  [12.701 Gelem/s 12.710 Gelem/s 12.717 Gelem/s]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
code_converter/code_converter/LUT (3000000 nt)
                        time:   [669.32 µs 669.86 µs 670.43 µs]
                        thrpt:  [4.4748 Gelem/s 4.4785 Gelem/s 4.4822 Gelem/s]
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe
code_converter/code_converter/SSE2 (3000000 nt)
                        time:   [155.19 µs 155.26 µs 155.34 µs]
                        thrpt:  [19.313 Gelem/s 19.323 Gelem/s 19.331 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
code_converter/code_converter/AVX2 (3000000 nt)
                        time:   [56.545 µs 56.906 µs 57.501 µs]
                        thrpt:  [52.173 Gelem/s 52.719 Gelem/s 53.055 Gelem/s]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
```

