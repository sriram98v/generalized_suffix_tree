extern crate clap;
use clap::{arg, Command};
use std::collections::HashMap;
use bio::io::fasta;
use generalized_suffix_tree::suffix_tree::KGST;
use indicatif::{ProgressBar, ProgressStyle};
use generalized_suffix_tree;

fn build_tree(file:&str, num_seq: &usize, max_depth: &usize)->KGST<char, String>{
    println!("Building tree from {}", file);
    let reader = fasta::Reader::from_file(file).unwrap();

    let total_size = reader.records().count();

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    
    let mut tree: KGST<char, String> = KGST::new('$');

    let reader = fasta::Reader::from_file(file).unwrap();
    let mut strings:HashMap<String, Vec<char>> = HashMap::new();

    let mut count = 0;
    
    for result in reader.records() {

        let result_data = result.unwrap();

        let seq: Vec<char> = result_data.seq()
        .to_vec()
        .iter()
        .map(|x| *x as char)
        .collect();
    
        tree.add_string(seq.to_vec(), result_data.id().to_string(), max_depth);

        strings.insert(result_data.id().to_string(), seq);
        pb.inc(1);   
        count+=1;
        if &count==num_seq {
            break;
        }
    }
    tree
}

fn save_tree(tree: &mut KGST<char, String>, output_path: String){
    println!("Saving tree to {}.", &output_path);
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(tree).unwrap(),
    ).unwrap();
    println!("Saved");
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
            .arg(arg!(-d --depth <DEPTH> "Max tree depth. (0==full_depth)")
                .required(true)
                )
            .arg(arg!(-n --num <NUM_SEQ> "Number of seq. (0==all)")
                .required(true)
                .value_parser(clap::value_parser!(u32))
                )
        )
        .about("CLI tool to build K-Truncated generalized suffix trees")
        .get_matches();

        match matches.subcommand(){
            Some(("build",  sub_m)) => {
                let mut tree: KGST<char, String> = build_tree(sub_m.get_one::<String>("source").expect("required").as_str(), sub_m.get_one::<usize>("num").expect("required"), sub_m.get_one::<usize>("depth").expect("required"));
                save_tree(&mut tree, sub_m.get_one::<String>("out").expect("required").to_string());
            },
            _ => {
                println!("Either build a tree or query an existing tree. Refer help page (-h flag)");
            }
        }
}