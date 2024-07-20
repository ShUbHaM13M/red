use std::process::exit;

use crate::app::App;

pub fn usage() {
    println!(
        r"
Usage: editor [options] [file ...] Edit file(s)

<Options>
    -h, --help : Print this help message
    -r : Readonly mode
    "
    );
}

pub fn command_parser(args: Vec<String>) -> App {
    if args.is_empty() {
        usage();
        exit(0);
    }

    let mut app = App::new();
    let mut file_path = String::new();
    let args_len = args.len();

    //TODO: Support for multiple files
    if args_len == 1 {
        file_path = args.first().expect("Unable to get file path").to_string();
        app.file_paths.push(file_path);
        return app;
    }

    for (index, arg) in args.into_iter().enumerate() {
        let arg = arg.as_str();
        if index == args_len - 1 {
            file_path.push_str(arg);
            app.file_paths.push(file_path);
            break;
        }
        match arg {
            "-h" | "--help" => {
                usage();
                exit(0);
            }
            "-r" => {
                app.readonly = true;
            }
            _ => {
                eprintln!("Unknown argument {arg}");
                usage();
                exit(1);
            }
        }
    }

    app
}
