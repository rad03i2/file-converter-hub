use file_converter_hub::{base64_decode,base64_encode,convert_delimited,normalize_newlines,write_output,Newline};
use std::{fs,time::{SystemTime,UNIX_EPOCH}};

#[test] fn csv_to_tsv_preserves_quoted_delimiters(){let got=convert_delimited("name,note\nRadwan,\"hello, world\"",',','\t').unwrap();assert_eq!(got,"name\tnote\nRadwan\thello, world");}
#[test] fn tsv_to_csv_quotes_commas(){let got=convert_delimited("name\tnote\nRadwan\thello, world",'\t',',').unwrap();assert_eq!(got,"name,note\nRadwan,\"hello, world\"");}
#[test] fn escaped_quotes_round_trip(){let got=convert_delimited("a,\"say \"\"hi\"\"\"",',','\t').unwrap();assert_eq!(got,"a\tsay \"hi\"");}
#[test] fn rejects_unterminated_quote(){assert!(convert_delimited("a,\"broken",',','\t').is_err());}
#[test] fn normalizes_newlines(){assert_eq!(normalize_newlines("a\r\nb\rc\n",Newline::Lf,true),"a\nb\nc\n");assert_eq!(normalize_newlines("a\nb",Newline::CrLf,false),"a\r\nb");}
#[test] fn base64_known_vector(){assert_eq!(base64_encode(b"hello"),"aGVsbG8=");assert_eq!(base64_decode("aGVs\nbG8=").unwrap(),b"hello");}
#[test] fn base64_binary_round_trip(){let bytes=[0,1,2,127,128,254,255];assert_eq!(base64_decode(&base64_encode(&bytes)).unwrap(),bytes);}
#[test] fn rejects_bad_base64(){assert!(base64_decode("abc").is_err());assert!(base64_decode("####").is_err());assert!(base64_decode("ab=c").is_err());}
#[test] fn output_refuses_overwrite_by_default(){let id=SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();let dir=std::env::temp_dir().join(format!("fch-{id}"));fs::create_dir_all(&dir).unwrap();let p=dir.join("out.txt");fs::write(&p,b"old").unwrap();assert!(write_output(&p,b"new",false).is_err());assert_eq!(fs::read(&p).unwrap(),b"old");write_output(&p,b"new",true).unwrap();assert_eq!(fs::read(&p).unwrap(),b"new");let _=fs::remove_dir_all(dir);}
