mod args;
use args::LCPArraysArgs;
use clap::Parser;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-12-19

rust-suffix-arrays: rust lcp arrays from the suffix arrays.
* */

fn main() {
    let args = LCPArraysArgs::parse();
    let lcparray_output = lcp_array(&args.lcparray_arg).unwrap();
    println!("The lcp arrays has been written: {}", lcparray_output);
}

fn lcp_array(path: &str) -> Result<String, Box<dyn Error>> {
    let file_appear = File::open(path).expect("file not present");
    let fileread_appear = BufReader::new(file_appear);
    let mut filecontent: Vec<String> = Vec::new();
    let mut filecontent_seq: Vec<String> = Vec::new();
    for i in fileread_appear.lines() {
        let line = i.expect("line not present");
        if line.starts_with("@") {
            filecontent.push(line)
        } else if line.starts_with("A")
            || line.starts_with("T")
            || line.starts_with("G")
            || line.starts_with("C")
        {
            filecontent_seq.push(line)
        }
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]

    struct SuffixMap {
        indices: Vec<usize>,
        array: Vec<String>,
    }
    let mut final_suffix_array: BTreeMap<String, SuffixMap> = BTreeMap::new();
    let mut final_suffix_sorted_btreemap: BTreeMap<String, Vec<(usize, String)>> = BTreeMap::new();
    for i in 0..filecontent_seq.len() {
        let seqhold: Vec<_> = filecontent_seq[i]
            .chars()
            .map(|x| String::from(x))
            .collect::<Vec<_>>();
        let mut heap_indices: Vec<usize> = Vec::new();
        let mut heap_string: Vec<String> = Vec::new();
        for i in 0..seqhold.len() {
            heap_indices.push(i);
            heap_string.push(seqhold[i..seqhold.len()].join("").to_string());
        }
        let mut suffix_hold: Vec<(usize, String)> = Vec::new();
        for (indices, str) in heap_indices.iter().zip(heap_string.iter()) {
            suffix_hold.push((*indices, str.to_string()));
        }
        suffix_hold.sort_by(|a, b| a.1.cmp(&b.1));

        final_suffix_sorted_btreemap.insert(filecontent_seq[i].clone(), suffix_hold);
        final_suffix_array.insert(
            seqhold.join(""),
            SuffixMap {
                indices: heap_indices,
                array: heap_string,
            },
        );
    }

    // decompressing the suffix array into a BtreeMap so that any suffix array can be just search
    // using the value of the BTreeMap and dont have to iterate over the two separate vectors
    // like in C++.

    let mut lcp_coordinate: BTreeMap<Vec<usize>, Vec<String>> = BTreeMap::new();

    for (_, vecvalue) in final_suffix_sorted_btreemap.iter() {
        let mut valuevec: Vec<(usize, String)> = vecvalue.to_vec();
        let mut suffix_to_lcp_strings: Vec<String> = Vec::new();
        let mut suffix_to_lcp_indices: Vec<usize> = Vec::new();
        for i in valuevec.iter_mut() {
            suffix_to_lcp_indices.push(i.0);
            suffix_to_lcp_strings.push(i.1.clone());
        }
        lcp_coordinate.insert(suffix_to_lcp_indices, suffix_to_lcp_strings);
    }

    // writing the BTreeMap for the LCP array but instead of using a inverse suffix array as said
    // in Kasai algorithm, i implemented a different hash string compare algorithm.
    // So it compares the length of the ith suffix to the i+1 suffix and use the length of the
    // ith suffix.
    //
    // In this way, i am only limiting to compare the two strings at a time as compared to the entire vector
    // of the strings. I futher put a restriction that if the next ith suffix in the tree is not
    // have the same length or smaller than the i-1 then just insert a 0usize.
    // reading a compiler design using a cannonical L-R automation and wrote this apart from
    // writing a canonical LR automation.

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct LCPArray {
        suffixindex: Vec<usize>,
        suffixarray: Vec<String>,
        lcparray: Vec<usize>,
    }

    let mut final_lcp_array: BTreeMap<Vec<usize>, LCPArray> = BTreeMap::new();
    for (array_indices, array_string) in lcp_coordinate.iter() {
        let suffix_array_strings: Vec<String> = array_string.to_vec();
        let mut suffix_lcp_array: Vec<usize> = Vec::new();
        for i in 0..=suffix_array_strings.len() - 2 {
            let start_lcp = suffix_array_strings[i].clone();
            let next_lcp = suffix_array_strings[i + 1].clone();
            if start_lcp.len() <= next_lcp.len() && next_lcp[0..start_lcp.len()] == start_lcp {
                suffix_lcp_array.push(next_lcp[0..start_lcp.len()].len());
            } else if start_lcp.len() >= next_lcp.len() && start_lcp[0..next_lcp.len()] != start_lcp
                || start_lcp.len() <= next_lcp.len() && next_lcp[0..start_lcp.len()] != start_lcp
            {
                suffix_lcp_array.push(0usize);
            }
        }
        suffix_lcp_array.push(0usize);
        final_lcp_array.insert(
            array_indices.to_vec(),
            LCPArray {
                suffixindex: array_indices.to_vec(),
                suffixarray: array_string.to_vec(),
                lcparray: suffix_lcp_array,
            },
        );
    }
    println!("{:?}", final_lcp_array);

    let mut final_lcp_array_write = File::create("lcp-array.txt").expect("file not present");
    for (final_write, final_array) in final_lcp_array.iter() {
        writeln!(
            final_lcp_array_write,
            "{:?}\t{:?}\t{:?}\t{:?}\n",
            final_write, final_array.suffixindex, final_array.suffixarray, final_array.lcparray
        )
        .expect("lcp array not present");
    }

    Ok("lcp_arrays have been written".to_string())
}
