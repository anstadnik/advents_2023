use anyhow::anyhow;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Balls {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl TryFrom<Vec<u32>> for Balls {
    type Error = anyhow::Error;

    fn try_from(v: Vec<u32>) -> Result<Self, Self::Error> {
        (v.len() == 3)
            .then(|| Balls {
                r: v[0],
                g: v[1],
                b: v[2],
            })
            .ok_or(anyhow!("Wrong number of balls"))
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub balls: Vec<Balls>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.strip_prefix("Game ").ok_or(anyhow!("No prefix"))?;
        let colon_idx = s.find(':').ok_or(anyhow!("No colon"))?;
        let id = s[..colon_idx].parse::<u32>()?;
        let balls_regexes =
            ["red", "green", "blue"].map(|c| Regex::new(&format!(r"(\d+) {c}")).unwrap());
        let balls = s[colon_idx + 1..]
            .split(';')
            .map(|b| {
                balls_regexes
                    .iter()
                    .map(|r| r.captures(b).map_or(Ok(0), |c| c[1].parse::<u32>()))
                    .collect::<Result<Vec<_>, _>>()?
                    .try_into()
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        Ok(Game { id, balls })
    }
}
