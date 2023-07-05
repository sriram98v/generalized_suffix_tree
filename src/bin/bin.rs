extern crate clap;
use clap::{arg, Command};
use std::collections::{HashMap, HashSet};
use bio::io::{fasta};
use generalized_suffix_tree::suffix_tree::KGST;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs};
use std::io::Write;
use error_chain::error_chain;
use generalized_suffix_tree;
use serde::{Serialize, Deserialize};
use std::{fmt};


#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum SeqElement {
    A, G, T, C, E
}

impl fmt::Display for SeqElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SeqElement::A => write!(f, "A"),
            SeqElement::G => write!(f, "G"),
            SeqElement::T => write!(f, "T"),
            SeqElement::C => write!(f, "C"),
            SeqElement::E => write!(f, "$"),
        }
    }
}

fn build_tree(file:&str, num_seq: u32)->KGST<SeqElement, String>{
    println!("Building tree from {}", file);
    let reader = fasta::Reader::from_file(file).unwrap();

    let total_size = reader.records().count();

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    
    let mut tree: KGST<SeqElement, String> = KGST::new(SeqElement::E);

    let reader = fasta::Reader::from_file(file).unwrap();
    let mut strings:HashMap<String, Vec<SeqElement>> = HashMap::new();

    let mut count = 0;
    
    for result in reader.records() {

        let result_data = result.unwrap();

        let x: Vec<char> = result_data.seq()
        .to_vec()
        .iter()
        .map(|x| *x as char)
        .collect();
        
        let seq: Vec<SeqElement> = x.iter()
            .map(|x|{
                match x{
                    'A' => SeqElement::A,
                    'G' => SeqElement::G,
                    'T' => SeqElement::T,
                    'C' => SeqElement::C,
                    _ => SeqElement::E,
                }
            })
            .collect();
    
        tree.add_string(seq.to_vec(), result_data.id().to_string());

        strings.insert(result_data.id().to_string(), seq);
        pb.inc(1);   
        count+=1;
        if count==num_seq {
            break;
        }
    }
    tree
}

fn main(){
    let matches = Command::new("Generalized suffix tree")
        .version("1.0")
        .author("Sriram Vijendran <vijendran.sriram@gmail.com>")
        .subcommand(Command::new("build")
            .about("Build suffix tree index from reference fasta file")
            .arg(arg!(-s --source <SRC_FILE> "Source file with sequences(fasta)")
                .required(true)
                )
            .arg(arg!(-o --out <SAVE_FILE> "save file")
                .required(true)
                )
            .arg(arg!(-n --num <NUM_SEQ> "Number of seq. (0==all)")
                .required(true)
                .value_parser(clap::value_parser!(u32))
                )
        )
        .about("Metagenomic classifier using Suffix trees")
        .get_matches();

        match matches.subcommand(){
            Some(("build",  sub_m)) => {
                let mut tree: KGST<SeqElement, String> = build_tree(sub_m.get_one::<String>("source").expect("required").as_str(), *sub_m.get_one::<u32>("num").expect("required"));
            },
            _ => {
                println!("Either build a tree or query an existing tree. Refer help page (-h flag)");
            }
        }
}