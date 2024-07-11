use clap::{App, Arg};
use die_scan::{die_scan, DIE_SHOWERRORS};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

fn main() {
    let matches = App::new("die_scan_example")
        .version("1.0")
        .author("Your Name : AutoNoob")
        .about("Scans files using diedll.dll")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the input file to scan")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("flags")
                .short('x')
                .long("flags")
                .value_name("FLAGS")
                .help("Sets the scanning flags")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let file_name = matches.value_of("file").unwrap();
    let flags = matches.value_of("flags").unwrap_or("1").parse::<u32>().unwrap_or(DIE_SHOWERRORS);

    match die_scan(file_name, flags) {
        Ok(result) => {
            let upx_keyword = "UPX";
            let vmp_keyword = "VMP";
            let parts_upx: Vec<_> = result.split(upx_keyword).collect();
            let mut parts_vmp: Vec<String> = Vec::new();

            for (i, part) in parts_upx.iter().enumerate() {
                if i > 0 {
                    parts_vmp.push(format!("{0}{1}", SetForegroundColor(Color::Green), upx_keyword));
                }
                parts_vmp.push(format!("{0}{1}", ResetColor, part));
            }

            let result_vmp = parts_vmp.join("");
            let parts_vmp: Vec<_> = result_vmp.split(vmp_keyword).collect();

            print!("Scan result: ");
            for (i, part) in parts_vmp.iter().enumerate() {
                if i > 0 {
                    print!("{0}{1}", SetForegroundColor(Color::Red), vmp_keyword);
                }
                print!("{0}{1}", ResetColor, part);
            }
            println!();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
