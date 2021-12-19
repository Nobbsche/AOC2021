// 18.12.2021 16:10
use itertools::Itertools;
use std::fs;
use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct Package {
    version : u64,
    package : u64,
    data : u64,
    sub_packages : Option<Vec<Package>>,
}

impl Package {

    pub fn get_versions_add(&self) -> u64 {
        let mut sum = self.version;
        if let Some(packages) = &self.sub_packages {
            for pack in packages {
                sum += pack.get_versions_add();
            }
        }
        sum
    }

    pub fn get_package_result (&self) -> u64 {
        if self.sub_packages.is_none() {
            return self.data
        }

        let result = match self.package {
            0 => { let mut sum = 0;
                   for package in self.sub_packages.as_ref().unwrap() {
                        sum += package.get_package_result();
                   } 
                   sum 
            },
            1 => { let mut mul = 1;
                    for package in self.sub_packages.as_ref().unwrap() {
                        mul *= package.get_package_result();
                    }
                    mul 
            },
            2 => {  let mut minimum = u64::MAX; 
                    for package in self.sub_packages.as_ref().unwrap() {
                        let result = package.get_package_result();
                        if  result < minimum { minimum = result }
                    }
                    minimum  
            },
            3 => {  let mut maximum = u64::MIN; 
                    for package in self.sub_packages.as_ref().unwrap() {
                        let result = package.get_package_result();
                        if  result > maximum { maximum = result }
                    }
                    maximum 
            },
            5 => {  let mut result = 0; 
                    let pack1 = self.sub_packages.as_ref().unwrap()[0].get_package_result();
                    let pack2 = self.sub_packages.as_ref().unwrap()[1].get_package_result();
                    if pack1 > pack2 { 
                        result = 1;
                    }
                    result
            },
            6 => {  let mut result = 0; 
                let pack1 = self.sub_packages.as_ref().unwrap()[0].get_package_result();
                let pack2 = self.sub_packages.as_ref().unwrap()[1].get_package_result();
                if pack1 < pack2 { 
                    result = 1;
                }
                result
            },
            7 => {  let mut result = 0; 
                let pack1 = self.sub_packages.as_ref().unwrap()[0].get_package_result();
                let pack2 = self.sub_packages.as_ref().unwrap()[1].get_package_result();
                if pack1 == pack2 { 
                    result = 1;
                }
                result
            },
            _ => 0
        };
        println!("result: {:?}", result);
        result
    }
}

pub fn get_message_result ( mut input : Vec<u8> ) -> u64 {
    let mut packages = vec![];
    while input.len() >= 11 {
        packages.push(create_package(&mut input));
    }
 
    println!("packages: {:?}", packages);

    let mut result = 0;

    for package in packages {
        result = package.get_package_result();
    }

    result
}

pub fn create_package ( input: &mut Vec<u8> ) -> Package {
    let mut sub_packages = None;
    let version = binary_to_number( input.drain(0..3).collect() );
    let package = binary_to_number( input.drain(0..3).collect() );

    if package == 4 {
        let pack = Package { version : version, package : package, data : read_id_4(input), sub_packages : None };
        return pack;
    }

    if input.remove(0) == 0 {
        sub_packages = Some(read_operator_0 ( input ));
    } else {
        let mut sub_package = vec![];
        let packages_to_read = binary_to_number( input.drain(0..11).collect() );
        for _i in 0..packages_to_read {
            sub_package.push(create_package(input));
        }
        sub_packages = Some(sub_package);
    }

    let pack = Package { version : version, package : package, sub_packages : sub_packages, data : 0 };
    pack
}

pub fn get_version_sum ( mut input : Vec<u8> ) -> u64 {

    let mut packages = vec![];
    while input.len() >= 11 {
        packages.push(create_package(&mut input));
    }

    let mut sum = 0;
    for package in packages {
        sum += package.get_versions_add();
    }
    sum
}

pub fn read_operator_0 ( input: &mut Vec<u8> ) -> Vec<Package> {

    let mut sub_packages = vec![];
    let bits_to_read = binary_to_number( input.drain(0..15).collect() );
    let mut data_bits : Vec<u8> = input.drain(0..bits_to_read as usize).collect();
    while !data_bits.is_empty() {
        sub_packages.push(create_package(&mut data_bits));
    }
    sub_packages

}

pub fn read_id_4 ( input: &mut Vec<u8> ) -> u64 {
    let mut data_vec = vec![];
    let mut read_more = true;
    while read_more { 
        let mut data_bits : Vec<u8> = input.drain(0..5).collect();
        if data_bits.remove(0) == 0 {
            read_more = false;
        }
        data_vec.append(&mut data_bits);    
    }
    binary_to_number(data_vec)
}

pub fn binary_to_number ( mut input : Vec<u8> ) -> u64 {
    let mut counter = 0;
    let mut number = 0;
    println!("number length: {:?}", input.len());
    while !input.is_empty() {
        let i = input.pop().unwrap();
        number += i as i64 * 2_i64.pow(counter);
        counter += 1;
    }
    number as u64
}

fn read_input_to_vector ( input_path: &str ) -> Vec<char> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { input.chars().collect() }
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
}

fn convert_hex( msg : Vec<char> ) -> Vec<u8> {

    let mut bin = vec![];

    msg.iter().foreach(|c| { match c {
        '0' => bin.append(&mut vec![0,0,0,0]),
        '1' => bin.append(&mut vec![0,0,0,1]),
        '2' => bin.append(&mut vec![0,0,1,0]),
        '3' => bin.append(&mut vec![0,0,1,1]),
        '4' => bin.append(&mut vec![0,1,0,0]),
        '5' => bin.append(&mut vec![0,1,0,1]),
        '6' => bin.append(&mut vec![0,1,1,0]),
        '7' => bin.append(&mut vec![0,1,1,1]),
        '8' => bin.append(&mut vec![1,0,0,0]),
        '9' => bin.append(&mut vec![1,0,0,1]),
        'A' => bin.append(&mut vec![1,0,1,0]),
        'B' => bin.append(&mut vec![1,0,1,1]),
        'C' => bin.append(&mut vec![1,1,0,0]),
        'D' => bin.append(&mut vec![1,1,0,1]),
        'E' => bin.append(&mut vec![1,1,1,0]),
        'F' => bin.append(&mut vec![1,1,1,1]),
        _   => {},
    } } );

    bin
}

#[cfg(test)]
mod tests {
    use super::*;

    static PATH : &str = "src/day16/input_day16.txt";

    #[test]
    fn read_path() {
        let char_vec = read_input_to_vector(PATH);
        assert_eq!(char_vec.is_empty(), false); 
    }

    #[test]
    fn read_hex_to_bin_test() {
        let msg = vec!['D','2','F','E','2','8'];
        assert_eq!(convert_hex(msg), vec![1,1,0,1,0,0,1,0,1,1,1,1,1,1,1,0,0,0,1,0,1,0,0,0])
    }

    #[test]
    fn binary_to_numer_test() {
        assert_eq!(binary_to_number(vec![1,1,0]), 6);
    }

    #[test]
    fn read_package() {
        let msg = vec!['D','2','F','E','2','8'];
        let mut bin = convert_hex(msg);
        let convert = create_package( &mut bin );
        assert_eq!(convert.version, 6 );
        assert_eq!(convert.package, 4 );
        assert_eq!(convert.data, 2021);
    }

    #[test]
    fn read_with_zero_operator() {
        let msg = vec!['3','8','0','0','6','F','4','5','2','9','1','2','0','0'];
        let mut bin = convert_hex(msg);
        let convert = create_package( &mut bin );
        assert_eq!(convert.version, 1 );
        assert_eq!(convert.package, 6 );
    }

    #[test]
    fn read_with_one_operator() {
        let msg = vec!['E','E','0','0','D','4','0','C','8','2','3','0','6','0'];
        let mut bin = convert_hex(msg);
        let convert = create_package( &mut bin );
        assert_eq!(convert.version, 7 );
        assert_eq!(convert.package, 3 );
    }

    #[test]
    fn version_sum_test() {
        let msg = vec!['8','A','0','0','4','A','8','0','1','A','8','0','0','2','F','4','7','8'];
        let bin = convert_hex(msg);
        assert_eq!(get_version_sum ( bin ), 16);
    }

    #[test]
    fn version_sum_test_1() {
        let msg = vec!['6','2','0','0','8','0','0','0','1','6','1','1','5','6','2','C','8','8','0','2','1','1','8','E','3','4'];
        let bin = convert_hex(msg);
        assert_eq!(get_version_sum ( bin ), 12);
    }

    #[test]
    fn version_sum_test_2() {
        let msg = vec!['C','0','0','1','5','0','0','0','0','1','6','1','1','5','A','2','E','0','8','0','2','F','1','8','2','3','4','0'];
        let bin = convert_hex(msg);
        assert_eq!(get_version_sum ( bin ), 23);
    }

    #[test]
    fn version_sum_test_3() {
        let msg = vec!['A','0','0','1','6','C','8','8','0','1','6','2','0','1','7','C','3','6','8','6','B','1','8','A','3','D','4','7','8','0'];
        let bin = convert_hex(msg);
        assert_eq!(get_version_sum ( bin ), 31);
    }

    #[test]
    fn sum_operator_test () {
        let str_input = "C200B40A82";
        let bin = convert_hex( str_input.chars().collect() );
        assert_eq!( get_message_result( bin ), 3 );
    }

    #[test]
    fn mul_operator_test () {
        let str_input = "04005AC33890";
        let bin = convert_hex( str_input.chars().collect() );
        assert_eq!( get_message_result( bin ), 54 );
    }

    #[test]
    fn minimum_operator_test () {
        let str_input = "880086C3E88112";
        let bin = convert_hex( str_input.chars().collect() );
        assert_eq!( get_message_result( bin ), 7 );
    }

    #[test]
    fn maximum_operator_test () {
        let str_input = "CE00C43D881120";
        let bin = convert_hex( str_input.chars().collect() );
        assert_eq!( get_message_result( bin ), 9 );
    }

    #[test]
    fn ge_operator_test () {
        let str_input = "D8005AC2A8F0";
        let bin = convert_hex( str_input.chars().collect() );
        assert_eq!( get_message_result( bin ), 1 );

        let str_input = "F600BC2D8F";
        let bin = convert_hex( str_input.chars().collect() );
        assert_eq!( get_message_result( bin ), 0 );

        let str_input = "9C005AC2F8F0";
        let bin = convert_hex( str_input.chars().collect() );
        assert_eq!( get_message_result( bin ), 0 );
    }

    #[test]
    fn complex_test () {
        let str_input = "9C0141080250320F1802104A08";
        let bin = convert_hex( str_input.chars().collect() );
        assert_eq!( get_message_result( bin ), 1 );
    }

    #[test]
    fn riddle_1() {
        let char_vec = read_input_to_vector(PATH);
        let bin = convert_hex(char_vec);
        assert_eq!(get_version_sum ( bin ), 953);
    }

    #[test]
    fn riddle_2() {
        let char_vec = read_input_to_vector(PATH);
        let bin = convert_hex(char_vec);
        assert_eq!( get_message_result( bin ), 246225449979 );
    }
}