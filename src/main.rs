use std::collections::HashMap;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\nMenu:");
        println!("1. Tambah karyawan ke departemen");
        println!("2. Lihat karyawan dalam departemen");
        println!("3. Lihat semua karyawan di perusahaan");
        println!("4. Keluar");

        let choice = loop {
            let mut input = String::new();
            println!("Masukkan pilihan (1-4):");
            std::io::stdin().read_line(&mut input).unwrap();
            match input.trim().parse::<i32>() {
                Ok(num) if (1..=4).contains(&num) => break num, // Pilihan valid
                Ok(_) => println!("Masukkan angka antara 1 dan 4."),
                Err(_) => println!("Input tidak valid. Masukkan angka!"),
            }
        };

        match choice {
            1 => {
                // Menambahkan karyawan ke departemen
                println!("Masukkan nama karyawan:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                println!("Masukkan nama departemen:");
                let mut department = String::new();
                std::io::stdin().read_line(&mut department).unwrap();
                let department = department.trim().to_string();

                let entry = company.entry(department.clone()).or_insert(Vec::new());
                entry.push(name);
                println!("Karyawan berhasil ditambahkan.");
            }
            2 => {
                // Melihat karyawan dalam departemen
                println!("Masukkan nama departemen:");
                let mut department = String::new();
                std::io::stdin().read_line(&mut department).unwrap();
                let department = department.trim().to_string();

                if let Some(employees) = company.get(&department) {
                    let mut sorted_employees = employees.clone();
                    sorted_employees.sort();
                    println!("Karyawan di departemen {}: ", department);
                    for employee in sorted_employees {
                        println!("{}", employee);
                    }
                } else {
                    println!("Departemen tidak ditemukan.");
                }
            }
            3 => {
                // Melihat semua karyawan di perusahaan
                println!("Daftar semua karyawan di perusahaan:");
                let mut all_employees: Vec<String> = Vec::new();

                for (department, employees) in &company {
                    for employee in employees {
                        all_employees.push(format!("{} - {}", employee, department));
                    }
                }

                all_employees.sort();
                for employee in all_employees {
                    println!("{}", employee);
                }
            }
            4 => {
                // Keluar dari program
                println!("Keluar...");
                break;
            }
            _ => unreachable!(), // Tidak akan terjadi karena validasi di awal loop
        }
    }
}
