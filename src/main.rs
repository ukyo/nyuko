extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
  let matches = App::new("My Super Program")
    .version("1.0")
    .author("hoge")
    .about("fuga")
    .arg(Arg::with_name("hoge")
      .short("h")
      .long("hoge")
      .takes_value(true)
      .help("Hoge!!!!"))
    .get_matches();
  
  println!("hoge is: {}", matches.value_of("hoge").unwrap());

}