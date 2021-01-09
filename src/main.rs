use std::fs;
use std::path::{PathBuf};
use structopt::StructOpt;
use glob::glob;
use pandoc::{self, OutputKind};

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    src: PathBuf,

    #[structopt(parse(from_os_str))]
    dest: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let src_root_dir: &str = args.src.to_str().unwrap();
    let dest_root_dir: &str = args.dest.to_str().unwrap();
    let mut num_succeed = 0;
    let mut num_failed = 0;

    for src_path in glob(&format!("{}/**/*.wiki", src_root_dir)).unwrap().filter_map(Result::ok) {
        let src_path_after_root: String = src_path.to_str().unwrap().chars().skip(src_root_dir.len()).collect();
        let dest_path_str = format!("{}{}", dest_root_dir, src_path_after_root)
            .replace(".wiki", ".org");

        let dest_path = PathBuf::from(&dest_path_str);
        let mut dest_path_dir = dest_path.clone();

        let mut pandoc = pandoc::new();
        pandoc.add_input(&src_path);
        pandoc.set_output(OutputKind::File(dest_path));

        // Create output directories if needed
        dest_path_dir.pop();
        fs::create_dir_all(dest_path_dir).unwrap();

        let res = pandoc.execute();

        if res.is_ok() {
            println!("Succeed: {}", dest_path_str);
            num_succeed += 1;
        } else {
            println!("Failed: {}", dest_path_str);
            num_failed += 1;

            eprintln!("{:?}", res.err().unwrap());
        }
    }


    println!("Succeed: {}, Failed: {}", num_succeed, num_failed);
}
