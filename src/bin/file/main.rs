use shaaa::*;
use std::time::*;
use std::io::prelude::*;

#[derive(paw_structopt::StructOpt, structopt::StructOpt)]
struct Args {
    #[structopt(short="v", long="variant", raw(possible_values="&[\"224\", \"256\", \"384\", \"512\"]"), default_value="256")]
    /// Which variant to use
    variant: usize,
}

#[paw::main]
fn main(args: Args) {
    let mut sha = from_length(args.variant).unwrap();

    let begin = Instant::now();

    let mut tot_size = 0; // In B
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    loop {
        let buf = stdin.fill_buf().unwrap();

        let len = buf.len();
        if len == 0 { break; }

        sha.update(buf);

        tot_size += len;
        stdin.consume(len);
    }

    let dig = sha.digest_renew();

    let end = Instant::now();

    // Stat
    let duration = end - begin;

    let nanos = duration.subsec_nanos();
    let secs = duration.as_secs();
    let secs_f = secs as f64 + nanos as f64 / 1000000000f64;

    let tot_size_m = tot_size as f64 / 1024f64 / 1024f64;
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
