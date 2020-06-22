mod subnetter {
    use std::net::{IpAddr, Ipv4Addr};
    enum IpErrors {
        InvalidLength,
        InvalidIp,
        InvalidInput,
        NoCIDR,
    }
    struct IpSubnet {
        ip: IpAddr,
        cidr_notation: u8,
        mask: (u8, u8, u8, u8),
        broadcast: (u8, u8, u8, u8),
        network_address: (u8, u8, u8, u8),
        block_size = u32,
    }

    impl IpSubnet {
        pub fn new(ip_with_cidr: &String) -> IpSubnet {
        }
    }

    //This function will triage most of the operations for the struct.
    //This is done so I don't have to run through processes over and over
    //again and take care of it very quickly.
    //This will also arm out into error handlers
    fn triage(s: &String) {

    }

    //This function will help with translating into the 
    //Error handler
    fn compile_into_ipaddr_struct(s: &String) -> IpAddr {
        let first_split_vec = match split_ip_cidr_string(s) {
            Err(e) => panic!("No CIDR given"),
            Ok(hahaha) => hahaha,
        }
        //At this point we will have a vec with the first index being
        //The IP as a &str and the second element being the CIDR notation
        
        //This string would be the IP
        //We don't need to care about the CIDR at this point.
        let ip_string = first_split_vec[0];
        let ip_tuple = 

    }
    fn split_ip_cidr_string(s: &String) -> Result<Vec<&str>, IpErrors>{
        let cidr_split: Vec<&str> = s.split('/').collect();
        if cidr_split.len() == 2 {
            Ok(cidr_split)
        } else {
            Err(IpErrors::NoCIDR)
        }
    }

    //Fix this tomorrow
    fn get_tuple_ip_from_string(s: &String) -> Result<(u8, u8, u8, u8), IpErrors> {
        let vec_split = split_dots_into_vec(s);
        let mut vec_format: Vec<u8> = Vec::new();
        if vec_split.len() == 4 {
            //Check that everything formats properly
            for x in 0..4 {
                vec_format[x] = match vec_split[x].parse::<u8>() {
                    Err(e) => return Err(IpErrors::InvalidIp),
                    Ok(num) => num,
                }
            }
        }
        Ok((vec_format[0], vec_format[1], vec_format[2], vec_format[3]))
    }

    fn split_dots_into_vec(s: &String) -> Result<Vec<&str>, IpErrors> {
        //Figures out if there is enough elements to pass into the next functions
        let split_vec = s.split('.').collect();
        if (split_vec.len() != 4) {
            return IpErrors::InvalidLength;
        }
        Ok(split_vec);
    }
    fn calculate_mask_from_cidr(cidr: u8) -> (u8, u8, u8, u8) {
        let filled_octets = cidr / 8;
        let remainder = get_subnet_from_remaining_bits(cidr % 8);
        let full_octet = 255;
        match filled_octets {
            1 => (full_octet, remainder, 0, 0),
            2 => (full_octet, full_octet, remainder, 0),
            3 => (full_octet, full_octet, full_octet, remainder),
            4 => (full_octet, full_octet, full_octet, full_octet),
            _ => (0, 0, 0, 0)
        }
    }


    /// Returns the number for a given number of bits that are turned on
    /// From right to left
    /// # Arguments
    /// * 'num' - the number of bits that are supposed to be flipped on
    ///             for an octet.
    /// # Example
    ///   3 = 240 
    /// 2 = 192 etc... 
    fn get_subnet_from_remaining_bits(num: u8) -> u8 {
        let mut total: u8 = 0;
        for x in 8 - num..8 {
            total += 2u8.pow(x as u32);
        }
        total
    }


}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}
