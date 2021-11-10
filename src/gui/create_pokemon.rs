use crate::domain::entities::{PokemonName, PokemonNumber, PokemonTypes, Pokemon};
use crate::repositories::pokemon::{Repository, InsertError};
use std::sync::Arc;

// plain old data
pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) => match repo.insert(number, name, types) {
            Ok(Pokemon {
                number,
                name,
                types,
            }) => Ok(Response {
                number: u16::from(number),
                name: String::from(name),
                types: Vec::<String>::from(types),
            }),
            Err(InsertError::Conflict) => Err(Error::Conflict),
            Err(InsertError::Unknown) => Err(Error::Unknown),
        },
        _ => Err(Error::BadRequest),
    }
}