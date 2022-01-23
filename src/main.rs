use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

fn find_tag<L: AsRef<[u8]>, R: AsRef<[u8]>>(lhs: &L, rhs: &R) -> bool {
    let lhs = lhs.as_ref();
    let rhs = rhs.as_ref();
    lhs.windows(rhs.len()).any(|bytes| bytes == rhs)
}

fn parse_serial_output<W: Write, R: Read>(output: &mut W, mut reader: R, tag: &str) -> Result<()> {
    let mut line: Vec<u8> = Vec::new();

    loop {
        let mut buf = [0; 1024];
        let n = reader.read(&mut buf)?;

        output.write_all(&buf[..n])?;
        output.flush()?;

        let lines: Vec<_> = buf[..n].split(|b| *b == b'\n').collect();
        for (i, bytes) in lines.iter().enumerate() {
            line.extend_from_slice(bytes);
            if find_tag(&line, &tag) {
                return Ok(());
            }
            if i != lines.len() - 1 || bytes.is_empty() {
                line.clear();
            }
        }
    }
}

fn handle_stream(stream: UnixStream, tag: &str) -> Result<()> {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    parse_serial_output(&mut stdout, stream, tag)?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let program = &args[0].clone();
    let brief = format!("Usage: {} -s SERIAL -t TAG", program);

    let mut opts = getopts::Options::new();
    opts.reqopt("s", "serial", "serial port", "SERIAL");
    opts.reqopt("t", "tag", "exit on finding tag", "TAG");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            anyhow::bail!("{}\n{}", e, opts.usage(&brief));
        }
    };

    // unwrap safe because serial/tag are required
    let spath = &matches.opt_str("s").unwrap();
    let tag = &matches.opt_str("t").unwrap();
    let stream = UnixListener::bind(&spath).context("failed to create serial port")?;

    if let Some(stream) = stream.incoming().next() {
        let stream = stream?;
        handle_stream(stream, tag)?;
    }

    Ok(())
}
