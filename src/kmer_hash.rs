//! Kmer hash declarations

/* std use */

/* crates use */
use ahash::AHashMap as HashMap;
use entropy::shannon_entropy;
use fxread::initialize_reader;

/* project use */
use crate::sequence_normalizer::SequenceNormalizer;


/// given a kmer as a &[u8] check that it contains only ACGT letters
/// return true if it is the case, false otherwise
fn is_acgt(kmer: &[u8]) -> bool {
    for &byte in kmer {
        if byte != b'A' && byte != b'C' && byte != b'G' && byte != b'T' {
            return false;
        }
    }
    true
}



/// index all kmers of size kmer_size in the fasta file
/// returns a hashmap with the kmers as keys and their count as values, initialized to 0
pub fn index_kmers<T: Default>(
    file_name: String,
    kmer_size: usize,
    stranded: bool,
) -> anyhow::Result<(HashMap<Vec<u8>, T>, usize)> {
    let mut kmer_set = HashMap::new();
    let reverse_complement = if stranded { Some(false) } else { None };

    let mut reader = initialize_reader(&file_name)?;
    loop {
        let Some(mut record) = reader.next_record()? else {
            break;
        };
        record.upper();
        let acgt_sequence = record.seq();

        
        // for each kmer of the sequence, insert it in the kmer_set
        if acgt_sequence.len() < kmer_size {
            continue;
        }
        for i in 0..(acgt_sequence.len() - kmer_size + 1) {
            let kmer = &acgt_sequence[i..(i + kmer_size)];
            if is_acgt(kmer) { //TODO if not: get the position of the last non acgt letter and jump to the next potential possible kmer
                // If the entropy is too low, the kmer is not inserted
                if shannon_entropy(kmer) < 1.0 { // TODO: make this an option. 
                    continue;
                }
                kmer_set.insert(
                    SequenceNormalizer::new(kmer, reverse_complement)
                        .iter()
                        .collect(),
                    Default::default(), // RelaxedCounter::new(0)
                );
            }
        }
    }
    println!(
        "Indexed {} kmers, each of size {}",
        kmer_set.len(),
        kmer_size
    );

    Ok((kmer_set, kmer_size))
}
