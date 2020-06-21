mod subnetter {
    use std::net::{IpAddr, Ipv4Addr};
    enum IpErrors {
        InvalidLength,
        InvalidIp,
        InvalidInput,
        NoCIDR,
    }
    pub struct IpSubnet {
        ip: IpAddr,
        cidr_notation: u8,
        mask: (u8, u8, u8, u8),
    }

    impl IpSubnet {
        pub fn new() -> IpSubnet {
            IpSubnet {
                ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2)),
                cidr_notation: 29,
                mask: calculate_mask_from_cidr(29),
            }
        }
    }

    //Fix this tomorrow
    pub fn get_tuple_ip_from_string(s: &String) -> Result<(u8, u8, u8, u8), IpErrors> {
        let vec_split = split_dots_into_vec(s);
        if vec_split.len() != 4 {
            return Err(IpErrors::InvalidIp);
        }
        let builder: (u8, u8, u8, u8);
        for x in 0..4 {
            builder.x = {
                match vec_split[x].parse::<u8>() {
                    Err(e) => return Err(IpErrors::InvalidIp),
                    Ok(num) => num,
                }
            }          
        }
        Ok(builder)
    }

    fn split_dots_into_vec(s: &String) -> Vec<&str> {
        s.split('.').collect()
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
