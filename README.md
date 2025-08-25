Demo:

```
> cargo run --release -- ATCGatcgATCGatcg
[nucleotide_converter::NaiveCodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]
[nucleotide_converter::LUTCodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]
[nucleotide_converter::SSE2CodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]
[nucleotide_converter::AVX2CodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]
```

Benchmark:

```sh
> cargo bench

code_converter/code_converter/Naive (3000000 nt)
                        time:   [164.17 µs 164.31 µs 164.53 µs]
                        thrpt:  [18.234 Gelem/s 18.258 Gelem/s 18.273 Gelem/s]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
code_converter/code_converter/LUT (3000000 nt)
                        time:   [685.33 µs 686.34 µs 687.84 µs]
                        thrpt:  [4.3615 Gelem/s 4.3710 Gelem/s 4.3775 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
code_converter/code_converter/SSE2 (3000000 nt)
                        time:   [155.22 µs 155.27 µs 155.33 µs]
                        thrpt:  [19.314 Gelem/s 19.321 Gelem/s 19.327 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe
code_converter/code_converter/AVX2 (3000000 nt)
                        time:   [57.064 µs 57.203 µs 57.374 µs]
                        thrpt:  [52.288 Gelem/s 52.445 Gelem/s 52.573 Gelem/s]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
```

