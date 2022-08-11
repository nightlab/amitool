use clap::{Arg, App, SubCommand, AppSettings};
use std::fs::File;
use std::io::prelude::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn read_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn merge_rom(evenfn: &str, oddfn: &str, outfn: &str, byteswap: bool) -> Result<(), String> {
    let bufeven = match read_file(evenfn) {
        Ok(v) => { v }
        Err(_) => { return Err(format!("Can't open/read even file \"{}\"!", evenfn).to_string()); }
    };
    let bufodd = match read_file(oddfn) {
        Ok(v) => { v }
        Err(_) => { return Err(format!("Can't open/read odd file \"{}\"!", oddfn).to_string()); }
    };
    if bufeven.len() != bufodd.len() {
        return Err(format!("Source files size mismatch ({} != {} bytes)!", bufeven.len(), bufodd.len()));
    }
    if bufeven.len() % 2 == 1 {
        return Err(format!("Source files have odd file size ({} bytes)!", bufeven.len()));
    }
    let mut fout = match File::create(outfn) {
        Ok(f) => { f }
        Err(_) => { return Err(format!("Can't create output file \"{}\"!", outfn).to_string()); }
    };
    let mut out: Vec<u8> = Vec::new();
    let il = bufeven.len();
    if byteswap {
        for n in (0..il).step_by(2) {
            out.push(bufeven[n + 1]);
            out.push(bufeven[n]);
            out.push(bufodd[n + 1]);
            out.push(bufodd[n]);
        }
    } else {
        for n in (0..il).step_by(2) {
            out.push(bufeven[n]);
            out.push(bufeven[n + 1]);
            out.push(bufodd[n]);
            out.push(bufodd[n + 1]);
        }
    }
    match fout.write_all(&out) {
        Ok(_) => {}
        Err(_) => { return Err(format!("Can't write output file \"{}\"!", outfn).to_string()); }
    }
    Ok(())
}

fn split_rom(infn: &str, evenfn: &str, oddfn: &str, byteswap: bool) -> Result<(), String> {
    let bufin = match read_file(infn) {
        Ok(v) => { v }
        Err(_) => { return Err(format!("Can't open/read rom file \"{}\"!", infn).to_string()); }
    };
    let mut fout1 = match File::create(evenfn) {
        Ok(f) => { f }
        Err(_) => { return Err(format!("Can't create output file \"{}\"!", evenfn).to_string()); }
    };
    let mut fout2 = match File::create(oddfn) {
        Ok(f) => { f }
        Err(_) => { return Err(format!("Can't create output file \"{}\"!", oddfn).to_string()); }
    };
    let mut out1: Vec<u8> = Vec::new();
    let mut out2: Vec<u8> = Vec::new();
    let il = bufin.len();
    if byteswap {
        for n in (0..il).step_by(4) {
            out1.push(bufin[n + 1]);
            out1.push(bufin[n]);
            out2.push(bufin[n + 3]);
            out2.push(bufin[n + 2]);
        }
    } else {
        for n in (0..il).step_by(4) {
            out1.push(bufin[n]);
            out1.push(bufin[n + 1]);
            out2.push(bufin[n + 2]);
            out2.push(bufin[n + 3]);
        }
    }
    match fout1.write_all(&out1) {
        Ok(_) => {}
        Err(_) => { return Err(format!("Can't write output file \"{}\"!", evenfn).to_string()); }
    }
    match fout2.write_all(&out2) {
        Ok(_) => {}
        Err(_) => { return Err(format!("Can't write output file \"{}\"!", oddfn).to_string()); }
    }
    Ok(())
}

fn main() {
    let matches = App::new("Amiga ROM Tool")
        .version(VERSION)
        .author("Stephan König <tb@octoplex.org>")
        .setting(AppSettings::GlobalVersion)
        .subcommand(
            SubCommand::with_name("merge")
                .about("Merge 2 split odd/even rom files into a single rom")
                .arg(Arg::with_name("byteswap").short("b").long("byteswap"))
                .arg(Arg::with_name("evenfile").index(1).required(true))
                .arg(Arg::with_name("oddfile").index(2).required(true))
                .arg(Arg::with_name("outputfile").index(3).required(true))
        ).subcommand(
            SubCommand::with_name("split")
                .about("Split rom file into odd/even files")
                .arg(Arg::with_name("byteswap").short("b").long("byteswap"))
                .arg(Arg::with_name("inputfile").index(1).required(true))
                .arg(Arg::with_name("evenfile").index(2).required(true))
                .arg(Arg::with_name("oddfile").index(3).required(true))
        ).get_matches();
	if let Some(m) = matches.subcommand_matches("merge") {
        let in1fn = m.value_of("evenfile").unwrap();
        let in2fn = m.value_of("oddfile").unwrap();
        let outfn = m.value_of("outputfile").unwrap();
        let bs = m.is_present("byteswap");
        match merge_rom(in1fn, in2fn, outfn, bs) {
            Ok(_) => {}
            Err(e) => { println!("ERROR: {}", e); }
        }
        return;
    }
    if let Some(m) = matches.subcommand_matches("split") {
        let infn = m.value_of("inputfile").unwrap();
        let out1fn = m.value_of("evenfile").unwrap();
        let out2fn = m.value_of("oddfile").unwrap();
        let bs = m.is_present("byteswap");
        match split_rom(infn, out1fn, out2fn, bs) {
            Ok(_) => {}
            Err(e) => { println!("ERROR: {}", e); }
        }
        return;
    }
}
