use std::process;
mod subnetter {
    use std::net::Ipv4Addr;
    pub enum IpErrors {
        InvalidInput(u8),
        InvalidCidr(u8),
    }

    struct IpSubnet {
        ip: Ipv4Addr,
        mask: (u8, u8, u8, u8), 
        cidr: u8,
    }

    pub fn new(ip_string: String) -> IpSubnet {

        //Begin by parsing the String into parts.
    }    

    fn build_ipv4_struct(numbers: Vec<u8>) -> Ipv4Addr {
        //Gonna put this in here, because I do not want to have another result type return
        if numbers.len() != 4 {
            panic!("Invalid length was given");
        }
        Ipv4Addr::new(numbers[0], numbers[1], numbers[2], numbers[3])

    }

    fn split_string(ip_string: &String) -> Result<Vec<&str>, IpErrors> {
        //Splititng at the slash notation and if the result is not a len of 2 then we return an error.
        let split: Vec<&str> = ip_string.split('/').collect();
        if split.len() == 2 {
            return Ok(split);
        }
        Err(IpErrors::InvalidInput(1))
    }

    fn convert_string_into_tuple(ipv4: &String) -> Result<Vec<u8>, IpErrors>{
        let vec_split: Vec<&str> = ipv4.split('.').collect();
        if vec_split.len() != 4 {
            return Err(IpErrors::InvalidInput(2));
        }

        let mut vec_format: Vec<u8> = Vec::new();

        for item in vec_split {
            let val = item.parse::<u8>();

            match val {
                Err(e) => {
                    return Err(IpErrors::InvalidInput(3));
                },
                Ok(num) => {
                    vec_format.push(num);
                },
            }
        }
        Ok(vec_format)
    }

    fn valid_cidr(cidr: &str) -> Result<u8, IpErrors> {
        match cidr.parse::<u8>() {
            Err(e) => return Err(IpErrors::InvalidCidr(1)),
            Ok(num) => Ok(num)
        }

    }

    fn calc_subnet(cidr: u8) -> Vec<u8> {
        let mut mask: Vec<u8> = Vec::new();
        let filled_count = cidr / 8;
        for n in 0..filled_count {
            mask.push(255);
        }
        //get the remainder
        let remainder = cidr % 8;

        //now we need to calculate the number for this value
        let mut total: u8 = 0;
        while remainder >= 0 {
            total += 2u8.pow(remainder as u32);
        }
        mask.push(total);

        while mask.len() < 4  {
            mask.push(0);
        }
        mask

    }




    //This section will be here to handle errors, because I am too dumb
    //To figure out how to split this more logically





    fn handle_invalid_input(error: IpErrors) {
        match error {
            IpErrors::InvalidInput(1) => {
                println!("Could not find the split between the IP and the CIDR notation");
                std::process::exit(1);
            },
            IpErrors::InvalidInput(2) => {
                println!("Invalid IP format");
                std::process::exit(2);
            },
            IpErrors::InvalidInput(3) => {
                println!("Invalid IP range");
                std::process::exit(3);
            },
            _ => {
                panic!("Invalid CIDR is not covered in this function");
            }
        }
    }

    

}

mod tests {
    use super::*;
    #[test]
    //Checks if it splits and checks properly.
    fn proper_split() {
        match subnetter::split_string(&String::from("127.0.0.1/27")) {
            Ok(num) => assert_eq!(1, 1),
            Err(e) => assert_eq!(1, 2),
        }
    }

    #[test]
    //Test to make sure that the subnetter can split an IP into a proper string
    fn check_ipstring_to_tuple() {
        assert_eq!(convert_string_into_tuple(&"127.0.0.1"), (127,0,0,0));
    }
}