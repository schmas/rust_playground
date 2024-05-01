use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::sync::{Arc, Mutex};
use std::thread;

// This is the `main` thread
#[allow(dead_code)]
fn main_original() {
    // This is our data to process.
    // We will calculate the sum of all digits via a threaded map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    //
    // TODO: see what happens to the output if you insert spaces!
    let data = "86967897 7374164718532 97327050364959
1186132 257556472396329 542624962850
7085623470186085190 7960690014725639
38397 96670710609417278323 8747669219
52380795257888236525459 303330302837
58495327135 744041048897885 734297812
699202164389808 73548808413720956532
1627842463745 2589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    /*************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     ************************************************************************/

    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = data.split_whitespace();

    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segment" with a
    // "destructuring assignment"
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.
        //
        // TODO: try removing the 'move' and see what happens
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                // iterate over the characters of our segment..
                .chars()
                // .. convert text-characters to their number value..
                .map(|c| c.to_digit(10).expect("should be a digit"))
                // .. and sum the resulting iterator of numbers
                .sum();

            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result={}", i, result);

            // "return" not needed, because Rust is an "expression language", the
            // last evaluated expression in each block is automatically its value.
            result
        }));
    }

    /*************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     ************************************************************************/

    // combine each thread's intermediate results into a single final sum.
    //
    // we use the "turbofish" ::<> to provide sum() with a type hint.
    //
    // TODO: try without the turbofish, by instead explicitly
    // specifying the type of final_result
    let final_result: u32 = children.into_iter().map(|c| c.join().unwrap()).sum();

    println!("Final sum result: {}", final_result);
}

#[allow(dead_code)]
fn main_with_default_pool() {
    let data = "86967897 7374164718532 97327050364959
1186132 257556472396329 542624962850
7085623470186085190 7960690014725639
38397 96670710609417278323 8747669219
52380795257888236525459 303330302837
58495327135 744041048897885 734297812
699202164389808 73548808413720956532
1627842463745 2589860345374828574668";

    let chunked_data: Vec<_> = data.split_whitespace().collect();

    let results: Vec<u32> = chunked_data
        .par_iter()
        .enumerate()
        .map(|(i, data_segment)| {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result={}", i, result);
            result
        })
        .collect();

    let final_result: u32 = results.into_iter().sum();
    println!("Final sum result: {}", final_result);
}

#[allow(dead_code)]
fn main_with_custom_pool() {
    let data = "86967897 7374164718532 97327050364959
1186132 257556472396329 542624962850
7085623470186085190 7960690014725639
38397 96670710609417278323 8747669219
52380795257888236525459 303330302837
58495327135 744041048897885 734297812
699202164389808 73548808413720956532
1627842463745 2589860345374828574668";

    let thread_indices = Arc::new(Mutex::new(Vec::new()));

    let thread_indices_idx = Arc::clone(&thread_indices);
    ThreadPoolBuilder::new()
        .start_handler(move |idx| {
            thread_indices_idx.lock().unwrap().push(idx);
        })
        .num_threads(5)
        .build_global()
        .unwrap();

    let chunked_data: Vec<_> = data.split_whitespace().collect();
    println!("Number of chunks: {}", chunked_data.len());

    let results: Vec<(usize, u32)> = chunked_data
        .par_iter()
        .enumerate()
        .map(|(i, data_segment)| {
            let idx = thread_indices
                .lock()
                .unwrap()
                .iter()
                .position(|x| *x == rayon::current_thread_index().unwrap())
                .unwrap();

            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("{idx} - processed segment {}, result={}", i, result);
            (i, result)
        })
        .collect();

    let final_result: u32 = results.into_iter().map(|(_, v)| v).sum();
    println!("Final sum result: {}", final_result);
}

fn main() {
    // main_original();
    // main_with_default_pool();
    main_with_custom_pool();
}
