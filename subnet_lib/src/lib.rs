use std::process;
use std::net::Ipv4Addr;

mod subnetter {



	pub enum IpErrors {
		InvalidInput(u8),
		InvalidCidr(u8),
	}

	struct IpSubnet {
		ip: std::net::Ipv4Addr,
		mask: (u8,u8,u8,u8),
		cidr: u8,
	}

	fn new(ip_string: String) -> IpSubnet {
		IpSubnet {
			ip: std::net::Ipv4Addr::new(127,0,0,1),
			mask: (255,255,255,0),
			cidr:27,
		}
	}

}
#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
