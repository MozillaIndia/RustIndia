
struct Employee {
	emp_id : i32,
	emp_sal : i32,
	emp_name : String,
}

struct FamilyDetails {
	fam_no : i32,
}


trait PrintDetails {
	fn print_details(&self);
}

impl PrintDetails for Employee {
	fn print_details(&self) {
		println!("Sal: {0}, Id: {1}, Name: {2}",self.emp_sal,self.emp_id,self.emp_name );
	}
}

impl PrintDetails for FamilyDetails {
	fn print_details(&self) {
		println!("Family Member no: {}",self.fam_no);
	}
}

fn custom_print<T: PrintDetails>(entity: T) {
    println!("{}", entity.print_details());
}

fn main() {

    let empviki = Employee{
		emp_id : 1,
		emp_sal : 100000,
		emp_name : "Viki".to_string(),
    };

    empviki.print_details();

    let famviki = FamilyDetails{
		fam_no : 4,
    };

    famviki.print_details();

    custom_print(empviki);
    custom_print(famviki);
}
