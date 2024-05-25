use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Choose an option:");
        println!("1. Add an employee to a department");
        println!("2. Get a list of all employees in a department");
        println!("3. Get a list of all employees in the company, sorted by department");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read loine");
        let choice = choice.trim();

        match choice {
            "1" => add_employee(&mut company),
            "2" => list_department_employees(&company),
            "3" => list_all_employees(&company),
            "4" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>) {
    println!("Enter the employee name:");
    let mut employee = String::new();
    io::stdin().read_line(&mut employee).expect("Failed to read line");
    let employee = employee.trim().to_string();
    
    println!("Enter the department:");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to read line");
    let department = department.trim().to_string();

    company.entry(department).or_insert(Vec::new()).push(employee);

    println!("Employee added successfully.");
}

fn list_department_employees(company: &HashMap<String, Vec<String>>) {
    println!("Enter the department name:");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to read line");
    let department = department.trim();

    match company.get(department) {
        Some(employees) => {
            let mut employees = employees.clone();
            employees.sort();
            println!("Employees in {} department:", department);
            for employee in employees {
                println!("{}", employee);
            }
        }
        None => println!("Department not found."),
    }
}

fn list_all_employees(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<&String> = company.keys().collect();
    departments.sort();

    for department in departments {
        println!("{} department:", department);
        let mut employees = company.get(department).unwrap().clone();
        employees.sort();
        for employee in employees {
            println!(" - {}", employee);
        }
    }
}
