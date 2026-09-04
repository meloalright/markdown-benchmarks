use std::fs;
use std::time::Instant;
// comrak 0.8 -> 0.54: `ComrakOptions` was renamed to `Options`.
use comrak::{markdown_to_html, Options as ComrakOptions};

fn main() {
    let input = fs::read_to_string("../sample1.md")
        .expect("failed to read file");

    {
        let iterations = 1000;
        let now = Instant::now();
        for _i in 0..iterations {
            let _output = markdown_to_html(&input, &ComrakOptions::default());
        }
        println!("{:6} iterations = {:3}.{:03}s", iterations, now.elapsed().as_secs(), now.elapsed().subsec_millis());
    }

    {
        let iterations = 10_000;
        let now = Instant::now();
        for _i in 0..iterations {
            let _output = markdown_to_html(&input, &ComrakOptions::default());
        }
        println!("{:6} iterations = {:3}.{:03}s", iterations, now.elapsed().as_secs(), now.elapsed().subsec_millis());
    }

    {
        let iterations = 100_000;
        let now = Instant::now();
        for _i in 0..iterations {
            let _output = markdown_to_html(&input, &ComrakOptions::default());
        }
        println!("{:6} iterations = {:3}.{:03}s", iterations, now.elapsed().as_secs(), now.elapsed().subsec_millis());
    }
}
