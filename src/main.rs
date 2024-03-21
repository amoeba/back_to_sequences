//! Back to sequences: find the origin of kmers

/* std use */

// TODO : limit the number of threads with an option


/* crates use */
use clap::Parser as _;

/* project use */
use back_to_sequences::back_to_sequences;
use back_to_sequences::back_to_multiple_sequences;
use back_to_sequences::cli::Args;
use back_to_sequences::kmer_counter::KmerCounterWithLog;

///////////////////////// MAIN /////////////////////////

fn main() {
    (|| {
        let args = Args::parse();


        // If out_sequences and out_kmers are not provided, we do nothing, we can quit
        if args.out_sequences == "" && args.out_kmers == "" && args.out_filelist == ""{
            return Err(eprintln!(
                "Warning: no output file provided, nothing to do"
            ));
        }

        // If out_kmers is not provided but output_kmer_positions is true, warn that it has no effect
        if args.out_kmers == "" && args.output_kmer_positions {
            eprintln!("Warning: --output_kmer_positions has no effect without --out-kmers");
        }

        // If out_kmers is not provided but counted_kmer_threshold is set, this has no effect
        if args.out_kmers == "" && args.counted_kmer_threshold > 0 {
            eprintln!("Warning: --counted-kmer-threshold has no effect without --out-kmers");
        }

        if !args.stranded && args.query_reverse {
            eprintln!("Warning: --query-reverse is useless without --stranded");
        }

        if args.min_threshold > args.max_threshold {
            return Err(eprintln!(
                "Error: --min-threshold must be <= --max-threshold"
            ));
        }

        if args.in_sequences != "" && args.in_filelist != ""{
            return Err(eprintln!(
                "Error: --in-sequences and --in-filelist are mutually exclusive"
            ));
        }

        

        if args.out_sequences != "" && args.out_filelist != ""{
            return Err(eprintln!(
                "Error: --out-sequences and --out-filelist are mutually exclusive"
            ));
        }

        if args.in_sequences == "" && args.in_filelist != ""{
            if args.out_filelist == ""{
                return Err(eprintln!(
                    "Error: --in-filelist requires --out-filelist"
                ));
            }

            if args.output_kmer_positions{
                return Err(eprintln!(
                    "Error: --in-filelist and --output-kmer-positions are mutually exclusive (for now)"
                ));
            }
            

            back_to_multiple_sequences( 
                args.in_filelist,
                args.in_kmers,
                args.out_filelist,
                args.out_kmers,
                args.kmer_size,
                args.counted_kmer_threshold,
                args.min_threshold,
                args.max_threshold,
                args.stranded,
                args.query_reverse,
                args.no_low_complexity,
            )
        }
        else{
            back_to_sequences::<std::sync::Mutex<KmerCounterWithLog>>(
                args.in_sequences,
                args.in_kmers,
                args.out_sequences,
                args.out_kmers,
                args.kmer_size,
                args.counted_kmer_threshold,
                args.min_threshold,
                args.max_threshold,
                args.stranded,
                args.query_reverse,
                args.no_low_complexity,
            )
        }
    })()
    .map_err(|()| std::process::exit(1))
    .ok();
}
