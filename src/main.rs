use std::fs::File;
use std::io::{self, Lines, BufReader, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::{HashMap, HashSet};

// const regex : Regex = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();

fn main() {
    let pws = read_passports();
    let pws_with_all_fields: Vec<&Passport> = pws.iter().filter(|p| p.has_required_fields()).collect();
    let valid1 = pws_with_all_fields.len();
    let pws_with_valids : Vec<&&Passport> = pws_with_all_fields.iter().filter(|x| x.all_fields_valid()).collect();
    let valid2 = pws_with_valids.len();
    println!("Answer 1 : {}", valid1);
    println!("Answer 2 : {}", valid2);
}

struct Passport {
    fields: HashMap<String, String>
}

impl Passport {
    fn new(lines: Vec<String>) -> Passport {
        let mut fields: HashMap<String, String> = HashMap::new();
        for line in lines {
            println!("Parsing line: {}", line.clone());
            let tokens: Vec<(&str, &str)> = line.as_str().split(' ')
                .map(|x| x.trim())
                .map(|x| Passport::to_pair(x))
                .collect();
            for (key, value) in tokens {
                fields.insert(String::from(key), String::from(value));
            }
        }
        return Passport { fields };
    }
    fn to_pair(s: &str) -> (&str, &str) {
        let s: Vec<&str> = s.split(':').collect();
        (s.get(0).unwrap(), s.get(1).unwrap())
    }

    fn has_required_fields(&self) -> bool {
        let keys: HashSet<&String> = self.fields.keys().collect();
        for key in keys.clone() {
            // println!("KEY: {}", key);
        }
        for key in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter() {
            if !keys.contains(&key.to_string()) {
                println!("Missing: {}", key);
                return false;
            }
        }
        return true;
    }
    fn field_valid(&self, field: &str, validation: &str) -> bool {
        let regex : Regex = Regex::new(validation).unwrap();
        let value = self.fields.get(field).unwrap();
        let is_valid= regex.is_match(value);
        // println!("CHECK {} {} -> {}",field, validation, is_valid);
        is_valid
    }

    fn all_fields_valid(&self) -> bool {
        let byr = self.field_valid("byr",r"^(19[2-9][0-9]|200[0-2])$");
        let ier = self.field_valid("iyr",r"^20(1[0-9]|20)$");
        let eyr = self.field_valid("eyr",r"^20(2[0-9]|30)$");
        let hgt = self.field_valid("hgt",r"^(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)$");
        let hcl = self.field_valid("hcl",r"^#[0-9a-f]{6}$");
        let ecl = self.field_valid("ecl",r"^(amb|blu|brn|gry|grn|oth|hzl)");
        let pid = self.field_valid("pid", r"^[0-9]{9}$");
        return byr && ier && eyr && hgt && hcl && ecl && pid;
    }
}

fn read_passports() -> Vec<Passport> {
    let mut passports: Vec<Passport> = vec!();
    if let Ok(lines) = read_lines("input.txt") {
        let mut pass_lines: Vec<String> = vec!();
        for line in lines {
            if let Ok(s) = line {
                // println!("Processing: {}", s);
                if s.is_empty() {
                    passports.push(Passport::new(pass_lines));
                    pass_lines = vec!();
                } else {
                    pass_lines.push(s);
                }
            }
        }
        passports.push(Passport::new(pass_lines));
    }
    return passports;
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}