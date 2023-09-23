use generalized_suffix_tree::suffix_tree::*;

#[test]
fn create_tree() {
    let _tree: KGST<char, String> = KGST::new('$');
}

#[test]
fn insert(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "MKAILVVLLYTFTTADADTLCIGYHANNSTDTVDTVLEKNVTVTHSVNLLENRHNGKLCKLRGVAPLHLGKCNIAGWILGNPECESLSTAGSWSYIVETSNPDNGTCYPGDFINYEELREQLSSVSSFEKFEIFPKTSSWPNHDTNRGVTAACPHDGAKSFYRNLLWLVKKEKENSYPMINKSYTNNKGKEVLVLWAIHHPATSADQQSLYQNANAYVFVGSSKYSKKFEPEIAARPKVRDQAGRMKYYWTLVEPGDKITFEATGNLVVPIYAFALKRNSGSGIIISDTSVHDCDTTCQTPNGAINTSLPFQNIHPVTIGECPKYVKSTKLRMATGLRNIPSIQSRGLFGAIAGFIEGGWTGMIDGWYGYHHQNEQGSGYAADLKSTQNAIDGITNKVNSVIEKMNTQFTAVGKEFNHLERRIENLNKKVDDGFLDIWTYNAELLVLLENERTLDYHDSNVKNLYEKVRSQLKNNAKEIGNGCFEFYHKCDDTCMESVKNGTYDYPKYSEEAKLNREEIDGVKLESTRIYQILAIYSTVASSLVLVVSLGAISFWMCSNGSLQCRICI".chars().collect();
    let item_id:String = "World".to_string();
    tree.insert(item_id.clone(), item_string.clone(),&0);
    dbg!(tree.get_node(&66));
    for i in 0..item_string.len(){
        assert!(tree.is_suffix(&item_string[i..]));
    }
}

#[test]
fn insert_set(){
    let mut tree: KGST<char, String> = KGST::new('$');
    // let string_set: Vec<Vec<char>> = vec!["GATTACAGATTACAXYZGATTACAGATTACA".to_string().chars().collect(), "CXYZTTATAGCXYZCGTACAGACCGAA".to_string().chars().collect()];
    let string_set: Vec<Vec<char>> = vec![
        "MKAILVVLLYTFTTADADTLCIGYHANNSTDTVDTVLEKNVTVTHSVNLLENRHNGKLCKLRGVAPLHLGKCNIAGWILGNPECESLSTAGSWSYIVETSNPDNGTCYPGDFINYEELREQLSSVSSFEKFEIFPKTSSWPNHDTNRGVTAACPHDGAKSFYRNLLWLVKKEKENSYPMINKSYTNNKGKEVLVLWAIHHPATSADQQSLYQNANAYVFVGSSKYSKKFEPEIAARPKVRDQAGRMKYYWTLVEPGDKITFEATGNLVVPIYAFALKRNSGSGIIISDTSVHDCDTTCQTPNGAINTSLPFQNIHPVTIGECPKYVKSTKLRMATGLRNIPSIQSRGLFGAIAGFIEGGWTGMIDGWYGYHHQNEQGSGYAADLKSTQNAIDGITNKVNSVIEKMNTQFTAVGKEFNHLERRIENLNKKVDDGFLDIWTYNAELLVLLENERTLDYHDSNVKNLYEKVRSQLKNNAKEIGNGCFEFYHKCDDTCMESVKNGTYDYPKYSEEAKLNREEIDGVKLESTRIYQILAIYSTVASSLVLVVSLGAISFWMCSNGSLQCRICI".to_string().chars().collect(), 
        "MKAILVVLLYTFTTANADTLCIGYHANNSTDTVDTVLEKNVTVTHSVNLLEDKHNGKLCKLRGVAPLHLGKCNIAGWILGNPECESLSTARSWSYIVETSNSDNGTCYPGDFINYEELREQLSSVSSFERFEIFPKTSSWPNHDSNKGVTAACPHAGAKSFYKNLVWLVKKEKENSYPKLNKTYINDKGKEVLVLWGIHHPPTTADQQSLYQNADAYVFVGTSRYSKKFKPEIAKRPKVRDQEGRMNYYWTLVEPGDKITFEATGNLVVPRYAFTMERNAGSGIIISDTPVHDCNTTCQTPEGAINTSLPFQNVHPITIGKCPKYVKSTKLRLATGLRNVPSIQSRGLFGAIAGFIEGGWTGMVDGWYGYHHQNEQGSGYAADLKSTQNAIDEITNKVNSVIEKMNTQFTAVGKEFNHLEKRIENLNKKVDDGFLDIWTYNAXLLVLLENERTLDYHDSNVKNLYEKVRTQLKNNAKEIGNGCFEFYHKCDNTCMESVKNGTYDYPKYSEEAKLNREKIDGVKLESTRIYQILAIYSTVASSLVLVVSLGAISFWMCSNGSLQCRICI".to_string().chars().collect(), 
        "MKAIIVVVLLYTFTTANADTLCIGYHANNSTDTVDTVLEKNVTVTHSVNLLKNRHNGKLCKLRGVAPLHLGKCNIAGWLLGNPECEPLSTASSWAYIVETSNSDNGTCYPGDFINYEELTEHLSSVSSFERFEIFPKTNSWPNHDTNKGVTAACPHAGTNSFYRNGIWLVKKENIYPKKSKSYKNKKKKEVLVLWAIHHPSTSADQQSLYQNADAYVFVGSSRYSRKFEPEIATRPKVRDQAGRMNYYWTLVEPGDKITFEATGNLVAPRYAFALKRNSGSGIIISDTSVHDCDTTCQTPNGAINTSLPFQNIHPVTIGECPKYVKSTKLRMATGLRNIPSIQSRGLFGAIAGFIEGGWTGMIDGWYGYHHQNEQGSGYAADLKSTQNAIDGITNKVNSVIEKMNTQFTAVGKEFSHLERRIENLNKKVDDGFLDIWTYNAELLVLLENERTLDYHDSNVKNLYEKVRSQLKNNAKEIGNGCFEFYHKCDDMCMESVKNGTYDYPKYSEEAKLNREEIDGVKLESTRIYQILAIYSTVASSLVLVVSLGAISFWMCSNGSLQCRICI".to_string().chars().collect(), 
        "MKAIIVVYLLHLQTATYADTLCIGYHANNSTDTVDTVLEKNVTVTHSVNLLENRHNGKLCKLRGVAPLHLGKCNIAGWLLGNPECESLSTASSWSYIVETSNSDNGTCYPGDFINYEELREQLSSVSSFERFEIFPKTSSWPNHDTNRGVTAACPHAGTNSFYRNLVWLVKKGNSYPKINKSYINNKEKEVLVLWAIHHPSTSADQQSLYQNADAYVFVGSSRYSKKFEPEIATRPKVRDQAGRMNYYWTLVEPGDKITFEATGNLVAPRYAFALKRNSGSGIIISDTSVHDCDTTCQTPNGAINTSLPFQNIHPVTIGECPKYVKSTKLRMATGLRNIPSIQSRGLFGAIAGFIEGGWTGMIDGWYGYHHQNEQGSGYAADLKSTQNAIDGITNKVNSVIEKMNTQFTAVGKEFSHLERRIENLNKKVDDGFLDIWTYNAELLVLLENERTLDYHDSNVKNLYEKVRSQLKNNAKEIGNGCFEFYHKCDDMCMESVKNGTYDYPKYSEEAKLNREGIDGVKLESTRIYQILAIYSTVASSLVLVVSLGAISFWMCSNGSLQCRICI".to_string().chars().collect(), 
    ];
    for item_idx in 0..string_set.len(){
        tree.insert(dbg!(item_idx.to_string()), string_set[item_idx].clone(), &0);
    }
    for item_idx in 0..string_set.len(){
        for string_idx in 0..string_set[item_idx].len()-1{
            let suffix_match = tree.suffix_match(&string_set[item_idx][string_idx..]);
            assert!(suffix_match.get(&item_idx.to_string()).expect("suffix not found!").contains(&string_idx));
        }
    }
}

// #[test]
// fn serialize_deserialize_tree(){
//     let mut tree: KGST<char, String> = KGST::new('$');
//     let item_string:Vec<char> = "GATTACAGATTACAXYZGATTACAGATTACA".chars().collect();
//     let item_id:String = "World".to_string();
//     tree.insert(item_id.clone(), item_string.clone(), &0);
//     let serialized = serde_json::to_string(&tree).unwrap();
//     let tree_2: KGST<char, String> = serde_json::from_str(&serialized).unwrap();
//     assert_eq!(tree.get_nodes(), tree_2.get_nodes());
// }


// #[test]
// fn insert_trunc(){
//     let mut tree: KGST<char, String> = KGST::new('$');
//     let item_string:Vec<char> = "MKAILVVLLYTFTTADADTLCIGYHANNSTDTVDTVLEKNVTVTHSVNLLENRHNGKLCKLRGVAPLHLGKCNIAGWILGNPECESLSTAGSWSYIVETSNPDNGTCYPGDFINYEELREQLSSVSSFEKFEIFPKTSSWPNHDTNRGVTAACPHDGAKSFYRNLLWLVKKEKENSYPMINKSYTNNKGKEVLVLWAIHHPATSADQQSLYQNANAYVFVGSSKYSKKFEPEIAARPKVRDQAGRMKYYWTLVEPGDKITFEATGNLVVPIYAFALKRNSGSGIIISDTSVHDCDTTCQTPNGAINTSLPFQNIHPVTIGECPKYVKSTKLRMATGLRNIPSIQSRGLFGAIAGFIEGGWTGMIDGWYGYHHQNEQGSGYAADLKSTQNAIDGITNKVNSVIEKMNTQFTAVGKEFNHLERRIENLNKKVDDGFLDIWTYNAELLVLLENERTLDYHDSNVKNLYEKVRSQLKNNAKEIGNGCFEFYHKCDDTCMESVKNGTYDYPKYSEEAKLNREEIDGVKLESTRIYQILAIYSTVASSLVLVVSLGAISFWMCSNGSLQCRICI".chars().collect();
//     let item_id:String = "World".to_string();
//     let max_depth: usize = 3;
//     tree.insert(item_id.clone(), item_string.clone(), &max_depth);
//     for i in 0..item_string.len()-max_depth{
//         let substring_match = tree.substring_match(&item_string[i..i+max_depth-1]);
//         assert!(substring_match.get(&item_id).expect("substring not found!").contains(&i));
//     }
// }

// #[test]
// fn insert_set_trunc(){
//     let mut tree: KGST<char, String> = KGST::new('$');
//     let string_set: Vec<Vec<char>> = vec!["GATTACAGATTACAXYZGATTACAGATTACA".to_string().chars().collect(), "CXYZTTATAGCXYZCGTACAGACCGAA".to_string().chars().collect()];
//     let id_set:Vec<String> = vec!["first".to_string(),"second".to_string()];
//     let it = string_set.iter().zip(id_set.iter());
//     let max_depth: usize = 3;
//     for (string,id) in it{
//         tree.insert(id.clone(), string.clone(), &max_depth);
//     }
//     for item_idx in 0..string_set.len(){
//         for string_idx in 0..string_set[item_idx].len()-max_depth{
//             let substring_match = tree.substring_match(&string_set[item_idx][string_idx..string_idx+max_depth]);
//             assert!(substring_match.get(&id_set[item_idx]).expect("substring not found!").contains(&string_idx));
//         }
//     }
// }

// #[test]
// fn insert_set_var_trunc(){
//     let mut tree: KGST<char, String> = KGST::new('$');
//     let string_set: Vec<Vec<char>> = vec!["GATTACAGATTACAXYZGATTACAGATTACA".to_string().chars().collect(), "CXYZTTATAGCXYZCGTACAGACCGAA".to_string().chars().collect()];
//     let id_set:Vec<String> = vec!["first".to_string(),"second".to_string()];
//     let max_depth: Vec<usize> = vec![3, 5];
//     for i in 0..string_set.len(){
//         tree.insert(id_set[i].clone(), string_set[i].clone(), &max_depth[i]);
//     }

//     for item_idx in 0..string_set.len(){
//         for string_idx in 0..string_set[item_idx].len()-max_depth[item_idx]{
//             let substring_match = tree.substring_match(&string_set[item_idx][string_idx..string_idx+max_depth[item_idx]]);
//             assert!(substring_match.get(&id_set[item_idx]).expect("Substring not found!").contains(&string_idx));
//         }
//     }

//     let mut tree: KGST<char, String> = KGST::new('$');
//     let string_set: Vec<Vec<char>> = vec!["GATTACAGATTACAXYZGATTACAGATTACA".to_string().chars().collect(), "CXYZTTATAGCXYZCGTACAGACCGAA".to_string().chars().collect()];
//     let id_set:Vec<String> = vec!["first".to_string(),"second".to_string()];
//     let max_depth: Vec<usize> = vec![5, 3];
//     for i in 0..string_set.len(){
//         tree.insert(id_set[i].clone(), string_set[i].clone(), &max_depth[i]);
//     }
//     for item_idx in 0..string_set.len(){
//         for string_idx in 0..string_set[item_idx].len()-max_depth[item_idx]{
//             let substring_match = tree.substring_match(&string_set[item_idx][string_idx..string_idx+max_depth[item_idx]]);
//             assert!(substring_match.get(&id_set[item_idx]).expect("Substring not found!").contains(&string_idx));
//         }
//     }
// }