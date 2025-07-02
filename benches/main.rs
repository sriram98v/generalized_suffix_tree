use generalized_suffix_tree::suffix_tree::*;
use rand::{distributions::Alphanumeric, Rng};
use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(args = [100, 500, 1000, 5000, 10000, 50000, 100000], sample_size = 1, sample_count = 10)]
fn benchmark_suftree_single_string(bencher: divan::Bencher, str_len: usize) {
    bencher
        .with_inputs(|| {
            let item_string:Vec<char> = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(str_len)
                .map(char::from)
                .collect();
            let item_id:String = "World".to_string();

            (item_string, item_id)
        })
        .bench_refs(|(item_string, item_id)| {
            let mut tree: KGST<char, String> = KGST::new('$');
            tree.insert(item_id.clone(), item_string.clone(),&0);
        });
    
}
