extern crate clap;
use std::collections::HashMap;

use clap::{arg, Command};
use bio::io::fasta;
use generalized_suffix_tree::suffix_tree::KGST;
use indicatif::{ProgressBar, ProgressStyle};
use generalized_suffix_tree;

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

fn save_tree(tree: &mut KGST<char, String>, output_path: String){
    println!("Saving tree to {}.", &output_path);
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(tree).unwrap(),
    ).unwrap();
    println!("Saved");
}

// fn save_tree_strings(tree: &mut KGST<char, String>, output_path: String){
//     println!("Saving tree strings to {}.", &output_path);
//     std::fs::write(
//         output_path,
//         serde_json::to_string_pretty(&tree.export_all_string_nodes()).unwrap(),
//     ).unwrap();
//     println!("Saved");
// }

fn save_nodes(tree: &mut KGST<char, String>, output_path: String){
    println!("Saving tree nodes to {}.", &output_path);
    let tree_nodes = &tree.export_all_nodes();
    println!("Writing nodes");
    std::fs::write(
        output_path.clone(),
        serde_json::to_string_pretty(&tree_nodes).expect("Node export error"),
    ).unwrap();
    // let mut strings_path = output_path.clone();
    // strings_path.push_str("string.json");
    // std::fs::write(
    //     strings_path,
    //     &tree_nodes.1.iter().map(|(k, v)|{
    //         let feature_str = (0..tree.num_nodes()).map(|idx| (match v.contains(&idx){
    //             true => "1".to_string(),
    //             false => "0".to_string(),
    //         })).collect::<String>();
    //         return format!("{},{}", k.clone(), feature_str);
    //     }).collect::<Vec<String>>()
    //     .join("\n"),
    // ).unwrap();
    println!("Saved");
}

// fn save_edges(tree: &mut KGST<char, String>, output_path: String){
//     println!("Saving tree nodes to {}.", &output_path);
//     let tree_edges = &tree.export_all_edges();
//     println!("Writing nodes");
//     std::fs::write(
//         output_path,
//         tree_edges.join("\n"),
//     ).unwrap();
//     println!("Saved");
// }

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
        )
        .subcommand(Command::new("nodes")
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
        )
        .subcommand(Command::new("edges")
        .about("Build suffix tree index from reference fasta file and save edges")
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
        )
        .about("CLI tool to build and serialize K-Truncated Generalized Suffix trees")
        .get_matches();

        match matches.subcommand(){
            Some(("build",  sub_m)) => {
                let mut tree: KGST<char, String> = build_tree(
                    sub_m.get_one::<String>("source").expect("required").as_str(), 
                    sub_m.get_one::<usize>("num").expect("required"), 
                    sub_m.get_one::<usize>("depth").expect("required")
                );
                save_tree(&mut tree, sub_m.get_one::<String>("out").expect("required").to_string());
            },
            Some(("nodes",  sub_m)) => {
                let mut tree: KGST<char, String> = build_tree(
                    sub_m.get_one::<String>("source").expect("required").as_str(), 
                    sub_m.get_one::<usize>("num").expect("required"), 
                    sub_m.get_one::<usize>("depth").expect("required")
                );
                save_nodes(&mut tree, sub_m.get_one::<String>("out").expect("required").to_string());
                // let mut strings_path = sub_m.get_one::<String>("out").expect("required").to_string();
                // strings_path.push_str("string.json");
                // save_tree_strings(&mut tree, strings_path);
            },
            Some(("edges",  sub_m)) => {
                let mut tree: KGST<char, String> = build_tree(
                    sub_m.get_one::<String>("source").expect("required").as_str(), 
                    sub_m.get_one::<usize>("num").expect("required"), 
                    sub_m.get_one::<usize>("depth").expect("required")
                );
                // save_edges(&mut tree, sub_m.get_one::<String>("out").expect("required").to_string());
            },
            _ => {
                println!("Either build a tree or query an existing tree. Refer help page (-h flag)");
            }
        }
}