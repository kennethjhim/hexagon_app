// plain old data
struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

fn execute(req: Request) -> u16 {
    req.number
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(req);

        assert_eq!(res, number)
    }
}