#![allow(clippy::upper_case_acronyms)]

use std::u128;
use gethostname::gethostname;
use pyo3::prelude::*;
use radix_fmt::radix_36;
use rand::{distributions::Alphanumeric, Rng};

fn string_radix_36(string: String, length: usize) -> String {
    format!(
        "{:0>length$}",
        format!(
            "{}",
            radix_36(string.into_bytes().iter().map(|&b| b as u16).sum::<u16>())
        )
        .to_lowercase()
    )
}

fn timestamp_radix_36(timestamp: u128) -> (String, String) {
    let timestamp_string = format!("{}", radix_36(timestamp as u64)).to_lowercase();
    let part_1 = timestamp_string[0..8].to_string();
    let part_2 = timestamp_string[8..12].to_string();
    (part_1, part_2)
}

#[pyclass(frozen, module = "uuidt", get_all, freelist = 10)]
struct UUIDT {
    namespace: String,
    timestamp: u128,
    hostname: String,
    random_chars: String,
}

#[pymethods]
impl UUIDT {
    #[new]
    fn new() -> PyResult<Self> {
        Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
            "Cannot instantiate UUIDT directly. Use uuidt() instead.",
        ))
    }

    fn __str__(&self) -> PyResult<String> {
        let (timestamp_part_1, timestamp_part_2) = timestamp_radix_36(self.timestamp);
        let namespace_radix_36 = string_radix_36(self.namespace.clone(), 4);
        let hostname_radix_36 = string_radix_36(self.hostname.clone(), 4);

        Ok(format!(
            "{}-{}-{}-{}-{}",
            timestamp_part_1,
            timestamp_part_2,
            namespace_radix_36,
            hostname_radix_36,
            self.random_chars
        ))
    }
}

/// Creates a new UUIDT object.
#[pyfunction]
fn new(namespace: String) -> PyResult<UUIDT> {
    if namespace.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Namespace cannot be empty.",
        ));
    }

    let hostname = gethostname().into_string().unwrap();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let random_chars = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect::<String>()
        .to_lowercase();

    Ok(UUIDT {
        namespace,
        timestamp,
        hostname,
        random_chars,
    })
}

/// Extract the timestamp from a UUIDT string.
#[pyfunction]
fn extract_timestamp(uuidt: String) -> PyResult<u128> {
    if uuidt.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "UUIDT cannot be empty.",
        ));
    }

    let uuidt_parts: Vec<&str> = uuidt.split('-').collect();
    if uuidt_parts.len() != 5 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Invalid UUIDT.",
        ));
    }

    let timestamp_part_1 = uuidt_parts[0];
    let timestamp_part_2 = uuidt_parts[1];

    let timestamp = format!("{}{}", timestamp_part_1, timestamp_part_2);
    match u128::from_str_radix(&timestamp, 36) {
        Ok(timestamp) => Ok(timestamp),
        Err(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Invalid UUIDT.",
        )),
    }
}

/// Timestamp-orderable UUIDs for Python, written in Rust.
#[pymodule]
fn uuidt(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(new, m)?)?;
    m.add_function(wrap_pyfunction!(extract_timestamp, m)?)?;
    m.add_class::<UUIDT>()?;
    Ok(())
}
