// use fasta::read::FastaReader;
use fxread::initialize_reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Write;
use glob::glob;

use clap::ArgMatches;




///////////////////////// SUBCOMMAND TO_READS /////////////////////////

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_non_empty_headers (in_tsv_dir: String, threshold: f32) -> HashMap<String, f32> {
    // declare the map
    let mut map: HashMap<String, f32> = HashMap::new();
    let regex_pattern = format!("{}/**/*.tsv", in_tsv_dir);
    // read the tsv files in the in_tsv_dir directory
    for entry in glob(regex_pattern.as_str()).expect("Failed to read glob pattern") {     
        if let Ok(mut lines) = read_lines(entry.unwrap()) {
            // Consumes the first line, must be 
            // kmers   D1
            lines.next();

            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    // a line is either as kmers:id        0
                    // or as kmers:id        value bigger than 0
                    // in the first case, we do not want to add it to the map
                    // in the second case, we want to add it to the map with its value
                    let mut iter = ip.split(":");
                    let _kmers = iter.next().unwrap(); // don't need this

                    let rest = iter.next().unwrap();
                    let mut iter2 = rest.split("\t"); // should be as "sequence2	0"
                    let sequence_id = String::from(iter2.next().unwrap());
                    let ratio_kmers: f32 = iter2.next().unwrap().parse().unwrap();
                    if ratio_kmers > threshold {
                        map.insert(sequence_id, ratio_kmers);
                    }
                }
            }
        }
    }
    map
}


fn output_reads (map: HashMap<String, f32>, in_fasta: String, out_fasta: String) -> std::io::Result<()> {
    
    // read the input fasta file
    // for each header, check if it is in the map
    // if it is, write the header and the sequence to the output fasta file
    // if it is not, do nothing
    // if the header is not in the map, do nothing
    // close the output fasta file
    // close the input fasta file
    
    let mut cnt = 0;
    let mut output = File::create(out_fasta.clone())?;
    let reader = initialize_reader(&in_fasta).unwrap();
    // for [description, seq] in FastaReader::new(infile) {
    for record in reader {
        let header = record.id_str_checked().unwrap().to_string();
        if map.contains_key(&header) {
            cnt += 1;
            let record_as_string = record.as_str_checked().unwrap().trim().as_bytes();
            let mut iter = record_as_string.split(|&x| x == b'\n');
            // for line in iter {
            //     println!("{:?}", line);
            // }
            let stringheader = iter.next().unwrap();
            // write the header in the output file with the value in the map
            output.write_all(stringheader)?;
            output.write_all(b" ")?;
            output.write_all(map.get(&header).unwrap().to_string().as_bytes())?;
            output.write_all(b"\n")?;
            for line in iter {
                output.write_all(line)?;
                output.write_all(b"\n")?;
            }
            
            
            
            // add the  value to the header
            // output.write_all(record.as_str_checked().unwrap().as_bytes())?;
            // // write the two lines in the output file
            // output.write_all(description.as_bytes())?;
            // output.write_all(b" ")?;
            // output.write_all(map.get(&header).unwrap().to_string().as_bytes())?;
            // output.write_all(b"\n")?;
            // output.write_all(seq.as_bytes())?;
            // output.write_all(b"\n")?;
        }
    }
    println!("Number of sequences in the output fasta file {} : {}", out_fasta, cnt);
    Ok(())
}

pub fn to_reads(sub_matches: &ArgMatches) {
    let inheaders = sub_matches.get_one::<String>("HEADERS").map(|s| s.clone()).unwrap();
    let infasta = sub_matches.get_one::<String>("INFASTA").map(|s| s.clone()).unwrap();
    let outfasta = sub_matches.get_one::<String>("OUTFASTA").map(|s| s.clone()).unwrap();
    let threshold= sub_matches.get_one::<f32>("THRESHOLD").map(|s| s.clone()).unwrap();
    let map = get_non_empty_headers(inheaders, threshold);
    let _ = output_reads (map, infasta, outfasta);
}