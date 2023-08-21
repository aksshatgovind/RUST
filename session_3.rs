use std::io;

struct Employee {
    name: String,
    id: u64,
    email: String,
    age: u8,
    phone_number: String,
}

fn main() {
    let mut employees = Vec::new();

    println!("Enter the employee name: ");
    let name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter the employee ID: ");
    let id = io::stdin().read_line(&mut String::new()).unwrap().trim().parse().unwrap();

    println!("Enter the employee email: ");
    let email = String::new();
    io::stdin().read_line(&mut email).unwrap();

    println!("Enter the employee age: ");
    let age = io::stdin().read_line(&mut String::new()).unwrap().trim().parse().unwrap();

    println!("Enter the employee phone number: ");
    let phone_number = String::new();
    io::stdin().read_line(&mut phone_number).unwrap();

    let employee = Employee {
        name,
        id,
        email,
        age,
        phone_number,
    };
    employees.push(employee);

    println!("{:#?}", employees);

    println!("Enter the employee ID to get the details: ");
    let id = io::stdin().read_line(&mut String::new()).unwrap().trim().parse().unwrap();
    let employee = get_employee_by_id(employees, id);
    if employee.is_some() {
        println!("{:#?}", employee.unwrap());
    } else {
        println!("No employee found with ID {}", id);
    }

    println!("Enter the employee age to get all employees with the same age: ");
    let age = io::stdin().read_line(&mut String::new()).unwrap().trim().parse().unwrap();
    let employees_with_same_age = get_employees_by_age(employees, age);
    println!("{:#?}", employees_with_same_age);
}

fn get_employee_by_id(employees: &Vec<Employee>, id: u64) -> Option<&Employee> {
    for employee in employees {
        if employee.id == id {
            return Some(employee);
        }
    }
    return None;
}

fn get_employees_by_age(employees: &Vec<Employee>, age: u8) -> Vec<&Employee> {
    let mut employees_with_same_age = Vec::new();
    for employee in employees {
        if employee.age == age {
            employees_with_same_age.push(employee);
        }
    }
    return employees_with_same_age;
}
