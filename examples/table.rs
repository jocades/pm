use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct Language {
    name: &'static str,
    designed_by: &'static str,
    invented_year: usize,
}

fn main() {
    let languages = vec![
        Language {
            name: "C",
            designed_by: "Dennis Ritchie",
            invented_year: 1972,
        },
        Language {
            name: "Go",
            designed_by: "Rob Pike",
            invented_year: 2009,
        },
        Language {
            name: "Rust",
            designed_by: "Graydon Hoare",
            invented_year: 2010,
        },
    ];

    let mut table = Table::new(languages);
    table.with(Style::modern_rounded());

    println!("{table}")
}
