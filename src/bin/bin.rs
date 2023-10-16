extern crate clap;

use clap::{arg, Command};
use bio::io::fasta;
use generalized_suffix_tree::data::tree_item::TreeItem;
use generalized_suffix_tree::suffix_tree::KGST;
use generalized_suffix_tree::suffix_tree::tree::SuffixTree;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;

fn build_tree(file:&str, num_seq: &usize, max_depth: &usize)->KGST<char, String>{
    println!("Building tree from {}", file);
    let reader = fasta::Reader::from_file(file).expect("File node found!");

    let total_size = reader.records().count();

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    
    let mut tree: KGST<char, String> = KGST::new('$');

    let reader = fasta::Reader::from_file(file).unwrap();

    let mut count = 0;
    
    for result in reader.records() {

        let result_data = result.unwrap();

        let seq: Vec<char> = result_data.seq()
        .to_vec()
        .iter()
        .map(|x| *x as char)
        .collect();
        
        if seq.len()<=1{
            continue;
        }
        tree.insert(result_data.id().to_string(), seq.to_vec(), max_depth);

        pb.inc(1);   
        count+=1;
        if &count==num_seq {
            break;
        }
    }
    tree
}

fn save_tree_edges(tree: &KGST<char, String>, output_path: String){
    println!("Saving tree nodes to {}.", &output_path);
    let edge_iter = tree.iter_edges_post();
    println!("Writing nodes");
    let mut f = File::create(output_path).expect("Unable to create file");
    for (n1, n2) in edge_iter{
        writeln!(f, "{} {}", n1, n2).expect("Write failed");
    }
    println!("Saved");
}

fn save_tree(tree: &KGST<char, String>, output_path: String){
    println!("Saving tree nodes to {}.", &output_path);
    let edge_iter = tree.iter_edges_post();
    let mut f = File::create(output_path).expect("Unable to create file");
    writeln!(f, "start kgst").expect("Write failed");
    writeln!(f, "start edges").expect("Write failed");
    for (n1, n2) in edge_iter{
        writeln!(f, "{}->{}; {}", n1, n2, tree.get_node_label(&n2).iter().map(|x| format!("{}", x)).collect::<String>()).expect("Write failed");
    }
    writeln!(f, "end").expect("Write failed");
    println!("Saved");
}

fn node_sim(tree: &KGST<char, String>, output_path: String){
    println!("Saving tree strings to {}.", &output_path);
    let string_iter = tree.iter_strings();
    let pb = ProgressBar::new(string_iter.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    let mut f = File::create(output_path).expect("Unable to create file");
    writeln!(f, "ID,node_values").expect("Write failed");
    for (_itemid, (item, _depth)) in string_iter{
        let mut node_values: Vec<u8> = vec![0; tree.num_nodes()];
        for node_id in item.get_nodes().iter(){
            for path_node in tree.get_node_path_pre(node_id).iter(){
                node_values[*path_node] = 1;
            }
        }
        writeln!(f, "{},{}", item.get_id(), node_values.iter().map(|i| format!("{}", i)).collect::<String>()).expect("Write failed");
        pb.inc(1);
    }
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
            .arg(arg!(-d --depth <MAX_DEPTH> "max depth of output tree. (0==length of longest string)")
                .required(true)
                .value_parser(clap::value_parser!(usize))

                )
            .arg(arg!(-n --num <NUM_SEQ> "Number of seq. (0==all)")
                .required(true)
                .value_parser(clap::value_parser!(usize))
                )
            .arg(arg!(--network "Export edges as network topology")
                .required(false)
                .value_parser(clap::value_parser!(bool))
                )
            .arg(arg!(--sim "Export node values per string")
                .required(false)
                .value_parser(clap::value_parser!(bool))
                )
        )
        .about("CLI tool to build and serialize K-Truncated Generalized Suffix trees")
        .get_matches();

        match matches.subcommand(){
            Some(("build",  sub_m)) => {
                let tree: KGST<char, String> = build_tree(
                    sub_m.get_one::<String>("source").expect("required").as_str(), 
                    sub_m.get_one::<usize>("num").expect("required"), 
                    sub_m.get_one::<usize>("depth").expect("required")
                );
                if sub_m.get_flag("network"){
                    save_tree_edges(&tree, sub_m.get_one::<String>("out").expect("required").to_string());
                }
                else if sub_m.get_flag("sim"){
                    node_sim(&tree, sub_m.get_one::<String>("out").expect("required").to_string());
                }
                else{
                    save_tree(&tree, sub_m.get_one::<String>("out").expect("required").to_string());
                }
            },
            _ => {
                println!("No option selected! Refer help page (-h flag)");
            }
        }
}