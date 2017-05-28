//-- #########################
//-- Task: Creation python modules in Rust
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 22 May 17
//-- #########################

#[macro_use] extern crate cpython;

use cpython::{PyString, Python, PyResult};

// Rust function to find the fibonacci series
fn fibo(py: Python, n : u64) -> PyResult<u64> {
    if n < 2 {
        return Ok(1)
    }
    let mut prev1 = 1;
    let mut prev2 = 1;
    for _ in 1..n {
        let new = prev1 + prev2;
        prev2 = prev1;
        prev1 = new;
    }
    Ok(prev1) 
}

// To build a Python compatible module we need an intialiser which expose the public interface
py_module_initializer!(example, initexample, PyInit_example, |py, m| {
    // Expose our function fibo as `extern "C"`
    try!(m.add(py, "fibo", py_fn!(py, fibo(rand_int: u64))));
    // Initialiser macro needs a Result<> as return value
    Ok(())
});
