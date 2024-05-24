//! Define Command Line Interface

/* std use */

/* crates use */
use clap::Parser;

/* project use */

/// Extract sequences that contain some kmers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input fasta or fastq [.gz] file containing the original sequences (eg. reads). THe stdin is used if not provided
    #[arg(long, default_value_t = String::from(""))]
    pub in_sequences: String,

    /// Input fasta file containing the original kmers
    #[arg(long)]
    pub in_kmers: String,

    /// Output file containing the filtered original sequences (eg. reads).
    /// It will be automatically in fasta or fastq format depending on the input file.
    /// If not provided, only the in_kmers with their count is output
    #[arg(long, default_value_t = String::from(""))]
    pub out_sequences: String,

    /// If provided, output text file containing the kmers that occur in the reads with their number of occurrences
    #[arg(long, default_value_t = String::from(""))]
    pub out_kmers: String,

    /// Size of the kmers to index and search
    #[arg(short, long, default_value_t = 31)]
    pub kmer_size: usize,

    /// Output sequences are those whose ratio of indexed kmers is in ]min_threshold; max_threshold]
    /// Minimal threshold of the ratio  (%) of kmers that must be found in a sequence to keep it (default 0%).
    /// Thus by default, if no kmer is found in a sequence, it is not output.
    #[arg(short, long, default_value_t = 0.0)]
    pub min_threshold: f32,

    /// Output sequences are those whose ratio of indexed kmers is in ]min_threshold; max_threshold]
    /// Maximal threshold of the ratio (%) of kmers that must be found in a sequence to keep it (default 100%).
    /// Thus by default, there is no limitation on the maximal number of kmers found in a sequence.
    #[arg(long, default_value_t = 100.0)]
    pub max_threshold: f32,

    /// Used original kmer strand (else canonical kmers are considered)
    #[arg(long, default_value_t = false)]
    pub stranded: bool,

    /// Query the reverse complement of reads. Useless without the --stranded option
    #[arg(long, default_value_t = false)]
    pub query_reverse: bool,

    /// Do not index low complexity kmers (ie. with a Shannon entropy < 1.0)
    #[arg(long, default_value_t = false)]
    pub no_low_complexity: bool,
}

/// check that a file name corresponds to a non empty file:
pub fn validate_non_empty_file(in_file: String) -> anyhow::Result<()> {
    if let Ok(metadata) = std::fs::metadata(in_file.clone()) {
        // Check if the file exists
        if !metadata.is_file() {
            anyhow::bail!("{:#} exists, but it's not a file.", in_file)
        }
    } else {
        anyhow::bail!("{:#} exists, but it's not a file.", in_file)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty_file_test() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let directory = temp_dir.into_path();
        let file = directory.join("empty.fasta");

        // test directory
        assert!(
            validate_non_empty_file(directory.clone().into_os_string().into_string().unwrap())
                .is_err()
        );

        // test not exist
        assert!(
            validate_non_empty_file(file.clone().into_os_string().into_string().unwrap()).is_err()
        );

        // test work
        std::fs::File::create(&file)?;
        assert!(
            validate_non_empty_file(file.clone().into_os_string().into_string().unwrap()).is_ok()
        );

        Ok(())
    }
}
