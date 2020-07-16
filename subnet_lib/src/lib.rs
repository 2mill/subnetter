mod subnetter {
    use std::net::Ipv4Addr;
    pub enum IpErrors {
        InvalidInput(u8),
    }

    struct IpSubnet {
        ip: Ipv4Addr,
        mask: (u8, u8, u8, u8), 
        cidr: u8,
    }

    pub fn new(ip_string: String) {
        //Begin by parsing the String into parts.

    }    

    pub fn split_string(ip_string: &String) -> Result<Vec<&str>, IpErrors> {
        //Splititng at the slash notation and if the result is not a len of 2 then we return an error.
        let split: Vec<&str> = ip_string.split('/').collect();
        if split.len() == 2 {
            return Ok(split);
        }
        Err(IpErrors::InvalidInput(1))
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
}