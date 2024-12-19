# rust-lcp-arrays

 - rust lcp arrays 
 - kasai algorithm says that use a inverse suffix tree but that is cost consuming.Instead, i implemented a string compression way so that it estimates the proportion of the
match based on the ith offset for the i+1 offset.
 - general note: Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version.
  

 ```
 cargo build 
 
 ```
 ```
➜ gauravsablok  rust-suffix-lcp git:(main) ✗ ./target/debug/rust-lcp-arrays -h
Usage: rust-lcp-arrays <LCPARRAY_ARG>

Arguments:
  <LCPARRAY_ARG>  please provide the path to the fastq file

Options:
  -h, --help     Print help
  -V, --version  Print version
```
- to run the binary 
```
./rust-lcp-arrays ./sample-files/sample.fasta
{[28, 19, 24, 15, 11, 7, 21, 4, 0, 20, 3, 25, 16, 12, 8, 27, 18, 23, 14, 10, 6, 2, 26, 17, 22, 13, 9, 5, 1]: LCPArray { suffixindex: [28, 19, 24, 15, 11, 7, 21, 4, 0, 20, 3, 25, 16, 12, 8, 27, 18, 23, 14, 10, 6, 2, 26, 17, 22, 13, 9, 5, 1], suffixarray: ["A", "ACATGACTGA", "ACTGA", "ACTGACATGACTGA", "ACTGACTGACATGACTGA", "ACTGACTGACTGACATGACTGA", "ATGACTGA", "ATGACTGACTGACTGACATGACTGA", "ATGCATGACTGACTGACTGACATGACTGA", "CATGACTGA", "CATGACTGACTGACTGACATGACTGA", "CTGA", "CTGACATGACTGA", "CTGACTGACATGACTGA", "CTGACTGACTGACATGACTGA", "GA", "GACATGACTGA", "GACTGA", "GACTGACATGACTGA", "GACTGACTGACATGACTGA", "GACTGACTGACTGACATGACTGA", "GCATGACTGACTGACTGACATGACTGA", "TGA", "TGACATGACTGA", "TGACTGA", "TGACTGACATGACTGA", "TGACTGACTGACATGACTGA", "TGACTGACTGACTGACATGACTGA", "TGCATGACTGACTGACTGACATGACTGA"], lcparray: [1, 0, 5, 0, 0, 0, 8, 0, 0, 9, 0, 4, 0, 0, 0, 2, 0, 6, 0, 0, 0, 0, 3, 0, 7, 0, 0, 0, 0] }}

```

 Gaurav Sablok 
