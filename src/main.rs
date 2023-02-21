// lib.rs

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rust_crypto::aes::{Aes128, Aes256};
use rust_crypto::blockmodes::{EcbEncryptor, EcbDecryptor};
use rust_crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};

#[pyfunction]
fn aes_encrypt(_py: Python, key: &[u8], input: &[u8]) -> PyResult<PyObject> {
    let mut output = vec![0; input.len()];
    let key = match key.len() {
        16 => Aes128::new(key),
        32 => Aes256::new(key),
        _ => return Err(exceptions::ValueError::py_err("Invalid key length")),
    };
    let mut encryptor = EcbEncryptor::new(key, &mut output, PaddingScheme::NoPadding);
    let mut buffer = ReadBuffer::new(input);
    let mut output_buffer = WriteBuffer::new(&mut output);
    let _ = encryptor.encrypt(&mut buffer, &mut output_buffer, true).unwrap();
    output.truncate(output_buffer.position() as usize);
    Ok(output.into_py(_py))
}

#[pyfunction]
fn aes_decrypt(_py: Python, key: &[u8], input: &[u8]) -> PyResult<PyObject> {
    let mut output = vec![0; input.len()];
    let key = match key.len() {
        16 => Aes128::new(key),
        32 => Aes256::new(key),
        _ => return Err(exceptions::ValueError::py_err("Invalid key length")),
    };
    let mut decryptor = EcbDecryptor::new(key, &mut output, PaddingScheme::NoPadding);
    let mut buffer = ReadBuffer::new(input);
    let mut output_buffer = WriteBuffer::new(&mut output);
    let _ = decryptor.decrypt(&mut buffer, &mut output_buffer, true).unwrap();
    output.truncate(output_buffer.position() as usize);
    Ok(output.into_py(_py))
}

#[pymodule]
fn q365AES(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(aes_encrypt, m)?)?;
    m.add_function(wrap_pyfunction!(aes_decrypt, m)?)?;
    Ok(())
}
