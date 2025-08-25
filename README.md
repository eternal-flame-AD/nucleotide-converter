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

code_converter/code_converter/NaiveToLower (3000000 nt)
                        time:   [269.96 µs 270.23 µs 270.57 µs]
                        thrpt:  [11.088 Gelem/s 11.102 Gelem/s 11.113 Gelem/s]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe
code_converter/code_converter/Naive (3000000 nt)
                        time:   [210.86 µs 211.14 µs 211.47 µs]
                        thrpt:  [14.186 Gelem/s 14.209 Gelem/s 14.227 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
code_converter/code_converter/LUT (3000000 nt)
                        time:   [678.28 µs 680.21 µs 683.14 µs]
                        thrpt:  [4.3915 Gelem/s 4.4104 Gelem/s 4.4229 Gelem/s]
Found 20 outliers among 100 measurements (20.00%)
  7 (7.00%) high mild
  13 (13.00%) high severe
code_converter/code_converter/SSE2 (3000000 nt)
                        time:   [195.90 µs 196.07 µs 196.27 µs]
                        thrpt:  [15.285 Gelem/s 15.301 Gelem/s 15.314 Gelem/s]
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe
code_converter/code_converter/AVX2 (3000000 nt)
                        time:   [95.698 µs 96.000 µs 96.274 µs]
                        thrpt:  [31.161 Gelem/s 31.250 Gelem/s 31.349 Gelem/s]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe
code_converter/code_converter/AVX512VBMI (3000000 nt)
                        time:   [89.508 µs 89.811 µs 90.135 µs]
                        thrpt:  [33.284 Gelem/s 33.403 Gelem/s 33.517 Gelem/s]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
code_converter/code_converter/NaiveToLower (100000000 nt)
                        time:   [21.499 ms 21.571 ms 21.650 ms]
                        thrpt:  [4.6188 Gelem/s 4.6359 Gelem/s 4.6514 Gelem/s]
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe
code_converter/code_converter/Naive (100000000 nt)
                        time:   [18.264 ms 18.282 ms 18.301 ms]
                        thrpt:  [5.4643 Gelem/s 5.4699 Gelem/s 5.4754 Gelem/s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
code_converter/code_converter/LUT (100000000 nt)
                        time:   [45.417 ms 45.588 ms 45.756 ms]
                        thrpt:  [2.1855 Gelem/s 2.1936 Gelem/s 2.2018 Gelem/s]
code_converter/code_converter/SSE2 (100000000 nt)
                        time:   [15.828 ms 15.888 ms 15.951 ms]
                        thrpt:  [6.2694 Gelem/s 6.2941 Gelem/s 6.3178 Gelem/s]
Found 10 outliers among 100 measurements (10.00%)
  10 (10.00%) high mild
code_converter/code_converter/AVX2 (100000000 nt)
                        time:   [14.701 ms 14.762 ms 14.826 ms]
                        thrpt:  [6.7451 Gelem/s 6.7743 Gelem/s 6.8021 Gelem/s]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
code_converter/code_converter/AVX512VBMI (100000000 nt)
                        time:   [14.489 ms 14.535 ms 14.581 ms]
                        thrpt:  [6.8584 Gelem/s 6.8801 Gelem/s 6.9018 Gelem/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

