use shared::iterator::ToArray;

pub struct Section {
    pub start: usize,
    pub end: usize,
}

pub fn parse_sections(line: &str) -> [Section; 2] {
    line.splitn(2, ',').map(parse_section).collect_array()
}

fn parse_section(section: &str) -> Section {
    let [start, end] = section.splitn(2, '-').collect_array();

    Section {
        start: start.parse().expect("expected integer"),
        end: end.parse().expect("expected integer"),
    }
}
