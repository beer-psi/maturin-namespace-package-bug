use audiopus::{coder::Decoder, Channels, SampleRate};
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn create_opus_decoder() {
    let _ = Decoder::new(SampleRate::Hz48000, Channels::Stereo);
}

/// A Python module implemented in Rust.
#[pymodule(name = "_native")]
fn namespace_package_reproduction(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_opus_decoder, m)?)?;
    Ok(())
}
