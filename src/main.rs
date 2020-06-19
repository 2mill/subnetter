use std::env;
use std::process;
use std::net::{IpAddr, Ipv4Addr};
use std::result::Result;



/// TODO : Port everything into a lib.rs, test cases, and better formatting along with Display trait
/// Also need a better error handler.

//Clean up config to be more modular
//Also make it so that the submask can be better displayed
//Calculate Range for a given address.
fn main() {

    //LOGIC

    //Take the IP, and CIDR and evalute that the input is valid.
    //Calculate the Mask
    //populate mask_array
    //Start off with checking for a valid IP

    let args: Vec<String> = env::args().collect();
    let IpSub: IpSubnetter = IpSubnetter::new(&args[1]);
    println!("{:?}", IpSub.subnet_long);
}
enum IpErrors {
    InvalidLength,
    InvalidIp,
    InvalidInput,
    NoCIDR,
}
struct IpSubnetter{
    ip: IpAddr,
    subnet:u8,
    subnet_long: (u8, u8, u8, u8),
}

impl IpSubnetter {
    pub fn new(ip_cidr: &String) -> IpSubnetter {
        let mut ip_vec: Vec<u8> = Vec::new();
        let mut total = String::new();
        //Splits the IP from the subnet.
        let ip_cidr_split = match IpSubnetter::ip_cidr_split(ip_cidr) {
            Err(e) => match e {
                IpErrors::NoCIDR => panic!("No CIDR notation"),
                _ => panic!("Something else went wrong in CIDR error handling"),
            },
            Ok (s)=> s,
        };
        IpSubnetter {
            ip: match IpSubnetter::check_and_return_u8_tuple(IpSubnetter::vec_split(ip_cidr_split.0)) {
                Err(e) => {
                    println!("There was an error");
                    match e {
                        InvalidIp => {
                            panic!("Invalid IP given!");
                        },
                        _ => panic!("Something else went wrong)"),
                    }
                },
                Ok(ip) => IpAddr::V4(Ipv4Addr::new(ip.0, ip.1, ip.2, ip.3)),
            },
            subnet: ip_cidr_split.1.parse::<u8>().unwrap(),
            subnet_long: IpSubnetter::calc_subnet_long(ip_cidr_split.1),
        }
    }

    //This function is core, please don't delete
    fn vec_split(s: &str) -> Vec<&str>{
        s.split('.').collect()
    }
    
    // fn ip_builder(ip_cidr: &String) -> IpAddr {
    //     let vals: Vec<&str> = ip_cidr.split('.').collect();
    //     println!("{:?}", &vals);
    //     if vals.len() != 4  {
    //         panic!("Invalid IP"),
    //     }

    //     //Check if the Ips are valid
    //     IpAddr::V4(Ipv4Addr::new(127,0,0,8))

    //     //Placeholder
    // }

    fn check_and_return_u8_tuple (vec: Vec<&str>) -> Result<(u8, u8, u8, u8), IpErrors>{
        if vec.len() != 4 {
            return Err(IpErrors::InvalidLength);
        }
        let mut u8_vec: Vec<u8> = Vec::new();

        //Fix this tomorrow
        for s in vec {
            u8_vec.push(match s.parse::<u8>() {
                Err(e) => return Err(IpErrors::InvalidIp),
                Ok(num) => num,
            });
        }
        println!("{:?}", u8_vec);
        //This should be the IP split into a tuple at this point
        Ok((u8_vec[0],u8_vec[1],u8_vec[2],u8_vec[3]))
    }

    fn calc_subnet_long(cidr_string: &str) -> (u8, u8, u8, u8) {
        println!("{}", cidr_string);
        let num = cidr_string.parse::<u8>().unwrap();
        let filled_octets = num / 8;
        let remaining_bits_number = IpSubnetter::get_subnet_from_remaining_bits(num % 8);
        let tuple_end: (u8, u8, u8, u8); 
        let octet: (u8, u8, u8, u8) = match filled_octets {
            1 => (255, remaining_bits_number, 0, 0),
            2 => (255, 255, remaining_bits_number, 0),
            3 => (255, 255, 255, remaining_bits_number),

            //This would be a broadcast address, and should not be allowed from earlier code
            4 => (255, 255, 255, 255),

            //This should never happen
            _ => (0, 0, 0, 0),
        };
        octet
        //Mask calculations
    }

    fn ip_cidr_split(length: &String) -> Result<(&str, &str), IpErrors> {
        let temp_vec: Vec<&str> = length.split('/').collect();
        if temp_vec.len() != 2 {
            return Err(IpErrors::NoCIDR);
        }

        Ok((temp_vec[0], temp_vec[1]))

        

    }

    //Don't fuck with this because it works, but could use some refacotring in the future.
    fn get_subnet_from_remaining_bits(num: u8) -> u8 {
        let mut total: u8 = 0;
        let two: u8 = 2;
        for x in 8-num..8 {

            total += 2u8.pow(x as u32);
        }
        total
    }
}

