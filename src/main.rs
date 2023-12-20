/// Module      : Main
/// Description : The main entry point for the program.
/// Copyright   : (c) Andrew Lonsdale 2023
/// License     : MIT
/// Maintainer  : andrew.lonsdale@lonsbio.com.au
/// Stability   : experimental
/// Portability : POSIX
mod fastqe_map;
extern crate clap;
extern crate bio;
extern crate argparse;
extern crate flate2;

#[macro_use]
extern crate log;
use std::env;
use std::io::{self, Read};
use std::io::Write;
use std::cmp;
use bio::io::fastq;
use std::fmt;
use std::fs::File;
use argparse::{ArgumentParser, StoreTrue, Store, Print, Collect, StoreOption};
use log::LevelFilter;
use clap::{Arg,Command,ArgMatches};
use flate2::read::GzDecoder;

// File I/O error. This can occur if at least one of the input FASTA
// files cannot be opened for reading. This can occur because the file
// does not exist at the specified path, or fastqe_rust does not have
// permission to read from the file.
const EXIT_FILE_IO_ERROR: i32 = 1;

// A command line error occurred. This can happen if the user specifies
// an incorrect command line argument. In this circumstance fastqe_rust will
// also print a usage message to the standard error device (stderr).
const EXIT_COMMAND_LINE_ERROR: i32 = 2;

// Input FASTA file is invalid. This can occur if fastqe_rust can read an
// input file but the file format is invalid.
const EXIT_FASTA_PARSE_ERROR: i32 = 3;

// Name of the program, to be used in diagnostic messages.
static PROGRAM_NAME: &'static str = "fastqe";

/// Exit the program, printing an error message on stderr, and returning
/// a specific error code. The program name is prefixed onto the front of
/// the error message. If logging is enabled we also write the error
/// message to the log file.
fn exit_with_error(status: i32, message: &String) -> () {
    error!(target: "log_messages", "{}", message);
    writeln!(&mut std::io::stderr(),
             "{} ERROR: {}!",
             PROGRAM_NAME,
             message)
        .unwrap();
    std::process::exit(status);
}

/// Basic statistics computed for a FASTA file.
/// Note that all values are with respect to only those reads whose
/// length is at least as long as the minimum.
#[derive(Debug, PartialEq)]
pub struct FastaStats {
    /// Minimum length of all sequences in the file.
    min_len: u64,
    /// Average length of all sequences in the file rounded towards zero.
    average_len: u64,
    /// Maximum length of all sequences in the file.
    max_len: u64,
    /// Total number of bases from all the sequences in the file.
    total: u64,
    /// Total number of sequences in the file.
    num_seqs: u64,
    /// Initial read buffer.
    longread: u64,
    /// Means results
    means: CountVec,
    /// Fastq record with mean scores
    meansRecord: fastq::Record
}



/// Construct a FastaStats value from an input FASTA file. Sequences
/// of length < `minlen` are ignored.
/// The input FASTA file is represented as an `io::Read` reader.
/// The result is of type `Result<Option<FastaStats>, io::Error>`
///  * `Err(error)` if an error occurred while reading the FASTA file.
///  * `Ok(Nothing)` if the FASTA file had no sequences of length >= `minlen`.
///  * `Ok(Some(stats))` if there were no errors reading the file and there was
///     at least one sequence in the file whose length was >= `minlen`.
impl FastaStats {
    pub fn new<R: io::Read>(minlen: u64, longread: u64, reader: R) -> Result<Option<FastaStats>, io::Error> {
        let fasta_reader = fastq::Reader::new(reader);
        let mut num_seqs: u64 = 0;
        let mut total: u64 = 0;
        let mut max_len: u64 = 0;
        let mut min_len: u64 = 0;
        let mut this_len: u64;
        let mut means = vec![0.0; longread as usize];
        let mut counts = vec![0.0; longread as usize];

        // Read each sequence in the input FASTA file.
        for next in fasta_reader.records() {
            match next {
                // The sequence was parsed correctly.
                Ok(record) => {
                    // Filter out sequences whose length is too short.
                    this_len = record.seq().len() as u64;
                    if this_len >= minlen {
                        num_seqs += 1;
                        total += this_len;
                        if num_seqs == 1 {
                            // This is the first sequence we have
                            // encountered, set max and min to the
                            // length of this sequence.
                            max_len = this_len;
                            min_len = this_len;
                        } else {
                            // Update max and min.
                            max_len = cmp::max(max_len, this_len);
                            min_len = cmp::min(min_len, this_len);
                        }
                    }


                    // after processing 
                    
                    //means = record.qual();
                    let mut vec_index = 0;
                    // scores come out here as u8 or 64 or -33?
                    for r in record.qual()
                    {
                        // let q = r;
                        // means.push(*q as f64);
                        means[vec_index] += *r as f64;
                        counts[vec_index] += 1.0;
                        vec_index += 1;

                    }



                }
                // There was an error parsing the sequence.
                // TODO fix this to catch errors
                //Err(error) => return fastq::Error::ReadError(error),
                 _ => ()
}
        }
        if num_seqs > 0 {


            trim_zeros(&mut means);
            trim_zeros(&mut counts);


            let result: Vec<f64> = means.iter().zip(counts.iter()).map(|(&a, &b)| a / b).collect();
            means = result.into_iter().map(|x| x.ceil()).collect();
            let means_as_q = means.iter().map(|&x| x as u8).collect::<Vec<u8>>();

            // record_mean = SeqRecord(Seq(fake_seq), id="test", name="mean scores",
            // description="example with mean fastq socres",
            // letter_annotations={'phred_quality':list(means_fp.round().astype(int))})

            //println!("{}", "A".repeat(means.len()));
            let fake_record = fastq::Record::with_attrs("id_str", Some("mean"), "A".repeat(means.len()).as_bytes(), &means_as_q.as_slice());
            //println!("{}",fake_record);

            // let mut sum_qual = record.qual().iter().sum::<u8>() as f64;
            
            //      if (sum_qual / record.seq().len() as f64 - 33.0) > 30.0 {
            //         writer.write_record(&record);
            //      }

            // We encountered at least one sequence, so we can
            // compute statistics for this file.
            let average_len = ((total as f64) / (num_seqs as f64)).floor() as u64;
            Ok(Some(FastaStats {
                min_len: min_len,
                average_len: average_len,
                max_len: max_len,
                total: total,
                num_seqs: num_seqs,
                longread: longread,
                means: CountVec(means),
                meansRecord: fake_record
                
            }))
        } else {
            // We did not encounter any sequences (satisfying the length
            // requirement). So we cannot compute statistics for this file.
            Ok(None)
        }
    }
}

/// Format the FastaStats for output.
/// Use a tab as a delimiter.
impl fmt::Display for FastaStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        let chars: Vec<String> = self.meansRecord.qual().iter().map(|&n|n).collect();
  let chars: Vec<_> = self.meansRecord.qual().iter().map(|&n|n as char).collect();

        //let chars = self.meansRecord.qual();
       // let chars_codes: Vec<_> = chars.iter().map(|n| fastqe_map::get_fastq_emoji_map().get(n)).collect();
        
        //let chars_codes: Vec<_> = chars.iter().map(|n| fastqe_map::get_fastq_emoji_map().get(n)).collect();
        

            // for i in  self.meansRecord.qual().iter().map(|&n| fastqe_map::get_fastq_emoji_map().get(&(n as char)).expect("REASON").to_string()).collect::<Vec<_>>() {
            //     let mut v = Vec::new();
            //     write!(f,"{:?}",replace(&i as &str, &mut v).unwrap());
            //     //assert_eq!(std::str::from_utf8(&v).unwrap(), o);
            // };
            //self.meansRecord.qual().iter().map(|&n| emojireplace( fastqe_map::get_fastq_emoji_map().get(&(n as char)).expect("REASON"))).collect::<Vec<_>>();

            write!(f,
                //"{}\t{}\t{:?}\t{}\t{}",
                //  "{}\t{:?}\t{:?}",
 
                "{}\t{}",
                // "{}\t{:?}",
                //self.num_seqs,
                //self.means,
                self.meansRecord.desc().unwrap(),
               // self.meansRecord.qual().iter().map(|&n| fastqe_map::get_fastq_emoji_map().get(&(n as char)).expect("REASON").to_string()).collect::<Vec<_>>(),
               self.meansRecord.qual().iter().map(|&n| emojireplace( fastqe_map::get_fastq_emoji_map().get(&(n as char)).expect("REASON")).unwrap().as_str()).collect::<Vec<_>>().join(" "),
                //self.average_len,
                //self.max_len
             )
            

            

            


    }
}

// let mut v = Vec::new();
// replace(i, &mut v).unwrap();

#[derive(Debug, PartialEq, Clone)]
struct CountVec(Vec<f64>);

impl CountVec {

fn pop(&mut self) -> Option<f64> {
    self.0.pop()
}
}

impl fmt::Display for CountVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, value) in self.clone().into_iter().enumerate() {
            if i != 0 { write!(f, ", ")?; }
            write!(f, "{}", value)?;
        }
        Ok(())
    }
}


impl IntoIterator for CountVec {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// Trim values from end where 0
fn trim_zeros(vec: &mut Vec<f64>) {
    while let Some(0.0) = vec.clone().into_iter().last() {
        vec.pop();
    }
}



// based on
// github: https://github.com/rossmacarthur/emojis/blob/trunk/examples/replace.rs 
// only works for short codes of emoji, or pass through
fn emojireplace(s: &str) -> Option<&'static emojis::Emoji> {
//fn emojireplace(s: &str) -> String {
   
    while let Some((i, m, n, j)) = s
        .find(':')
        .map(|i| (i, i + 1))
        .and_then(|(i, m)| s[m..].find(':').map(|x| (i, m, m + x, m + x + 1)))
    {
        // match emojis::get_by_shortcode(&s[m..n]) {
        //      Some(emoji) => {
        //          // Output the emoji.
        //          emoji //.as_str().to_string()
        //      }
        //      None =>  {emojis::get("🚀").unwrap().as_str().to_string()}
        //  };
        return emojis::get_by_shortcode(&s[m..n])
    }

    //  //std::string::String::from("")
    //  emojis::get("🚀").unwrap().as_str().to_string()

    // no shortcode (:...:) found so check if it is an emoji, otherwise use a deafault
    let fallthrough  = match emojis::get(s) {
        Some(emoji) => Some(emoji),
        
        None => emojis::get("❓")
    };

    fallthrough
    //emojis::get("❓")
}



// github: https://github.com/rossmacarthur/emojis/blob/trunk/examples/replace.rs 
fn replace(mut s: &str, mut o: impl Write) -> io::Result<()> {
    // The meaning of the index values is as follows.
    //
    //  : r o c k e t :
    // ^ ^           ^ ^
    // i m           n j
    //
    // i..j gives ":rocket:"
    // m..n gives "rocket"
    while let Some((i, m, n, j)) = s
        .find(':')
        .map(|i| (i, i + 1))
        .and_then(|(i, m)| s[m..].find(':').map(|x| (i, m, m + x, m + x + 1)))
    {
        match emojis::get_by_shortcode(&s[m..n]) {
            Some(emoji) => {
                // Output everything preceding, except the first colon.
                o.write_all(s[..i].as_bytes())?;
                // Output the emoji.
                o.write_all(emoji.as_bytes())?;
                // Update the string to past the last colon.
                s = &s[j..];
            }
            None => {
                // Output everything preceding but not including the colon.
                o.write_all(s[..n].as_bytes())?;
                // Update the string to start with the last colon.
                s = &s[n..];
            }
        }
    }
    o.write_all(s.as_bytes())
}

//copilot mimicing Python version
fn open_possible_gz_file(filename: &String) -> io::Result<Box<dyn Read>> {
    let file = File::open(filename)?;
    if filename.ends_with(".gz") {
        Ok(Box::new(GzDecoder::new(file)))
    } else {
        Ok(Box::new(file))
    }
}

/// Compute basic statistics for the input FASTA files,
/// and pretty print the results to standard output.
/// Program exits if a parse error occurs when reading an
/// input FASTA file.
fn compute_print_stats<R: io::Read>(options: &ArgMatches, filename: &String, reader: R) -> () {
    match FastaStats::new(*options.get_one::<u64>("minlen").unwrap(),*options.get_one::<u64>("long").unwrap(), reader) {
        Ok(Some(stats)) => {
            // Prefix the FASTA filename onto the front of the statistics
            println!("{}\t{}", filename, stats);
        }
        Ok(None) => {
            // We could not compute any statistics for the file because
            // it contained no sequences at least as long as the minimum
            // length. In this case we just print dashes.
            println!("{}\t0\t0\t-\t-\t-", filename);
        }
        // There was a parse error when reading a FASTA file.
        // Exit the program.
        Err(error) => exit_with_error(EXIT_FASTA_PARSE_ERROR, &format!("{}", error)),
    }
}

/// Initialise the logging infrastructure. If the command line option --log
/// is specified, we set up logging to write to the filename supplied as
/// an argument. If it is not supplied then logging will not occur. Log
/// messages are tagged with their date and time of occurrence.
/// If logging is initialised, then we write a message to indicate that
/// the program started, and then we log the command line arguments.
fn init_logging(options: &ArgMatches) -> () {
    // Check if --log was specified as a command line argument
    match options.get_one::<String>("log") {
        // --log was specified, set logging output to the supplied filename
        Some(ref filename) => ()
        ,
        // --log was not specified, do nothing. Log messages will not be written.
        None => ()
    }
}

/// The entry point for the program.
///  * Parse the command line arguments.
///  * Print the output header.
///  * Process each input FASTA file, compute stats and display
///    results.
fn main() {

 let options = Command::new("fastqe")
        .version("1.0")
        .about("Read one or more FASTQ files, compute quality stats for each file, print as emoji... for some reason.😄 \n\n🚨 Rust and WebAssembly beta: only command line options with a  ✅  are functional  🚨 \n rustc 1.75.0-beta.7 emcc 3.1.50")
        .arg(Arg::new("FASTQ_FILE")
            .help("Input FASTQ files")
            .num_args(1..))
        .arg(Arg::new("bin")
            .long("bin")
            .num_args(0)
            .help_heading("Emoji options")
            .help("Use binned scores (🚫 💀 💩 🚨 😄 😆 😎 😍 ) ❌ "))
        .arg(Arg::new("minlen")
            .long("minlen")
            .value_name("N")
            .default_value("0")
	    .value_parser(clap::value_parser!(u64))
            .help_heading("Statistics options")
            .help("Minimum length sequence to include in stats ✅")
            .num_args(1))
        .arg(Arg::new("scale")
            .long("scale")
            .num_args(0)
            .help_heading("Statistics options")
            .help("Show relevant scale in output ❌"))
        .arg(Arg::new("nomean")
            .long("nomean")
            .num_args(0)
            .help_heading("Statistics options")
            .help("Hide mean quality per position ❌"))
        .arg(Arg::new("min")
            .long("min")
            .help_heading("Statistics options")
            .num_args(0)
            .help("Show minimum quality per position  ❌"))
        .arg(Arg::new("max")
            .long("max")
            .help_heading("Statistics options")
            .num_args(0)
            .help("Show maximum quality per position ❌"))
        .arg(Arg::new("custom")
            .long("custom")
            .value_name("CUSTOM_DICT")
            .help_heading("Emoji options")
            .help("Use a mapping of custom emoji to quality in CUSTOM_DICT (🐍🌴) ❌")
            .num_args(1))
        .arg(Arg::new("noemoji")
            .long("noemoji")
            .help_heading("Emoji options")
            .num_args(0)
            .help("Use mapping without emoji (▁▂▃▄▅▆▇█) ❌"))
        .arg(Arg::new("noheader")
            .long("noheader")
            .num_args(0)
            .help("Hide the header before sample output ❌"))
        .arg(Arg::new("html")
            .long("html")
            .num_args(0)
            .help_heading("HTML report options")
            .help("Output all data as html ❌"))
        .arg(Arg::new("window")
            .long("window")
            .value_name("W")
            .default_value("1")
            .help_heading("HTML report options")
            .help("Window length to summarise reads in HTML report ❌")
            .num_args(1))
        .arg(Arg::new("html_escape")
            .long("html_escape")
            .num_args(0)
            .help_heading("HTML report options")
            .help("❌ Escape html within output, e.g. for Galaxy parsing"))
        .arg(Arg::new("output")
            .long("output")
            .value_name("OUTPUT_FILE")
            .help("Write output to OUTPUT_FILE instead of stdout")
            .num_args(1))
        .arg(Arg::new("long")
            .long("long")
            .value_name("READ_BUFFER")
            .default_value("500")
            .value_parser(clap::value_parser!(u64))
            .help("Buffer memory for long reads up to READ_BUFFER bp long ❌")
            .num_args(1))
        .arg(Arg::new("log")
            .long("log")
	    .value_parser(clap::value_parser!(String))
            .value_name("LOG_FILE")
            .help("Record program progress in LOG_FILE ✅")
            .num_args(1))
        // Add the rest of your arguments here
        .after_help("For more information, vist https://fastqe.com")
        .get_matches();

    //let options = parse_options();

    // Optionally initialise the logging system.
    init_logging(&options);

    // Display the output header.
    //println!("FILENAME\tNUMSEQ\tTOTAL\tMIN\tAVG\tMAX");
    println!("Filename\tStatistic\tQuality");


    let files = match options.get_many("FASTQ_FILE") {
    Some(files) => {
        // Process each FASTA file specified on the command line.
        // Exit the program if a file I/O error occurs.
        for filename in files {
            match open_possible_gz_file(filename) {
                Ok(file) => {
                    info!(target: "log_messages", "Processing FASTA file from {}", filename);
                    compute_print_stats(&options, filename, file);
                }
                Err(error) => exit_with_error(EXIT_FILE_IO_ERROR,
                    &format!("Failed to open '{}'. {}", filename, error)),
            }
        }


}
    None => {
//        // read from stdin instead.
        info!(target: "log_messages", "Processing FASTA file from stdin");
        compute_print_stats(&options, &String::from("stdin"), io::stdin())

}

};

}

/// Unit testing.
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function for running test cases.
    /// Arguments are:
    ///  * `minlen`: minimum length of sequences in the FASTA file to be
    ///     considered by the program.
    ///  * `input`: a string containing the contents of the input FASTA file.
    ///  * `expected`: the expected result of computing statistics for the input
    ///     FASTA file. `None` means that we do not expect any statistics to be
    ///     computed because there are no sequences in the input which meet the
    ///     minimum length requirements. `Some(stats)` means that we expect some
    ///     particular statistics to be computed.
    /// We compute the `FastaStats` for the input FASTA file and then compare against
    /// the expected result. If they are different we `panic` with a error message.
    fn test_fastastats_ok(minlen: u64,longread: u64, input: &String, expected: Option<FastaStats>) -> () {
        match FastaStats::new(minlen,longread, input.as_bytes()) {
            Ok(result) => {
                if result != expected {
                    panic!(format!("expected {:?} got {:?}", expected, result))
                }
            }
            Err(_error) => panic!(format!("expected {:?} got Err(..)", expected)),
        }
    }

    /// Test for computations which are expected to fail.
    /// NB io::Error does not currently implement PartialEq, so we resort to
    /// just checking for the existence of an error, not its actual content.
    /// Hopefully this will be fixed in Rust soon:
    /// https://github.com/rust-lang/rust/issues/34158
    fn test_fastastats_err(minlen: u64, longread: u64, input: &String) -> () {
        let comp = FastaStats::new(minlen, longread, input.as_bytes());
        match comp {
            Ok(result) => panic!(format!("expected Err(..) got {:?}", result)),
            Err(_error) => (),
        }
    }

    /// Empty input FASTA file. Should not produce any statistics.
    #[test]
    fn zero_byte_input() {
        test_fastastats_ok(0, &String::from(""), None)
    }

    /// Input FASTA file consisting of just a newline character.
    /// Result is expected to be an error, as the input is not considered
    /// a valid FASTA file.
    #[test]
    fn single_newline_input() {
        test_fastastats_err(0, &String::from("\n"))
    }

    /// Input FASTA file consisting of a single greater than sign,
    /// which is the minimal requirement for the FASTA header.
    #[test]
    fn single_greater_than_input() {
        test_fastastats_ok(0,
                           &String::from(">"),
                           None)
    }

    /// Input FASTA file consisting of a single sequence.
    /// The sequence is split over two lines.
    #[test]
    fn one_sequence() {
        test_fastastats_ok(0,
                           &String::from(">header\nATGC\nA"),
                           Some(FastaStats {
                               min_len: 5,
                               average_len: 5,
                               max_len: 5,
                               total: 5,
                               num_seqs: 1,
                           }))
    }

    /// Input FASTA file consisting of two sequences.
    /// The sequences are split over multiple lines.
    #[test]
    fn two_sequence() {
        test_fastastats_ok(0,
                           &String::from(">header1\nATGC\nAGG\n>header2\nTT\n"),
                           Some(FastaStats {
                               min_len: 2,
                               average_len: 4,
                               max_len: 7,
                               total: 9,
                               num_seqs: 2,
                           }))
    }

    /// Input FASTA file without a sequence header. This is considered an
    /// error, because it is not a valid FASTA file.
    #[test]
    fn no_header() {
        test_fastastats_err(0, &String::from("no header\n"))
    }

    /// Input FASTA file with 2 sequences, and minlen less than the lengths
    /// of all the sequences in the file. None of the sequences should be
    /// filtered out.
    #[test]
    fn minlen_less_than_all() {
        test_fastastats_ok(2,
                           &String::from(">header1\nATGC\nAGG\n>header2\nTT\n"),
                           Some(FastaStats {
                               min_len: 2,
                               average_len: 4,
                               max_len: 7,
                               total: 9,
                               num_seqs: 2,
                           }))
    }

    /// Input FASTA file with 2 sequences, and minlen greater than
    /// the length of one of the sequences. This sequence should be
    /// filtered out, and not considered in the calculation of the
    /// statistics.
    #[test]
    fn minlen_greater_than_one() {
        test_fastastats_ok(3,
                           &String::from(">header1\nATGC\nAGG\n>header2\nTT\n"),
                           Some(FastaStats {
                               min_len: 7,
                               average_len: 7,
                               max_len: 7,
                               total: 7,
                               num_seqs: 1,
                           }))
    }

    /// Input FASTA file with 2 sequences, and minlen greater than
    /// the length of all the sequences. All sequences should be filtered
    /// out, and thus there are no statistics to compute.
    #[test]
    fn minlen_greater_than_all() {
        test_fastastats_ok(8,
                           &String::from(">header1\nATGC\nAGG\n>header2\nTT\n"),
                           None)
    }
}
