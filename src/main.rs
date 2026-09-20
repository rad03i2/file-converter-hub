use std::{env, fs, path::PathBuf, process};
use file_converter_hub::{base64_decode,base64_encode,convert_delimited,normalize_newlines,write_output,Newline};

fn usage(){eprintln!("File Converter Hub 1.0.0 — Radwan Abdulhadi Ahmed / @rad03i2\n\nUsage:\n  file-converter csv-to-tsv <input> <output> [--overwrite]\n  file-converter tsv-to-csv <input> <output> [--overwrite]\n  file-converter normalize-text <input> <output> [--crlf] [--no-final-newline] [--overwrite]\n  file-converter base64-encode <input> <output> [--overwrite]\n  file-converter base64-decode <input> <output> [--overwrite]\n  file-converter --version");}
fn main(){if let Err(e)=run(){eprintln!("error: {e}");process::exit(2)}}
fn run()->Result<(),String>{
 let args:Vec<String>=env::args().skip(1).collect();
 if args.first().map(String::as_str)==Some("--version"){println!("file-converter 1.0.0 — Radwan Abdulhadi Ahmed / @rad03i2");return Ok(())}
 if args.len()<3 {usage();return Err("missing command, input, or output".into())}
 let cmd=&args[0]; let input=PathBuf::from(&args[1]); let output=PathBuf::from(&args[2]);
 if input==output{return Err("input and output paths must differ".into())}
 let overwrite=args.iter().any(|a|a=="--overwrite");
 let data=fs::read(&input).map_err(|e|format!("cannot read {}: {e}",input.display()))?;
 let result=match cmd.as_str(){
  "csv-to-tsv"=>convert_delimited(std::str::from_utf8(&data).map_err(|_|"CSV must be UTF-8")?,',','\t')?.into_bytes(),
  "tsv-to-csv"=>convert_delimited(std::str::from_utf8(&data).map_err(|_|"TSV must be UTF-8")?,'\t',',')?.into_bytes(),
  "normalize-text"=>{let text=std::str::from_utf8(&data).map_err(|_|"text input must be UTF-8")?;let style=if args.iter().any(|a|a=="--crlf"){Newline::CrLf}else{Newline::Lf};normalize_newlines(text,style,!args.iter().any(|a|a=="--no-final-newline")).into_bytes()},
  "base64-encode"=>{let mut s=base64_encode(&data);s.push('\n');s.into_bytes()},
  "base64-decode"=>base64_decode(std::str::from_utf8(&data).map_err(|_|"base64 input must be UTF-8")?)?,
  _=>{usage();return Err(format!("unknown command: {cmd}"))}
 };
 write_output(&output,&result,overwrite).map_err(|e|format!("cannot write {}: {e}",output.display()))?;
 println!("Converted {} -> {} ({} bytes)",input.display(),output.display(),result.len()); Ok(())
}
