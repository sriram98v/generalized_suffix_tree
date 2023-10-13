//! # K-Truncated Generalized Suffix Tree
//! Implementation of the [truncated suffix tree](https://pubmed.ncbi.nlm.nih.gov/18283030/)
//! construction of which is performed in linear time
//!
//! ## Examples
//!
//! ```
//! use generalized_suffix_tree::suffix_tree::KGST;
//! 
//! // Initalize empty tree
//! let mut tree: KGST<char, String> = KGST::new('$');
//! 
//! // insert item with corresponding item id
//! let item_string:Vec<char> = "MKAILVVLLYTFTTADADTLCIGYHANNSTDTVDTVLEKNVTVTHSVNLLENRHNGKLCKLRGVAPLHLGKCNIAGWILGNPECESLSTAGSWSYIVETSNPDNGTCYPGDFINYEELREQLSSVSSFEKFEIFPKTSSWPNHDTNRGVTAACPHDGAKSFYRNLLWLVKKEKENSYPMINKSYTNNKGKEVLVLWAIHHPATSADQQSLYQNANAYVFVGSSKYSKKFEPEIAARPKVRDQAGRMKYYWTLVEPGDKITFEATGNLVVPIYAFALKRNSGSGIIISDTSVHDCDTTCQTPNGAINTSLPFQNIHPVTIGECPKYVKSTKLRMATGLRNIPSIQSRGLFGAIAGFIEGGWTGMIDGWYGYHHQNEQGSGYAADLKSTQNAIDGITNKVNSVIEKMNTQFTAVGKEFNHLERRIENLNKKVDDGFLDIWTYNAELLVLLENERTLDYHDSNVKNLYEKVRSQLKNNAKEIGNGCFEFYHKCDDTCMESVKNGTYDYPKYSEEAKLNREEIDGVKLESTRIYQILAIYSTVASSLVLVVSLGAISFWMCSNGSLQCRICI".chars().collect();
//! let item_id:String = "World".to_string();
//! tree.insert(item_id.clone(), item_string.clone(),&0);
//!
//!
//! // Query if some string is a substring in the tree
//! let substring_match = tree.substring_match(&"STVASSLVLVVSLGAISFWMCSNGSLQCRICI".chars().collect::<Vec<char>>());
//! 
//! // Query if some string is a suffix in the tree
//! let suffix_match = tree.suffix_match(&"RGVAPLHLGKCNIAGWILGNPECESLSTAGSWSYIVE".chars().collect::<Vec<char>>());
//!
//! // Clear tree
//! tree.clear();
//! ```

pub mod suffix_node;
pub mod suffix_tree;
pub mod data;
pub mod utils;
pub mod iter;