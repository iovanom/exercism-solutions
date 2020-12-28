use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    if !input.is_empty() {
        let mut threads = Vec::with_capacity(worker_count);
        let chunk_size = (input.len() as f64 / worker_count as f64).ceil() as usize;

        input.chunks(chunk_size).for_each(|chunk| {
            let in_vec = Vec::from(chunk);
            let thread = thread::spawn(move || {
                let mut result: HashMap<char, usize> = HashMap::new();
                for sentence in in_vec {
                    for c in sentence
                        .chars()
                        .map(|c| c.to_ascii_lowercase())
                        .filter(|c| c.is_alphabetic())
                    {
                        let counter = result.entry(c).or_insert(0);
                        *counter += 1;
                    }
                }
                result
            });
            threads.push(thread);
        });
        for thread in threads {
            let res = thread.join().expect("thread panicked");
            for (&k, v) in res.iter() {
                let counter = result.entry(k).or_insert(0);
                *counter += v;
            }
        }
    }
    result
}
