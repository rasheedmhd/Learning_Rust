enum Fruit {
    Citrus(String),
    Pineapple(String),
}

enum Currency {
    BTC(f64),
    GHS(f64),
    USD(f64),
}

fn main() {
    let four = "V4"; // ip address v4
    let e_four = IpAddr::V4;

    let price = Currency::BTC(1.0);
    let price = Currency::GHS(0.004); //
    let price: f64 = 0.004; 

    let orange = Fruit::Citrus(String::from("Orange"));
    let home2 = IpAddr::V4(String::from("0.0.0.0"));


    fn is_valid_ip_addr(v: String, addr: String) {
        if v == four {

        }
    }

    fn e_is_valid_ip_addr(v: IpAddrKind, addr: String) {

    }
    e_is_valid_ip_addr(IpAddrKind::V4, "0.0.0.0");
    e_is_valid_ip_addr(IpAddrKind::V6, "0.0.0.0");
}