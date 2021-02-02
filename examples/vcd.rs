use pronto::ProntoFrame;
use std::fs::File;
use std::io;
use vcd::{SimulationCommand, TimescaleUnit, Value};

fn main() -> io::Result<()> {

    let mut args = std::env::args();
    let _ = args.next();
    let pronto_line = args.next();

    if let Some(pronto_line) = pronto_line {
        let frame = pronto::decode(&pronto_line).unwrap();
        let mut buf = File::create("pronto.vcd")?;
        write_vcd(&frame, &mut buf)?;
    }

    Ok(())
}

fn write_vcd(pframe: &ProntoFrame, w: &mut dyn io::Write) -> io::Result<()> {
    let mut writer = vcd::Writer::new(w);

    let timescale = pframe.carrier_period() as u32;

    println!("timescale: {}", timescale);

    // Write the header
    writer.timescale(timescale, TimescaleUnit::US)?;
    writer.add_module("top")?;
    let data = writer.add_wire(1, "data")?;
    writer.upscope()?;
    writer.enddefinitions()?;

    // Write the initial values
    writer.begin(SimulationCommand::Dumpvars)?;
    writer.change_scalar(data, Value::V0)?;
    writer.end()?;

    let mut t = 0;
    let mut level = false;

    writer.timestamp(0)?;
    // Start with the signal low
    writer.change_scalar(data, false)?;

    for dt in pframe.burst1().iter().chain(pframe.burst2().iter()) {
        level = !level;
        t += u64::from(*dt);

        writer.timestamp(t)?;
        writer.change_scalar(data, level)?;
    }

    Ok(())
}
