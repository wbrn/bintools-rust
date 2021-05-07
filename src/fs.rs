use std::process::exit;
use term_size;

use crate::shell;

pub fn run(args: &Vec<String>) {
    if args.len() < 1 {
        eprintln!("Usage: fs [options] <search patterns>");
        exit(1);
    }

    let (_, h) = term_size::dimensions().unwrap();

    // Try to split search strings and options
    let mut sstrs: Vec<&str> = Vec::new();
    let mut opts: Vec<&str> = Vec::new();
    for s in args {
        if '-' as u8 != s.as_bytes()[0] {
            sstrs.push(s);
        } else {
            opts.push(s);
        }
    }

    let sstr = sstrs.join(" ");
    let opt = opts.join(" ");
    let file_pos_cmd = format!(r#"rg --color=always -n {} "{}" | fzf --ansi -e --tac -0 -1 --cycle --min-height=20 -d ':' --preview="echo '\033[1;32m  {{1}}\033[0m'; fspreview {{}} {}" --preview-window=right:60% | gawk -F':' '{{printf "%s +%s", $1, $2}}'"#, opt, sstr, h);
    let file_pos_out = shell::run_with_out(&file_pos_cmd);
    if file_pos_out.stdout != "" {
        let edit_file_cmd = "nvim ".to_owned() + &file_pos_out.stdout;
        shell::run(&edit_file_cmd);
    }
}
