use pyo3::prelude::*;

#[pyfunction]
fn encrypt(data: &str, shift: u8) -> String {
    let mut result = String::new();

    for c in data.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            result.push((((c as u8 - base + shift) % 26) + base) as char);
        } else {
            result.push(c);
        }
    }

    result
}

#[pyfunction]
fn decrypt(data: &str, shift: u8) -> String {
    encrypt(data, 26 - shift)
}

#[pymodule]
fn helloq365(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encrypt, m)?)?;
    m.add_function(wrap_pyfunction!(decrypt, m)?)?;

    Ok(())
}
