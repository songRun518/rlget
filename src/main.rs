use clap::Parser;

#[derive(Debug, clap::Parser)]
/// RLget
struct Cli {
    /// Number of download threads
    #[arg(short, long, default_value_t = 2020)]
    threads: u64,

    /// The amount of memory for each thread to chunk request by in KB
    #[arg(long = "mem", default_value_t = 256)]
    memory: u64,

    #[arg(long)]
    /// The output file name [default: value at end of url after /]
    filename: Option<String>,

    url: String,
}

fn main() {
    let cli = Cli::parse();
    let filename = cli.filename.unwrap_or(rlget::filename(&cli.url));

    println!("threads: {}", cli.threads);
    println!("url: {}", cli.url);
    println!("memory: {}", cli.memory);
    println!("filename: {}\n", filename);

    let download = rlget::download::Download {
        threads: cli.threads,
        url: cli.url,
        memory: cli.memory,
        filename,

        ..Default::default()
    };

    download.get();
}
