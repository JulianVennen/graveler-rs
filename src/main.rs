use rand::{thread_rng, Rng};

const ITERATIONS: usize = 1_000_000_000;

fn main() {
    let num = num_cpus::get();
    println!("Starting {} threads", num);
    let threads = (0..num).map(|_| {
        std::thread::spawn(move || {
            run_iterations(ITERATIONS / num)
        })
    }).collect::<Vec<_>>();

    let mut max_ones: usize = 0;
    for t in threads {
        let ones: usize = t.join().unwrap();
        if ones > max_ones {
            max_ones = ones;
        }
    }
    println!("Max number of ones: {}", max_ones);
}

fn run_iterations(iterations: usize) -> usize  {
    let mut rng = thread_rng();
    let mut max_ones = 0;
    for _ in 0..iterations {
        let mut ones = 0;
        for _ in 0..231 {
            if rng.gen_range(0..4) == 1 {
                ones += 1;
            }
        }

        if ones > max_ones {
            max_ones = ones;
        }
    }

    max_ones
}
