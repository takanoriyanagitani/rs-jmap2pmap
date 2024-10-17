use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use std::process::ExitCode;

use serde_json::Map;
use serde_json::Value;

use prost::Message;
use prost_types::Struct;

fn rdr2jmap2pmap2wtr<R, W>(rdr: R, wtr: &mut W) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    let m: Map<String, Value> = serde_json::from_reader(rdr)?;
    let s: Struct = rs_jmap2pmap::jmap2pmap::jmap2pmap(m);
    let serialized: Vec<u8> = s.encode_to_vec();
    wtr.write_all(&serialized)?;
    wtr.flush()
}

fn stdin2stdout() -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let br = BufReader::new(il);

    let o = io::stdout();
    let mut ol = o.lock();

    {
        let mut bw = BufWriter::new(ol.by_ref());
        rdr2jmap2pmap2wtr(br, &mut bw)?;
        bw.flush()?;
    }
    ol.flush()?;

    Ok(())
}

fn main() -> ExitCode {
    stdin2stdout()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
