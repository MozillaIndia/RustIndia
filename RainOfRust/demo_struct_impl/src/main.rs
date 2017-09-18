// Project Name : Employee Directory
// Author : Viki & MozPune community
// Version : 0.0.1
// Email id : dvigneshwer@gmail.com

use std::{i32,f32};

// Complex application specific datatype

#[derive(Debug)]
struct EmployeeDetails {
	emp_id : i32,
	emp_sal : f32,
	emp_name : & 'static str,
	NoOfMembers: i32,
}

#[derive(Debug)]
struct FamilyDetails {
	NoOfMembers: i32,
}

// Adding methods to employee details
impl EmployeeDetails {
	// To check for executive employee
	fn check_executive(&self) -> &str {

		let sal_thres = 500.00;

		let mut executive_status = "true";

		if self.emp_sal >= sal_thres {
			executive_status = "false";
		}

		executive_status
	}
}

trait FamilyMembers {
	fn no_of_members_details(&self) -> i32;
}

impl FamilyMembers for EmployeeDetails {
	fn no_of_members_details(&self) -> i32{
		self.NoOfMembers
	}
}

impl FamilyMembers for FamilyDetails {
	fn no_of_members_details(&self) -> i32{
		self.NoOfMembers
	}
}


// All the execution
fn main() {
	
	// Declared sample variable using the app specific datatype
	let emp_mehul = EmployeeDetails {
		emp_id : 1001,
		emp_sal : 100.500,
		emp_name : "Mehul Patel",
		NoOfMembers : 16,
	};

	let emp_mehul_family = FamilyDetails {
		NoOfMembers : 2,
	};

	// Printing the value inside the employee datatype
	println!("Employee ID {0} & Salary {1} & Name {2}",emp_mehul.emp_id, emp_mehul.emp_sal, emp_mehul.emp_name);

	println!(" FamilyDetails {}", emp_mehul_family.NoOfMembers);

	// Checking if mehul is executive or Not
	println!("Is Mehul an executive ? {}",emp_mehul.check_executive() );

	println!("employee team details for mehul {}",emp_mehul.no_of_members_details() );

	println!("Family member details for mehul {}",emp_mehul_family.no_of_members_details() );

}
