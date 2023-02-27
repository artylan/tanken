use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    pub fn new(datetext: &str) -> Self {
        let mut split = datetext.split(".");
        Date {
            day: split.next().unwrap().parse().unwrap(),
            month: split.next().unwrap().parse().unwrap(),
            year: split.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Record {
    pub date: Date,
    pub km: u32,
    pub liter: f32,
    pub costs: f32,
}

impl Record {
    pub fn new(line: &str) -> Record {
        let mut split = line.split("\t");
        Record {
            date: Date::new(split.next().unwrap()),
            km: split.next().unwrap().parse().unwrap(),
            liter: Self::parse_german(split.next().unwrap()),
            costs: Self::parse_german(split.next().unwrap()),
        }
    }

    fn parse_german(numtext: &str) -> f32 {
        numtext.replace(",", ".").parse().unwrap()
    }
}

#[derive(Debug)]
pub struct Statistics {
    data: Vec<Record>,
}

impl Statistics {
    pub fn new(filename: &str) -> Statistics {
        Statistics {
            data: Self::parse_file(filename),
        }
    }

    fn parse_file(filename: &str) -> Vec<Record> {
        let file = File::open(filename).unwrap();
        let mut stats: Vec<Record> = Vec::new();
        BufReader::new(file).lines().for_each(|line| {
            stats.push(Record::new(&line.unwrap()));
        });
        stats
    }

    pub fn get_years<F>(&self, f: F) -> Vec<(u16, f32)>
    where
        F: Fn(&Record) -> f32,
    {
        let mut map: HashMap<u16, f32> = HashMap::new();
        self.data.iter().for_each(|rec| {
            map.entry(rec.date.year)
                .and_modify(|v| *v = *v + f(rec))
                .or_insert(f(rec));
        });

        let mut years: Vec<(u16, f32)> = map.into_iter().collect();
        years.sort_by_key(|v| v.0);
        years
    }

    pub fn get_kilometers(&self) -> u32 {
        self.data.last().unwrap().km - self.data.first().unwrap().km
    }

    pub fn get_consumption(&self) -> f32 {
        let mut liter: f32 = self.data.iter().map(|r| r.liter).sum();
        liter -= self.data.last().unwrap().liter;
        liter / self.get_kilometers() as f32 * 100 as f32
    }

    pub fn get_total<F>(&self, f: F) -> f32
    where
        F: Fn(&Record) -> f32,
    {
        self.data.iter().map(|r| f(r)).sum()
    }

    pub fn get_average(&self) -> f32 {
        self.get_total(|r| r.costs) / self.get_total(|r| r.liter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date() {
        let d = Date::new("7.04.2022");
        assert_eq!(d.day, 7);
        assert_eq!(d.month, 4);
        assert_eq!(d.year, 2022);
    }

    #[test]
    fn parse_record() {
        let r = Record::new("7.04.2022\t100000\t90,5\t120,6");
        assert_eq!(r.date.day, 7);
        assert_eq!(r.date.month, 4);
        assert_eq!(r.date.year, 2022);
        assert_eq!(r.km, 100000);
        assert_eq!(r.liter, 90.5);
        assert_eq!(r.costs, 120.6);
    }
}
