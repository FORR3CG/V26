use std::fmt::{Debug, Display};

#[derive(Debug, Default, Eq)]
struct Punktur {
    // Ef við derive-um Default trait-ið
    lysing: String, // lysing: String::default()
    x: i32,         // i32::default()
    y: i32,         // i32::default()
}

impl Punktur {
    fn new(lysing: &str, x: i32, y: i32) -> Self {
        Self {
            lysing: lysing.to_string(),
            x,
            y,
        }
    }

    fn fjarlaegd(&self) -> f32 {
        // sqrt(x*x + y*y)
        ((self.x as f32).powi(2) + (self.y as f32).powi(2)).sqrt()
    }
}

impl Display for Punktur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - x: {}, y: {}, fjarl: {}",
            self.lysing,
            self.x,
            self.y,
            self.fjarlaegd()
        )
    }
}

/* impl Default for Punktur {
    fn default() -> Self {
        Self { lysing: "núllpunktur".to_string(), x: 0, y: 0 }
    }
} */

/* impl Debug for Punktur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Debug punktur: lýsing: {}, x: {}, y: {}, fjarl: {}"
                , self.lysing, self.x, self.y, self.fjarlaegd()
        )
    }
} */

impl PartialEq for Punktur {
    fn eq(&self, other: &Self) -> bool {
        // lysing þarf að vera eins, x-in og y-in líka
        // self.lysing == other.lysing &&
        // self.x == other.x &&
        // self.y == other.y
        // eða t.d.:
        // ef fjarlægðin er sú sama, segjum við að þeir séu eins
        self.fjarlaegd() == other.fjarlaegd() && self.lysing == other.lysing
    }
}

impl PartialOrd for Punktur {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Punktur {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        /* if self.fjarlaegd() == other.fjarlaegd() {
            self.lysing.cmp(&other.lysing)
        } else {
            self.fjarlaegd().total_cmp(&other.fjarlaegd())
        } */
        // eða
        self.fjarlaegd()
            .total_cmp(&other.fjarlaegd())
            .then(self.lysing.cmp(&other.lysing))
    }
}

impl std::ops::Add<Punktur> for Punktur {
    type Output = Punktur;

    fn add(self, rhs: Punktur) -> Self::Output {
        Self {
            lysing: format!("{}_{}", self.lysing, rhs.lysing),
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

fn main() {
    //let mut dp = Punktur::default();
    let mut punktar = vec![
        Punktur::new("c", 10, 20),
        Punktur::new("b", 0, 20),
        Punktur::new("d", 4, 3),
        Punktur::new("e", 5, 1),
        Punktur::new("a", 5, 1),
    ];
    punktar.iter().for_each(|p| println!("{}", p));
    punktar.sort();
    println!("--------------------------");
    for p in &punktar {
        println!("{}", p)
    }
    let p1 = Punktur::new("a", 10, 20);
    let p2 = Punktur::new("b", 7, 3);
    let p3 = p1 + p2;
    println!("{}", p3)
    //let p3 = p1.add(p2);
}
