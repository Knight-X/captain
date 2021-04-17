use service;
use clap::App;

pub fn run() {
    let matches = App::new("captain")
        .version("0.0.1")
        .author("kazami")
        .arg(clap::Arg::new("input")
            .about("the input wasm file")
            .required(true)
            .index(1))
        .subcommand(App::new("run")
            .about("run cli tool")
            .version("0.0.1"))
        .get_matches();

    if let Some(i) = matches.value_of("input") {
        println!("Value for input: {}", i);
    }


    let _str = matches.value_of("input").unwrap();
    println!("the path is {}", _str);
    let string = String::from(_str);

    if let Some(ref matches) = matches.subcommand_matches("run") {
            service::task(string);
    }
}

