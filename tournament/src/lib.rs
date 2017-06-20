use std::collections::HashMap;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Team<'a> {
    name: &'a str,
}
impl<'a> Team<'a> {
    fn new(name: &'a str) -> Self {
        Team { name }
    }
}

enum MatchResult<'a> {
    NonDraw { winner: Team<'a>, loser: Team<'a> },
    Draw(Team<'a>, Team<'a>),
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Stats {
    // This field order is needed for Ord to be derived correctly. Stats will be ordered by
    // their point value (wins = 3, draws = 1, and losses = 0).
    wins: usize,
    draws: usize,
    losses: usize,
}
impl Stats {
    fn new() -> Self {
        Stats {
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    fn matches_played(&self) -> usize {
        self.wins + self.losses + self.draws
    }

    fn points(&self) -> usize {
        (self.wins * 3) + self.draws
    }
}

struct Tally<'a> {
    data: HashMap<Team<'a>, Stats>,
}
impl<'a> Tally<'a> {
    fn new() -> Self {
        Tally { data: HashMap::new() }
    }

    fn add(&mut self, match_result: MatchResult<'a>) {
        match match_result {
            MatchResult::NonDraw { winner, loser } => {
                self.stats(winner).wins += 1;
                self.stats(loser).losses += 1;
            }
            MatchResult::Draw(team_1, team_2) => {
                self.stats(team_1).draws += 1;
                self.stats(team_2).draws += 1;
            }
        }
    }

    fn stats(&mut self, team: Team<'a>) -> &mut Stats {
        self.data.entry(team).or_insert(Stats::new())
    }
}
impl<'a> IntoIterator for &'a Tally<'a> {
    type Item = (&'a Team<'a>, &'a Stats);
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut values = self.data.iter().collect::<Vec<_>>();
        // Order by points, descending. In case of a tie, teams are ordered alphabetically.
        values.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        values.into_iter()
    }
}


#[derive(Debug)]
enum ParseError {
    NotEnoughFields,
    InvalidMatchOutcome,
}

// TODO: Use std::convert::TryFrom once it's stabilized.
trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
impl<'a> TryFrom<&'a str> for MatchResult<'a> {
    type Error = ParseError;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut input_fields = input.split(';');
        match (input_fields.next(), input_fields.next(), input_fields.next()) {
            (Some(winner_name), Some(loser_name), Some("win")) |
            (Some(loser_name), Some(winner_name), Some("loss")) => {
                Ok(MatchResult::NonDraw {
                       winner: Team::new(winner_name),
                       loser: Team::new(loser_name),
                   })
            }
            (Some(name_1), Some(name_2), Some("draw")) => {
                Ok(MatchResult::Draw(Team::new(name_1), Team::new(name_2)))
            }
            (Some(_), Some(_), Some(_)) => Err(ParseError::InvalidMatchOutcome),
            _ => Err(ParseError::NotEnoughFields),
        }
    }
}

macro_rules! row {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => (
    format!("{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            $a,
            $b,
            $c,
            $d,
            $e,
            $f)
    )
}

fn tally_safe(input: &str) -> Result<String, ParseError> {
    let mut tally = Tally::new();
    for line in input.lines() {
        tally.add(MatchResult::try_from(line)?);
    }

    let mut output = row!("Team", "MP", "W", "D", "L", "P");
    for (team, stats) in &tally {
        output.push('\n');
        output.push_str(&row!(team.name,
                              stats.matches_played(),
                              stats.wins,
                              stats.draws,
                              stats.losses,
                              stats.points()));
    }
    Ok(output)
}

pub fn tally(input: &str) -> String {
    tally_safe(input).unwrap()
}
