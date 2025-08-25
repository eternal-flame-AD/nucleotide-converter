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
                        time:   [224.08 µs 224.29 µs 224.56 µs]
                        thrpt:  [13.359 Gelem/s 13.375 Gelem/s 13.388 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
code_converter/code_converter/Naive (3000000 nt)
                        time:   [162.37 µs 162.51 µs 162.70 µs]
                        thrpt:  [18.439 Gelem/s 18.460 Gelem/s 18.476 Gelem/s]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
code_converter/code_converter/LUT (3000000 nt)
                        time:   [636.88 µs 637.39 µs 638.08 µs]
                        thrpt:  [4.7016 Gelem/s 4.7067 Gelem/s 4.7105 Gelem/s]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
code_converter/code_converter/SSE2 (3000000 nt)
                        time:   [151.51 µs 151.68 µs 151.90 µs]
                        thrpt:  [19.750 Gelem/s 19.778 Gelem/s 19.800 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
code_converter/code_converter/AVX2 (3000000 nt)
                        time:   [49.491 µs 49.717 µs 49.992 µs]
                        thrpt:  [60.010 Gelem/s 60.342 Gelem/s 60.617 Gelem/s]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
code_converter/code_converter/AVX512VBMI (3000000 nt)
                        time:   [47.380 µs 47.454 µs 47.544 µs]
                        thrpt:  [63.099 Gelem/s 63.220 Gelem/s 63.318 Gelem/s]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
code_converter/code_converter/NaiveToLower (100000000 nt)
                        time:   [14.795 ms 14.808 ms 14.822 ms]
                        thrpt:  [6.7469 Gelem/s 6.7530 Gelem/s 6.7588 Gelem/s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
code_converter/code_converter/Naive (100000000 nt)
                        time:   [11.117 ms 11.128 ms 11.139 ms]
                        thrpt:  [8.9777 Gelem/s 8.9864 Gelem/s 8.9951 Gelem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
code_converter/code_converter/LUT (100000000 nt)
                        time:   [35.660 ms 35.688 ms 35.716 ms]
                        thrpt:  [2.7999 Gelem/s 2.8021 Gelem/s 2.8042 Gelem/s]
code_converter/code_converter/SSE2 (100000000 nt)
                        time:   [10.146 ms 10.161 ms 10.176 ms]
                        thrpt:  [9.8266 Gelem/s 9.8418 Gelem/s 9.8563 Gelem/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
code_converter/code_converter/AVX2 (100000000 nt)
                        time:   [8.2613 ms 8.2756 ms 8.2909 ms]
                        thrpt:  [12.061 Gelem/s 12.084 Gelem/s 12.105 Gelem/s]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
code_converter/code_converter/AVX512VBMI (100000000 nt)
                        time:   [7.9321 ms 7.9601 ms 7.9940 ms]
                        thrpt:  [12.509 Gelem/s 12.563 Gelem/s 12.607 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low mild
  7 (7.00%) high mild
  4 (4.00%) high severe
```

