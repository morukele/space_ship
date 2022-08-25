use rand_derive2::RandGen;
use strum_macros::{Display, EnumString};
use std::str::FromStr;
use inquire::{Text, Select};
use rand::{random, Rng, thread_rng};

#[derive(Debug, RandGen, Display)]
enum Name{
    Akira, Californa, Daedalus,
    Eisenberg, Interpid, Miranda,
    Nova, Reliant, Sagan
}

#[derive(Debug, RandGen)]
struct Station{
    name: Name,
    version: u8,
    sections: Vec<Section>
}

#[derive(Debug, RandGen, Eq, PartialEq)]
struct Section{
    name: SectionName,
    active: bool,
}

#[derive(Debug, RandGen, Display, EnumString)]
#[derive(Eq, PartialEq)]
enum SectionName{
    AstroScience, Solar, Antenna,
    RadiationMirrors, Sleeping, NuclearGenerator,
    Galley, Transponder, Tracking
}

impl Station {
    /// Creates a new space station
    fn new() -> Self{
        Station{
            name:random(),
            version:random(),
            sections:(0..10).map(|_| random()).collect()
        }
    }

    /// Checks number of days left for the station to function
    fn days_left(&self) -> usize{
        self.sections
            .iter()
            .filter(|m| m.active)
            .count()
    }

    /// Returns names of active sections
    fn working_sections(&self) -> Vec<String>{
        self.sections.iter()
            .filter(|m|m.active)
            .map(|m| m.name.to_string())
            .collect()
    }

    /// Returns names of inactive sections
    fn broken_sections(&self) -> Vec<String>{
        self.sections.iter()
            .filter(|m|!m.active)
            .map(|m| m.name.to_string())
            .collect()
    }

    fn new_day(&mut self) {
        self.break_something();
    }

    fn break_something(&mut self) {
        let broken_index = thread_rng().gen_range(0..self.sections.len());
        let mut broken_section = &mut self.sections[broken_index];
        if broken_section.active {
            broken_section.active = false;
            println!("(Section-FAILURE {})", &broken_section.name);
        } else {
            println!("(sections OK)");
        }
    }
    fn status(&self) {
        dbg!(&self);
    }
}

/// Fixes `broken_section` on `station`
fn repair(broken_section:String, station: &mut Station){
    let section = SectionName::from_str(broken_section.as_str()).unwrap();

    let broken_index = station.sections.iter()
        .position(|m|m.name == section)
        .expect("Section not found.");

    station.sections[broken_index].active = true;
}

/// Break something in the station
fn science(working_section: String, station: &mut Station) {
    station.break_something();
}

/// Build a simple menu based on `items`
fn menu(items: &[String]) -> String{
    Select::new("MENU", items.to_vec())
        .prompt()
        .unwrap()
}

fn main() {
    let mut station = Station::new();
    let mut station_log = vec![];

    loop {
        let days_left = station.days_left();
        if days_left < 1{
            println!("(END-TRANSMISSION)");
            break;
        }
        println!("{days_left} UNTIL FINAL TRANSMISSION");
        station_log.push(Text::new("Enter your log:").prompt().unwrap());
        match menu(&[
            "NEW DAY".into(), "STATUS".into(),
            "POWERDOWN".into()]).as_str() {
            "NEW DAY" => {
                // menu system defined after here
                station.new_day();
                match menu(&["REPAIR".into(), "SCIENCE".into()]).as_str() {
                    "REPAIR" => {
                        repair(menu(&station.broken_sections()), &mut station);
                        continue;
                    },
                    "SCIENCE" => {
                        science(menu(&station.working_sections()), &mut station);
                        continue;
                    }
                    &_ => panic!(),
                }
            },
            "STATUS" => station.status(),
            "POWERDOWN" => break,
            &_ => panic!("test"),
        }
    }
    dbg!(station_log);
}
