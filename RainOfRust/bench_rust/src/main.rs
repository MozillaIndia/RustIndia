//-- #########################
//-- Task: Benchmarking experiments
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 22 May 17
//-- #########################
// Importing external crates

#![feature(test)] 

extern crate test;
extern crate rayon;
use rayon::prelude::*;

// ordinary average function in Rust - Imperative style
pub fn ordavg(list: &[f64]) -> f64{
	let mut total = 0.;
	
	for el in list{
		total += *el
	}

	total/list.len() as f64
}	

// HLL version of the average function in Rust
pub fn hllavg(list: &[f64]) -> f64{

	list.iter().sum::<f64>() / list.len() as f64

}

// parallel version (Rayon)
pub fn rayavg(list:&[f64]) -> f64{

	list.par_iter().sum::<f64>() / list.len() as f64

}

// fold version - Reduce
pub fn foldavg(list: &[f64]) -> f64{

	list.iter().fold(0.,|a, b| a + b) / list.len() as f64

}

// Testing and benchmarking the functions
#[cfg(test)]
mod tests {
	use super::*; 
	use test::Bencher;

	#[test]
	fn check_results() {
		let rand_list = [1.0,1.0,1.0,1.0,1.0,1.0];
		assert_eq!(1.0, ordavg(&rand_list));
		assert_eq!(1.0, hllavg(&rand_list));
		assert_eq!(1.0, rayavg(&rand_list));
		assert_eq!(1.0, foldavg(&rand_list));
	}

	#[bench]
    fn bench_ordavg(b: &mut Bencher) {
        b.iter(|| {
        	let rand_list = [1.0,2.0,3.0,5.0,4.0,6.0];
        	ordavg(&rand_list)
        });
    }

    #[bench]
    fn bench_hllavg(b: &mut Bencher) {
        b.iter(|| {
        	let rand_list = [1.0,2.0,3.0,5.0,4.0,6.0];
        	hllavg(&rand_list)
        } );
    }

    #[bench]
    fn bench_rayavg(b: &mut Bencher) {
        b.iter(|| {
        	let rand_list = [1.0,2.0,3.0,5.0,4.0,6.0];
        	rayavg(&rand_list)
        });
    }

   	#[bench]
    fn bench_foldavg(b: &mut Bencher) {
        b.iter(|| {
        	let rand_list = [1.0,2.0,3.0,5.0,4.0,6.0];
        	foldavg(&rand_list)
        });
    }

}

// Main execution place
fn main() {
	let rand_list = [1.0,1.0,1.0,1.0,1.0,1.0];
	println!("{:?}", ordavg(&rand_list)); 
	println!("{:?}",hllavg(&rand_list));
	println!("{:?}", rayavg(&rand_list));
	println!("{:?}", foldavg(&rand_list));
}