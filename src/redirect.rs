use std::io::{BufReader, BufRead, BufWriter, Write};
use std::process::{Command, Stdio};
use std::net::{TcpStream, Shutdown};

/*fn repl_str(old_str: String) -> String{
	let mut new_str = old_str.replace("\n","<br/>");
}*/

pub fn redirect_io(proc_name: String, mut stream: TcpStream) {
    let mut proc = Command::new(proc_name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut outobj = BufReader::new(proc.stdout.take().unwrap());
    let mut errobj = BufReader::new(proc.stderr.take().unwrap());
    let mut inobj = BufWriter::new(proc.stdin.take().unwrap());
    let mut outbuf = String::new();
    let mut errbuf = String::new();
	inobj.write_all(b"uname -a").unwrap();
    // Read from stdout
    while let Ok(bytes_read) = outobj.read_line(&mut outbuf) {
        if bytes_read == 0 {
            break;
        }
        stream.write_all(outbuf.replace("\n","<br/>").as_bytes()).unwrap();
        outbuf.clear();
    }

    // Read from stderr
    while let Ok(bytes_read) = errobj.read_line(&mut errbuf) {
        if bytes_read == 0 {
            break;
        }
        stream.write_all(errbuf.replace("\n","<br/>").as_bytes()).unwrap();
        errbuf.clear();
    }
    stream.write_all(b"1</code>").unwrap();
    //stream.shutdown(Shutdown::Both).expect("Failed to shutdown stream");
}
