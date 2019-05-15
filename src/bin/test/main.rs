use shaaa::*;
use std::time::*;
use rand::Rng;

#[derive(paw_structopt::StructOpt, structopt::StructOpt)]
struct Args {
    #[structopt(short="v", long="variant", raw(possible_values="&[\"224\", \"256\", \"384\", \"512\"]"), default_value="256")]
    /// Which variant to use
    variant: usize,

    #[structopt(short="z", long="zero")]
    /// Use zeros instead of random values to test
    zero: bool,

    #[structopt(short="s", long="size", default_value="128")]
    /// Batch size in KiB
    size: usize,

    #[structopt(short="c", long="count", default_value="16")]
    /// Batch count
    count: usize,
}

#[paw::main]
fn main(args: Args) {
    let mut buf: Vec<u8> = vec![0; args.size * 1024];

    let mut sha = from_length(args.variant).unwrap();

    let begin = Instant::now();

    let mut filling_time = Duration::new(0, 0);

    for _ in 0..args.count {
        if !args.zero {
            let start_fill = Instant::now();
            rand::thread_rng().fill(buf.as_mut_slice());
            filling_time += Instant::now() - start_fill;
        }

        sha.update(&buf);
    }

    let dig = sha.digest_renew();

    let end = Instant::now();

    // Stat
    let duration = (end - begin) - filling_time;

    let nanos = duration.subsec_nanos();
    let secs = duration.as_secs();
    let secs_f = secs as f64 + nanos as f64 / 1000000000f64;

    let tot_size = args.count * args.size; // In KiB
    let tot_size_m = tot_size as f64 / 1024f64;
    let m_per_sec = tot_size_m / secs_f;

    println!("Digest:");
    for i in dig.iter() {
        print!("{:0>2x}", i);
    }
    println!("");

    println!("Time:");
    println!("{}.{:0>9}s", secs, nanos);

    println!("Speed:");
    println!("{} MiB/s", m_per_sec);
}
