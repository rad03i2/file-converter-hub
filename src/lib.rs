use std::{fs, io, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Newline { Lf, CrLf }

pub fn convert_delimited(input: &str, from: char, to: char) -> Result<String, String> {
    if from == to { return Ok(input.to_string()); }
    let rows = parse_delimited(input, from)?;
    Ok(rows.into_iter().map(|r| r.into_iter().map(|f| escape_field(&f, to)).collect::<Vec<_>>().join(&to.to_string())).collect::<Vec<_>>().join("\n"))
}

fn parse_delimited(input: &str, delim: char) -> Result<Vec<Vec<String>>, String> {
    let mut rows = Vec::new(); let mut row = Vec::new(); let mut field = String::new();
    let mut chars = input.chars().peekable(); let mut quoted = false;
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') { chars.next(); field.push('"'); } else { quoted = false; }
            } else { field.push(c); }
        } else {
            match c {
                '"' if field.is_empty() => quoted = true,
                c if c == delim => { row.push(std::mem::take(&mut field)); },
                '\n' => { row.push(std::mem::take(&mut field)); rows.push(std::mem::take(&mut row)); },
                '\r' if chars.peek() == Some(&'\n') => {},
                _ => field.push(c),
            }
        }
    }
    if quoted { return Err("unterminated quoted field".into()); }
    if !field.is_empty() || !row.is_empty() { row.push(field); rows.push(row); }
    Ok(rows)
}

fn escape_field(field: &str, delim: char) -> String {
    if field.contains(delim) || field.contains('"') || field.contains('\n') || field.contains('\r') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else { field.to_string() }
}

pub fn normalize_newlines(input: &str, style: Newline, final_newline: bool) -> String {
    let unified = input.replace("\r\n", "\n").replace('\r', "\n");
    let trimmed = unified.trim_end_matches('\n');
    let sep = match style { Newline::Lf => "\n", Newline::CrLf => "\r\n" };
    let body = trimmed.replace('\n', sep);
    if final_newline { format!("{body}{sep}") } else { body }
}

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub fn base64_encode(data: &[u8]) -> String {
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let a=chunk[0]; let b=*chunk.get(1).unwrap_or(&0); let c=*chunk.get(2).unwrap_or(&0);
        out.push(B64[(a>>2) as usize] as char); out.push(B64[(((a&3)<<4)|(b>>4)) as usize] as char);
        out.push(if chunk.len()>1 { B64[(((b&15)<<2)|(c>>6)) as usize] as char } else {'='});
        out.push(if chunk.len()>2 { B64[(c&63) as usize] as char } else {'='});
    } out
}

pub fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let clean: Vec<u8> = input.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    if clean.len()%4 != 0 { return Err("base64 length must be a multiple of 4".into()); }
    let mut out=Vec::new();
    for (idx, chunk) in clean.chunks(4).enumerate() {
        let last = idx == clean.len()/4-1;
        if !last && (chunk[2]==b'=' || chunk[3]==b'=') { return Err("padding is only allowed in the final block".into()); }
        let v0=val(chunk[0])?; let v1=val(chunk[1])?;
        if chunk[0]==b'=' || chunk[1]==b'=' { return Err("invalid base64 padding".into()); }
        out.push((v0<<2)|(v1>>4));
        if chunk[2]!=b'=' { let v2=val(chunk[2])?; out.push((v1<<4)|(v2>>2)); if chunk[3]!=b'=' { let v3=val(chunk[3])?; out.push((v2<<6)|v3); } }
        else if chunk[3]!=b'=' { return Err("invalid base64 padding".into()); }
    } Ok(out)
}
fn val(b:u8)->Result<u8,String>{ match b { b'A'..=b'Z'=>Ok(b-b'A'), b'a'..=b'z'=>Ok(b-b'a'+26), b'0'..=b'9'=>Ok(b-b'0'+52), b'+'=>Ok(62), b'/'=>Ok(63), b'='=>Ok(0), _=>Err(format!("invalid base64 character: {}", b as char)) } }

pub fn write_output(path: &Path, data: &[u8], overwrite: bool) -> io::Result<()> {
    if path.exists() && !overwrite { return Err(io::Error::new(io::ErrorKind::AlreadyExists, "output exists; pass --overwrite to replace it")); }
    let parent=path.parent().unwrap_or_else(||Path::new(".")); fs::create_dir_all(parent)?;
    let name=path.file_name().and_then(|s|s.to_str()).unwrap_or("output");
    let tmp=parent.join(format!(".{name}.tmp-{}", std::process::id()));
    fs::write(&tmp,data)?;
    if overwrite && path.exists() { fs::remove_file(path)?; }
    match fs::rename(&tmp,path) { Ok(())=>Ok(()), Err(e)=>{let _=fs::remove_file(&tmp);Err(e)} }
}
