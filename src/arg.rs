use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short='d', long="data", default_value = "./data", help="The data directory. The input file should be located in this directory. The output file will also be created in this directory.")]
    pub data: String,

    #[arg(short='f', long="fmt", default_value = "Code: {{short_code}}", help="Format for output file. Use {{short_code}} as placeholder for country code.")]
    pub fmt: String,

    #[arg(short='t', long="timeout", default_value_t = 5, help="Timeout for downloading flags in seconds.")]
    pub timeout: u32,
}