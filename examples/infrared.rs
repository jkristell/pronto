use infrared::protocols::{Denon, Nec, Rc5, Rc6};
use infrared::recv::{BufferReceiver, InfraredReceiver};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {

    let filename = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("data/testdata.pronto".to_string() );

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    println!("CmdNum\tCommand");

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("#") {
            continue;
        }

        let frame = pronto::decode(&line).unwrap();
        let values = frame.iter().copied().collect::<Vec<_>>();
        let samplerate = frame.carrier_frequency().round() as u32;

        let receiver = BufferReceiver::new(&values, samplerate);

        // Run some receivers on the data to see if we can detect any commands
        run_decoders(&receiver);
    }

    Ok(())
}

fn run_decoders(receiver: &BufferReceiver) {
    decode::<Nec>(&receiver);
    decode::<Rc5>(&receiver);
    decode::<Rc6>(&receiver);
    decode::<Denon>(&receiver);
}

fn decode<Protocol>(recv: &BufferReceiver)
where
    Protocol: InfraredReceiver,
    Protocol::Cmd: Debug,
{
    let iter = recv.iter::<Protocol>();

    for (id, cmd) in iter.enumerate() {
        println!("{}\t{:?}", id, cmd);
    }
}
