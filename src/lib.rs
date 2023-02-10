extern crate reqwest;

mod macros;
mod structures;
mod errors;
mod interfaces;

#[cfg(test)]
mod tests {
    use crate::interfaces::Api;
    use std::fs::File;
    use std::io::Read;
    use crate::interfaces::Interface;

    #[test]
    fn test_new() {
        #[allow(unused_variables)]
        let interface = Api::new("false_secret".to_string());
    }

    fn read_secret() -> String {
        let mut file = File::open("auth").unwrap_or_else(|_| { panic!("Cannot open auth file") });
        let mut secret = String::new();
        file.read_to_string(&mut secret).expect("Cannot read auth file");

        secret.trim().to_string()
    }

    #[test]
    fn test_plain() {
        let interface = Api::new(read_secret());
        println!("GETing to /test ...");
        match interface.get("/test") {
            Ok(r) => {
                println!("Response:\n\n====================\n{}\n====================", r)
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_areas() {
        let interface = Interface::new(read_secret());
        println!("Calling interface.areas()...\n");
        println!("{:?}", interface.areas());
    }

    #[test]
    fn test_routes() {
        let interface = Interface::new(read_secret());
        println!("Calling interface.routes()...\n");
        let areas = interface.areas().unwrap();
        println!("{:?}", areas);
        println!("{:?}", interface.routes(areas));
    }
}


