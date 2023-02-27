use tanken::Statistics;

fn main() {
    let stats = Statistics::new("./tanken.txt");

    println!("Kosten pro Jahr:");
    println!("----------------");
    stats
        .get_years(|r| r.costs)
        .iter()
        .for_each(|(y, c)| println!("Jahr {}: {:>8.2} Euro.", y, c));

    println!();
    println!("Liter pro Jahr:");
    println!("---------------");
    stats
        .get_years(|r| r.liter)
        .iter()
        .for_each(|(y, c)| println!("Jahr {}: {:>8.2} Liter.", y, c));

    println!();
    println!("Gefahrene Kilometer: {}.", stats.get_kilometers());

    println!();
    println!("Gesamtverbrauch: {:.2} Liter.", stats.get_total(|r| r.liter));

    println!();
    println!("Gesamtkosten: {:.2} Euro.", stats.get_total(|r| r.costs));

    println!();
    println!("Verbrauch {:.2} Liter pro 100 km.", stats.get_consumption());
    println!();
}
