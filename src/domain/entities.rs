use std::convert::TryFrom;

// business rules

// Pokenumber - number has to be > 0 and < 899
pub struct PokemonNumber(u16);

impl TryFrom<u16> for PokemonNumber {
    type Error = ();
    
    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n > 0 && n < 899 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<PokemonNumber> for u16 {
    fn from(n: PokemonNumber) -> u16 {
        n.0
    }
}

// Pokename - the name cannot be an empty string
pub struct PokemonName(String);

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

// Poketypes - should not be an empty list and one of the defined Pokemon types. For now, we define Electric type
pub struct PokemonTypes(Vec<PokemonType>);

impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.iter() {
                match PokemonType::try_from(String::from(t)) {
                    Ok(pt) => pts.push(pt),
                    _ => return Err(()),
                }
            }
            Ok(Self(pts))
        }
    }
}

enum PokemonType {
    Electric,
}

impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Electric" => Ok(Self::Electric),
            _ => Err(()),
        }
    }
}
