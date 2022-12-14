use crate::util::read_input;

pub fn part1() -> usize {
    read_section_pairs()
        .iter()
        .fold(0, |it, pair| if pair.one_contains_other() { it + 1 } else { it } )
}

pub fn part2() -> usize {
    read_section_pairs()
        .iter()
        .fold(0, |it, pair| if pair.one_overlaps_other() { it + 1 } else { it } )
}


fn read_section_pairs() -> Vec<SectionPair> {
    read_input("input/day04")
        .iter()
        .map(|it| it.split_once(",").expect("could not split"))
        .map(|it| SectionPair {
            first: Section {
                start: it.0.split_once("-").expect("").0.parse::<usize>().expect("Cannot Parse Int"),
                end: it.0.split_once("-").expect("").1.parse::<usize>().expect("Cannot Parse Int"),
            },
            second: Section {
                start: it.1.split_once("-").expect("").0.parse::<usize>().expect("Cannot Parse Int"),
                end: it.1.split_once("-").expect("").1.parse::<usize>().expect("Cannot Parse Int"),
            }
        })
        .collect()
}


struct SectionPair {
    first: Section,
    second: Section
}

impl SectionPair {
    fn one_contains_other(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    fn one_overlaps_other(&self) -> bool {
        self.first.overlaps(&self.second)
    }
}

struct Section {
    start: usize,
    end: usize,
}

impl Section {
    pub fn contains(&self, other: &Section) -> bool {
        (self.start <= other.start) && (self.end >= other.end)
    }

    pub fn overlaps(&self, other: &Section) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}