use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct LCPArraysArgs {
    /// please provide the path to the fastq file
    pub lcparray_arg: String
}
