use clap::Parser;
use rhai::Scope;

// simple program to run rhai scripts with a input file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    script: String,
    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    std::env::set_var("RUST_LOG", "DEBUG");
    pretty_env_logger::init_timed();
    let args = Args::parse();
    log::info!("Starting RHAI engine");
    let mut engine = rhai::Engine::new();
    engine.on_print(|s| log::debug!("{s}"));
    engine.on_debug(|text, source, pos| log::debug!("{source:?}:{pos} > {text}"));
    log::info!("Reading inputs");
    let input = if let Some(input_file) = args.input {
        String::from_utf8(std::fs::read(input_file).unwrap()).unwrap()
    } else {
        String::from("no input file given!")
    };
    let script = String::from_utf8(std::fs::read(&args.script).unwrap()).unwrap();
    log::info!("Creating Scrope...");
    let mut scope = Scope::new();
    scope.push("input", input);
    log::info!("Compile Script {}", &args.script);
    let compiled = engine.compile(script).unwrap();
    log::info!("Running Script {}", &args.script);
    let start = std::time::Instant::now();
    engine.run_ast_with_scope(&mut scope, &compiled).unwrap();
    let diff = std::time::Instant::now().duration_since(start).as_millis();
    log::info!("Script ended after {}ms", diff);
}
