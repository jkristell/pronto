use std::num::ParseIntError;

const PRONTO_CLOCK_SCALE_FACTOR: f64 = 0.241246;

#[derive(Debug)]
pub struct ProntoFrame {
    pronto_freq: u16,
    burst1: Vec<u16>,
    burst2: Vec<u16>,
}

impl ProntoFrame {
    /// Returns the carrier frequency in Hz
    pub fn carrier_frequency(&self) -> f64 {
        1e6 / self.carrier_period()
    }

    /// Return the carrier period time in micro seconds (us)
    pub fn carrier_period(&self) -> f64 {
        f64::from(self.pronto_freq) * PRONTO_CLOCK_SCALE_FACTOR
    }

    /// Burst data 1
    ///
    /// The timing is in relation to the carrier_frequency of the frame. Multiply it by the
    /// carrier period to get the representation in micro seconds (us)
    pub fn burst1(&self) -> &[u16] {
        &self.burst1
    }

    /// Burst data 2
    ///
    /// The timing is in relation to the carrier_frequency of the frame. Multiply it by the
    /// carrier period to get the representation in micro seconds (us)
    pub fn burst2(&self) -> &[u16] {
        &self.burst2
    }

    /// Return an iterator over both bursts
    pub fn iter(&self) -> impl Iterator<Item = &u16> {
        // Add an initial zero to the pulses
        std::iter::once(&0u16)
            .chain(self.burst1())
            .chain(self.burst2())
    }
}

pub fn decode(s: &str) -> Result<ProntoFrame, Error> {
    let values = s
        .split_whitespace()
        .map(|group| u16::from_str_radix(group, 16))
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    if values.len() < 4 {
        return Err(Error::DataError("Not enough elements in input".into()));
    }

    if values[0] != 0 {
        return Err(Error::DataError("Pronto type not zero".into()));
    }

    let pronto_freq = values[1];
    // The length is for the burst pairs. So multiply it by 2
    let burst1_len = usize::from(values[2]) * 2;
    let burst2_len = usize::from(values[3]) * 2;

    let burst1_end = 4 + burst1_len;
    if burst1_end > values.len() {
        return Err(Error::DataError("Wrong len for burst 1".into()));
    }
    let burst1 = values[4..burst1_end].to_vec();
    let burst2 = values[burst1_end..].to_vec();

    if burst2.len() != burst2_len {
        return Err(Error::DataError(format!(
            "Burst 2 length is invalid, expexted: {} got {}",
            burst2_len,
            burst2.len()
        )));
    }

    Ok(ProntoFrame {
        pronto_freq,
        burst1,
        burst2,
    })
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse value as hex")]
    ParseError(#[from] ParseIntError),

    #[error("Input data error")]
    DataError(String),
}

