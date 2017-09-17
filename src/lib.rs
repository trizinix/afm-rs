//use std::error::Error;
//use std::fs::File;
//use std::io::prelude::*;
//use std::path::Path;

#[macro_use]
extern crate nom;
extern crate geo;

use std::str::FromStr;
use nom::{IResult, float_s, digit};
use geo::Bbox;

pub fn mytest() {
    println!("Hallo Welt");
}

struct CharMetric {
    name: String,
    bbox: Bbox<f32>,
    ligatures: Vec<Ligature>,
    characterCode: i32,
    wx: f32,
    w0x: f32,
    w1x: f32,
    wy: f32,
    w0y: f32,
    w1y: f32,
    w: Vec<f32>,
    w0: Vec<f32>,
    w1: Vec<f32>,
    vv: Vec<f32>
}

struct TrackKern {
    degree: i32,
    minPointSize: f32,
    minKern: f32,
    maxPointSize: f32,
    maxKern: f32
}

struct KernPair {
    firstKernCharacter: String,
    secondKernCharacter: String,
    x: f32,
    y: f32
}

struct Ligature {
    successor: String,
    ligature: String
}

struct Composite {
    name: String,
    parts: Vec<CompositePart>
}

struct CompositePart {
    name: String,
    xDisplacement: i32,
    yDisplacement: i32
}

enum Command {
    FontName(String),
    FullName(String),
    FamilyName(String),
    Weight(String),
    FontBBox(Bbox<f32>),
    Version(String),
    Notice(String),
    EncodingScheme(String),
    MappingScheme(u32),
    EscChar(u32),
    CharacterSet(String),
    Characters(u32),
    IsBaseFont(bool),
    VVector(f32, f32),
    IsFixedV(bool),
    CapHeight(f32),
    XHeight(f32),
    Ascender(f32),
    Descender(f32),
    StdHW(f32),
    StdVW(f32),
    Comment(String),
    UnderlinePosition(f32),
    UnderlineThickness(f32),
    ItalicAngle(f32),
    CharWidth(f32, f32),
    IsFixedPitch(bool),
    CharMetrics(Vec<CharMetric>),
    Composites(Vec<Composite>),
    KernData {kern: Vec<TrackKern>, 
              kernpairs: Vec<KernPair>, 
              kernpairs0: Vec<KernPair>, 
              kernpairs1: Vec<KernPair>}
}

fn is_eol(c : char) -> bool {
    c == '\r' || c == '\n'
}

//fn command_int(&str)


//named!(boolean<&str, bool>, ws!(map_res!(alt!(tag!("true"), tag!("false")), FromStr::from_str)));
//named!(boolean<&str, bool>, map_res!(tag!("false"), FromStr::from_str));

named!(afm_version<&str, f32>, ws!(preceded!(tag!("StartFontMetrics"), call!(float_s))));
named!(font_name <&str, &str>, ws!(preceded!(tag!("FontName"), take_till!(is_eol))));
/*
named!(full_name <&str, &str>, ws!(preceded!(tag!("FullName"), take_till!(is_eol))));
named!(weight <&str, &str>, ws!(preceded!(tag!("Weight"), take_till!(is_eol))));
named!(font_bbox <&str, Bbox<f32>>, ws!(preceded!(tag!("FontBBox"), do_parse!(
    x1: call!(float_s) >>
    y1: call!(float_s) >>
    x2: call!(float_s) >>
    y2: call!(float_s) >>
        // (x1, y1) is LowerLeft,(x2, y2) is UpperRight
        (Bbox{xmin: x1,  xmax: x2, ymin: y1, ymax: y2})
    ))));
named!(version <&str, &str>, ws!(preceded!(tag!("Version"), take_till!(is_eol))));
named!(notice <&str, &str>, ws!(preceded!(tag!("Notice"), take_till!(is_eol))));
named!(encoding_scheme <&str, &str>, ws!(preceded!(tag!("EncodingScheme"), take_till!(is_eol))));
named!(mapping_scheme <&str, i32>, ws!(preceded!(tag!("MappingScheme"), call!(digit))));
named!(esc_char <&str, i32>, ws!(preceded!(tag!("EscChar"), call!(digit))));
named!(character_set <&str, &str>, ws!(preceded!(tag!("CharacterSet"), take_till!(is_eol))));
named!(characters <&str, i32>, ws!(preceded!(tag!("Characters"), call!(digit))));
named!(is_base_font <&str, i32>, ws!(preceded!(tag!("IsBaseFont"), call!(boolean))));
*/

//named!(kern_data<&str, Command>, (do_parse!());
named!(kern_pairs<&str, Vec<KernPair>, do_parse!(
    n:     call!(digit) >>
    pairs: take!(n)
))

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_courier() {
        assert_eq!(afm_version(&"StartFontMetrics 4.1\n\nComment C"), IResult::Done("Comment C", 4.1));
        assert_eq!(font_name(&"\nFontName Courier\n"), IResult::Done("", "Courier"));
        /*let path = Path::new("files/Courier.afm");
        let mut file = File::open(&path).expect("Could not open {} file".format(path.display()));
        let mut s = String::new();
        file.read_to_string(&mut s).expect("Could not read {}".format(path.display()));*/
    }
}
