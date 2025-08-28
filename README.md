A basic nucleotide sequence packer and converter.

Demo:

```
> cargo run --release -- ATCGatcgATCGatcgNn
[nucleotide_converter::NaiveCodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 255, 255]
[nucleotide_converter::LUTCodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 255, 255]
[nucleotide_converter::SSE2CodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 255, 255]
[nucleotide_converter::SSSE3CodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 255, 255]
[nucleotide_converter::AVX2CodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 255, 255]
[nucleotide_converter::AVX512VbmiCodeConverter] [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 255, 255]
[nucleotide_converter::custom_alphabet::LUTUnpacker -> nucleotide_converter::custom_alphabet::LUTInPlacePacker] packed -> ATCGatcgATCGatcgNn = [81, 42, 81, 42, 81, 42, 81, 42, ff]
[nucleotide_converter::custom_alphabet::SSSE3Unpacker -> nucleotide_converter::custom_alphabet::SSE41InPlacePacker] packed -> ATCGatcgATCGatcgNn = [81, 42, 81, 42, 81, 42, 81, 42, ff]
```

Benchmark:

```sh
> cargo bench
custom_alphabet_converter/dragmap_pack/LUT (3000000 nt)
                        time:   [860.29 µs 863.63 µs 867.58 µs]
                        thrpt:  [3.4579 Gelem/s 3.4737 Gelem/s 3.4872 Gelem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
Benchmarking custom_alphabet_converter/dragmap_pack_inplace/LUT (3000000 nt): Collecting 100 samples in estim
custom_alphabet_converter/dragmap_pack_inplace/LUT (3000000 nt)
                        time:   [516.50 µs 517.93 µs 520.06 µs]
                        thrpt:  [5.7685 Gelem/s 5.7922 Gelem/s 5.8083 Gelem/s]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
Benchmarking custom_alphabet_converter/dragmap_unpack/LUT (3000000 nt): Collecting 100 samples in estimated 6
custom_alphabet_converter/dragmap_unpack/LUT (3000000 nt)
                        time:   [659.52 µs 664.66 µs 671.40 µs]
                        thrpt:  [4.4683 Gelem/s 4.5136 Gelem/s 4.5487 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe
Benchmarking custom_alphabet_converter/dragmap_pack/SSE41 (3000000 nt): Collecting 100 samples in estimated 5
custom_alphabet_converter/dragmap_pack/SSE41 (3000000 nt)
                        time:   [153.90 µs 153.98 µs 154.06 µs]
                        thrpt:  [19.474 Gelem/s 19.483 Gelem/s 19.493 Gelem/s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
Benchmarking custom_alphabet_converter/dragmap_pack_inplace/SSE41 (3000000 nt): Collecting 100 samples in est
custom_alphabet_converter/dragmap_pack_inplace/SSE41 (3000000 nt)
                        time:   [154.42 µs 155.16 µs 156.01 µs]
                        thrpt:  [19.229 Gelem/s 19.335 Gelem/s 19.428 Gelem/s]
Found 20 outliers among 100 measurements (20.00%)
  18 (18.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high severe
Benchmarking custom_alphabet_converter/dragmap_unpack/SSE41 (3000000 nt): Collecting 100 samples in estimated
custom_alphabet_converter/dragmap_unpack/SSE41 (3000000 nt)
                        time:   [84.956 µs 85.140 µs 85.349 µs]
                        thrpt:  [35.150 Gelem/s 35.236 Gelem/s 35.313 Gelem/s]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
Benchmarking custom_alphabet_converter/dragmap_pack/LUT (100000000 nt): Collecting 100 samples in estimated 5
custom_alphabet_converter/dragmap_pack/LUT (100000000 nt)
                        time:   [28.349 ms 28.418 ms 28.492 ms]
                        thrpt:  [3.5098 Gelem/s 3.5189 Gelem/s 3.5274 Gelem/s]
Found 14 outliers among 100 measurements (14.00%)
  12 (12.00%) high mild
  2 (2.00%) high severe
Benchmarking custom_alphabet_converter/dragmap_pack_inplace/LUT (100000000 nt): Collecting 100 samples in est
custom_alphabet_converter/dragmap_pack_inplace/LUT (100000000 nt)
                        time:   [17.327 ms 17.420 ms 17.544 ms]
                        thrpt:  [5.7000 Gelem/s 5.7405 Gelem/s 5.7715 Gelem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
Benchmarking custom_alphabet_converter/dragmap_unpack/LUT (100000000 nt): Collecting 100 samples in estimated
custom_alphabet_converter/dragmap_unpack/LUT (100000000 nt)
                        time:   [21.552 ms 21.624 ms 21.701 ms]
                        thrpt:  [4.6081 Gelem/s 4.6245 Gelem/s 4.6400 Gelem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
Benchmarking custom_alphabet_converter/dragmap_pack/SSE41 (100000000 nt): Collecting 100 samples in estimated
custom_alphabet_converter/dragmap_pack/SSE41 (100000000 nt)
                        time:   [5.1426 ms 5.1521 ms 5.1607 ms]
                        thrpt:  [19.377 Gelem/s 19.410 Gelem/s 19.445 Gelem/s]
Found 21 outliers among 100 measurements (21.00%)
  12 (12.00%) low severe
  4 (4.00%) high mild
  5 (5.00%) high severe
Benchmarking custom_alphabet_converter/dragmap_pack_inplace/SSE41 (100000000 nt): Collecting 100 samples in e
custom_alphabet_converter/dragmap_pack_inplace/SSE41 (100000000 nt)
                        time:   [5.3018 ms 5.3039 ms 5.3062 ms]
                        thrpt:  [18.846 Gelem/s 18.854 Gelem/s 18.862 Gelem/s]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
Benchmarking custom_alphabet_converter/dragmap_unpack/SSE41 (100000000 nt): Collecting 100 samples in estimat
custom_alphabet_converter/dragmap_unpack/SSE41 (100000000 nt)
                        time:   [3.7368 ms 3.7420 ms 3.7477 ms]
                        thrpt:  [26.683 Gelem/s 26.723 Gelem/s 26.761 Gelem/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

code_converter/code_converter/NaiveToLower (3000000 nt)
                        time:   [222.23 µs 222.43 µs 222.70 µs]
                        thrpt:  [13.471 Gelem/s 13.487 Gelem/s 13.500 Gelem/s]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
code_converter/code_converter/Naive (3000000 nt)
                        time:   [160.75 µs 160.94 µs 161.18 µs]
                        thrpt:  [18.613 Gelem/s 18.641 Gelem/s 18.662 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
code_converter/code_converter/LUT (3000000 nt)
                        time:   [636.30 µs 636.98 µs 637.82 µs]
                        thrpt:  [4.7036 Gelem/s 4.7097 Gelem/s 4.7148 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
code_converter/code_converter/SSE2 (3000000 nt)
                        time:   [152.33 µs 152.49 µs 152.70 µs]
                        thrpt:  [19.647 Gelem/s 19.674 Gelem/s 19.694 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
code_converter/code_converter/SSSE3 (3000000 nt)
                        time:   [100.65 µs 100.80 µs 100.99 µs]
                        thrpt:  [29.706 Gelem/s 29.763 Gelem/s 29.805 Gelem/s]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe
code_converter/code_converter/AVX2 (3000000 nt)
                        time:   [47.860 µs 47.956 µs 48.091 µs]
                        thrpt:  [62.382 Gelem/s 62.558 Gelem/s 62.683 Gelem/s]
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
code_converter/code_converter/AVX512VBMI (3000000 nt)
                        time:   [47.677 µs 47.743 µs 47.826 µs]
                        thrpt:  [62.727 Gelem/s 62.837 Gelem/s 62.924 Gelem/s]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe
code_converter/code_converter/NaiveToLower (100000000 nt)
                        time:   [15.056 ms 15.065 ms 15.074 ms]
                        thrpt:  [6.6339 Gelem/s 6.6381 Gelem/s 6.6420 Gelem/s]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
code_converter/code_converter/Naive (100000000 nt)
                        time:   [11.166 ms 11.183 ms 11.200 ms]
                        thrpt:  [8.9282 Gelem/s 8.9418 Gelem/s 8.9554 Gelem/s]
code_converter/code_converter/LUT (100000000 nt)
                        time:   [35.518 ms 35.551 ms 35.585 ms]
                        thrpt:  [2.8101 Gelem/s 2.8129 Gelem/s 2.8155 Gelem/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
code_converter/code_converter/SSE2 (100000000 nt)
                        time:   [10.263 ms 10.274 ms 10.286 ms]
                        thrpt:  [9.7216 Gelem/s 9.7332 Gelem/s 9.7441 Gelem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
code_converter/code_converter/SSSE3 (100000000 nt)
                        time:   [8.5260 ms 8.5413 ms 8.5592 ms]
                        thrpt:  [11.683 Gelem/s 11.708 Gelem/s 11.729 Gelem/s]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe
code_converter/code_converter/AVX2 (100000000 nt)
                        time:   [8.3107 ms 8.3251 ms 8.3403 ms]
                        thrpt:  [11.990 Gelem/s 12.012 Gelem/s 12.033 Gelem/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
code_converter/code_converter/AVX512VBMI (100000000 nt)
                        time:   [7.7293 ms 7.7496 ms 7.7738 ms]
                        thrpt:  [12.864 Gelem/s 12.904 Gelem/s 12.938 Gelem/s]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe
```

