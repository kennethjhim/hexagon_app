use druid::{im::Vector, Data, Lens};
use crate::domain::entities::{PokemonName, PokemonNumber, PokemonTypes};
use crate::repositories::pokemon::{FetchAllError, Repository};
use std::sync::Arc;

pub enum Error {
    Unknown,
}

#[derive(Clone, Data, Lens)]
pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vector<String>
}

pub fn execute(repo: Arc<dyn Repository>) -> Result<Vector<Response>, Error> {
    match repo.fetch_all() {
        Ok(pokemons) => Ok(pokemons
            .into_iter()
            .map(|p| Response {
                number: u16::from(p.number),
                name: String::from(p.name),
                types: Vector::from(Vec::<String>::from(p.types)),
            })
            .collect::<Vector<Response>>()),
        Err(FetchAllError::Unknown) => Err(Error::Unknown),
    }
}