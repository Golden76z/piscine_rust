mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (&str, &Store) {
    let mut biggest_store: Option<(&str, &Store)> = None;

    for (_, floor) in &mall.floors {
        for (name, store) in &floor.stores {
            match biggest_store {
                None => biggest_store = Some((name, store)),
                Some((_, current_biggest)) => {
                    if store.square_meters > current_biggest.square_meters {
                        biggest_store = Some((name, store));
                    }
                }
            }
        }
    }

    biggest_store.expect("Mall should have at least one store")
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, &Employee)> {
    let mut highest_paid: Vec<(&str, &Employee)> = Vec::new();
    let mut max_salary = 0.0;

    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (name, employee) in &store.employees {
                if employee.salary > max_salary {
                    highest_paid.clear();
                    highest_paid.push((name, employee));
                    max_salary = employee.salary;
                } else if employee.salary == max_salary {
                    highest_paid.push((name, employee));
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut emp_num = 0;

    // Count employees
    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            emp_num += store.employees.len();
        }
    }

    // Add guards
    emp_num += mall.guards.len();

    emp_num
}

pub fn check_for_securities(mall: &mut Mall, pool: HashMap<String, Guard>) {
    let mut total_sqm = 0;

    // Calculate total square meters
    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            total_sqm += store.square_meters;
        }
    }

    // Calculate how many guards are needed
    let guards_needed = (total_sqm + 199) / 200;
    let current_guards = mall.guards.len() as u64;

    // Add guards if needed
    if current_guards < guards_needed {
        let guards_to_add = guards_needed;
        let mut added = 0;

        for (name, guard) in pool {
            if added >= guards_to_add {
                break;
            }
            mall.hire_guard(name, guard);
            added += 1;
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_, floor) in &mut mall.floors {
        for (_, store) in &mut floor.stores {
            for (_, employee) in &mut store.employees {
                let hours_worked = employee.working_hours.1 - employee.working_hours.0;
                if hours_worked >= 10 {
                    employee.raise(employee.salary * 0.1);
                } else {
                    employee.cut(employee.salary * 0.1);
                }
            }
        }
    }
}
