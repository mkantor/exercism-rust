pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    // Associated constants aren't stabilized yet, otherwise I'd prefer this:
    // const YEAR_LEN: Duration;
    fn year_len() -> Duration;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::year_len().seconds as f64
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn year_len() -> Duration {
        Duration::from(7600544)
    }
}
impl Planet for Venus {
    fn year_len() -> Duration {
        Duration::from(19414149)
    }
}
impl Planet for Earth {
    fn year_len() -> Duration {
        Duration::from(31557600)
    }
}
impl Planet for Mars {
    fn year_len() -> Duration {
        Duration::from(59354033)
    }
}
impl Planet for Jupiter {
    fn year_len() -> Duration {
        Duration::from(374355659)
    }
}
impl Planet for Saturn {
    fn year_len() -> Duration {
        Duration::from(929292363)
    }
}
impl Planet for Uranus {
    fn year_len() -> Duration {
        Duration::from(2651370019)
    }
}
impl Planet for Neptune {
    fn year_len() -> Duration {
        Duration::from(5200418560)
    }
}
