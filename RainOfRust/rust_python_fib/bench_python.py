#-- #########################
#-- Task: Benchmarking experiments
#-- Author: Vigneshwer.D
#-- Version: 1.0.0
#-- Date: 22 May 17
#-- #########################

# Importing modules
import example

# fibonacci implementation in Rust
def rust_fibo(val):
	return example.fibo(val)

# fibonacci implementation in Python
def py_fibo(n):
  if n < 2:
    return 1

  prev1 = 1
  prev2 = 1

  for i in range(1, n):
    next = prev1 + prev2
    prev2 = prev1
    prev1 = next
    
  return prev1

val =50

# benchmarking rust_fibo
def test_rust_fibo(benchmark):
	benchmark(rust_fibo, val)

# benchmarking py_fibo
def test_py_fibo(benchmark):
	benchmark(py_fibo, val)


if __name__ == '__main__':

	# Series value
	val =50

	# Calling rust version
	rust_val = rust_fibo(val)
	print "Value predicted by rust_fibo: ", rust_val

	# Calling python version
	py_val = py_fibo(val)
	print "Value predicted by py_fibo: ", py_val

