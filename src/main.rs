#[macro_use]
extern crate clap;

extern crate cursive;

mod instructions;
mod interpreter;


use clap::*;

fn main()
{
  let matches = App::new("brfk")
    .version(crate_version!())
    .author(crate_authors!())
    .about("A Brainfuck Interpreter, Compiler, Debugger, and Optimizer.\n\
           Currently only has a working interpreter, and only supports basic \
           and Extended Type I Brainfuck.")
    .arg(Arg::with_name("file")
         .short("p").long("path").value_name("FILE")
         .help("The path to the Brainfuck source code")
         .required(true).takes_value(true))
    .arg(Arg::with_name("mode")
         .long("mode")
         .value_name("MODE")
         .takes_value(true)
         .require_equals(true)
         .possible_values(&["b", "x1"])//, "x2", "x3", "bp"])
         .hide_possible_values(true)
         .help("The mode that the Brainfuck code should be parsed in.\n\
               b     Basic mode (Default)\n\
               x1    Extended Type I mode"))//\n\
//               x2    Extended Type II mode\n\
//               x3    Extended Type III mode\n\
//               bp    BrainPlus mode"))
//    .arg(Arg::with_name("ext")
//         .short("e").long("extensions")
//         .help("Enables brfk specific extensions"))
    .setting(AppSettings::ColoredHelp)
    .get_matches();

  let file = matches.value_of("file").unwrap();
  let mode = match matches.value_of("mode").unwrap_or("b")
  {
    "b"  => instructions::ParseMode::Basic,
    "x1" => instructions::ParseMode::Extended1,
    "x2" => instructions::ParseMode::Extended2,
    "x3" => instructions::ParseMode::Extended3,
    "bp" => instructions::ParseMode::BrainPlus,
    err  => panic!("Invalid mode: {}", err)
  };
  let enable_extensions = matches.is_present("ext");
  let mut program = interpreter::Program::new(
                      instructions::parse_code(
                        read_file(std::path::Path::new(file)),
                        mode, enable_extensions).unwrap());

  program.run();
}

fn read_file(file_path: &std::path::Path) -> String
{
  use std::io::Read;

  let mut file = std::fs::File::open(file_path)
    .expect(&format!("File not found: {}", file_path.to_str().unwrap()));

  let mut file_contents = String::new();
  file.read_to_string(&mut file_contents).expect("Could not read file");
  file_contents
}
