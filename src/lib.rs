use pom::char_class;
use pom::Parser;
use pom::parser::*;
use std::str::FromStr;
use std::string::*;
use std::str;
use std::char;

use std::collections::HashMap;

extern crate pom;
extern crate geo;

use geo::Bbox;

#[derive(PartialEq, Debug)]
pub struct FontMetrics {
    metric_sets: i32,
    font_name: String,
    full_name: String,
    family_name: String,
    weight: String,
    font_bbox: Bbox<f64>,
    font_version: String,
    notice: String,
    encoding_scheme: String,
    mapping_scheme: u32,
    esc_char: u32,
    character_set: String,
    characters: u32,
    is_base_font: bool,
    v_vector: (f64, f64),
    is_fixed_v: bool,
    cap_height: f64,
    x_height: f64,
    ascender: f64,
    descender: f64,
    comments: Vec<String>,

    underline_position: f64,
    underline_thickness: f64,
    italic_angle: f64,
    char_width: (f64, f64),
    is_fixed_pitch: bool,
    standard_horizontal_width: f64,
    standard_vertical_width: f64,

    char_metrics: Vec<CharMetric>,
    char_metrics_map: HashMap<String, CharMetric>,
    track_kern: Vec<TrackKern>,
    composites: Vec<Composite>,
    kern_pairs: Vec<KernPair>,
    kern_pairs0: Vec<KernPair>,
    kern_pairs1: Vec<KernPair>
}

impl Default for FontMetrics {
    fn default() -> FontMetrics {
        FontMetrics {
            // TODO check for actual default values
            metric_sets: 0,
            font_name: String::new(),
            full_name: String::new(),
            family_name: String::new(),
            weight: String::new(),
            font_bbox: Bbox{xmin: 0.,  xmax: 0., ymin: 0., ymax: 0.},
            font_version: String::new(),
            notice: String::new(),
            encoding_scheme: String::new(),
            mapping_scheme: 0,
            esc_char: 0,
            character_set: String::new(),
            characters: 0,
            is_base_font: true,
            v_vector: (0.0, 0.0),
            is_fixed_v: true,
            cap_height: 0.0,
            x_height: 0.0,
            ascender: 0.0,
            descender: 0.0,
            comments: Vec::new(),

            underline_position: 0.0,
            underline_thickness: 0.0,
            italic_angle: 0.0,
            char_width: (0.0, 0.0),
            is_fixed_pitch: true,
            standard_horizontal_width: 0.0,
            standard_vertical_width: 0.0,

            char_metrics: Vec::new(),
            char_metrics_map: HashMap::new(),
            track_kern: Vec::new(),
            composites: Vec::new(),
            kern_pairs: Vec::new(),
            kern_pairs0: Vec::new(),
            kern_pairs1: Vec::new()
        }
    }
}


#[derive(PartialEq, Debug)]
pub struct CharMetric {
    name: String,
    bbox: Bbox<f64>,
    ligatures: Vec<Ligature>,
    character_code: i32,
    wx: f64,
    w0x: f64,
    w1x: f64,
    wy: f64,
    w0y: f64,
    w1y: f64,
    w: (f64, f64),
    w0: (f64, f64),
    w1: (f64, f64),
    vv: (f64, f64)
}

impl Default for CharMetric {
    fn default()-> CharMetric {
        CharMetric {
            name: String::new(),
            bbox: Bbox{xmin: 0.,  xmax: 0., ymin: 0., ymax: 0.},
            ligatures: Vec::new(),
            character_code: 0,
            wx: 0.0,
            w0x: 0.0,
            w1x: 0.0,
            wy: 0.0,
            w0y: 0.0,
            w1y: 0.0,
            w: (0.0,0.0),
            w0: (0.0, 0.0),
            w1: (0.0, 0.0),
            vv: (0.0, 0.0)
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct TrackKern {
    degree: i32,
    min_point_size: f64,
    min_kern: f64,
    max_point_size: f64,
    max_kern: f64
}

#[derive(PartialEq, Debug)]
pub struct KernPair {
    first_kern_character: String,
    second_kern_character: String,
    x: f64,
    y: f64
}

#[derive(PartialEq, Debug)]
pub struct Ligature {
    successor: String,
    ligature: String
}

#[derive(PartialEq, Debug)]
pub struct Composite {
    name: String,
    parts: Vec<CompositePart>
}

#[derive(PartialEq, Debug)]
pub struct CompositePart {
    name: String,
    x_displacement: i32,
    y_displacement: i32
}

fn string_char(c: u8) -> bool {
    c >= 0x20 && c <= 0x7E
}

fn name_char(c: u8) -> bool {
    string_char(c) && (!char_class::space(c))
}

fn digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn space() -> Parser<u8, ()> {
	is_a(char_class::space).repeat(1..).discard()
}

fn eol() -> Parser<u8, ()> {
    (is_a(char_class::space).repeat(0..) - one_of(b"\r\n").repeat(1..)).discard()
}

// Types

fn string() -> Parser<u8, String> {
    is_a(string_char).repeat(1..).convert(String::from_utf8)
}

fn name() -> Parser<u8, String> {
    is_a(name_char).repeat(1..).convert(String::from_utf8)
}

fn boolean() -> Parser<u8, bool> {
    seq(b"true").map(|_| true) | seq(b"false").map(|_| false)
}

fn integer() -> Parser<u8, i32> {
	  let integer = sym(b'-').opt() - (one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0'));
    integer.collect().convert(String::from_utf8).convert(|s| i32::from_str(&s))
}

fn uinteger() -> Parser<u8, u32> {
	  let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
    integer.collect().convert(String::from_utf8).convert(|s| u32::from_str(&s))
}

fn hex_integer() -> Parser<u8, i32> {
    let hex_digits = is_a(char_class::hex_digit).repeat(1..).collect();
    sym(b'<') * hex_digits.convert(|v| i32::from_str_radix(&String::from_utf8(v).unwrap(), 16))
}

fn number() -> Parser<u8, f64> {
	  let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
	  let frac = sym(b'.') + one_of(b"0123456789").repeat(1..);
	  let exp = one_of(b"eE") + one_of(b"+-").opt() + one_of(b"0123456789").repeat(1..);
	  let number = sym(b'-').opt() + integer + frac.opt() + exp.opt();
	  number.collect().convert(String::from_utf8).convert(|s| f64::from_str(&s))
}

fn bbox() -> Parser<u8, Bbox<f64>> {
    let numbers = (number() - space()).repeat(3) + number();
    numbers.map(|(nums, num)| Bbox {xmin: nums[0], xmax: nums[1], ymin: nums[2], ymax: num})
}

// Combinators

fn start_command(command: &'static [u8]) -> Parser<u8, ()> {
    (seq(command) * space()).discard()
}

fn string_command(command: &'static [u8], build: &'static Fn(String) -> Command) -> Parser<u8, Command> {
    start_command(command) * string().map(build)
}

fn integer_command(command: &'static [u8], build: &'static Fn(i32) -> Command) -> Parser<u8, Command> {
    start_command(command) * integer().map(build)
}

fn uinteger_command(command: &'static [u8], build: &'static Fn(u32) -> Command) -> Parser<u8, Command> {
    start_command(command) * uinteger().map(build)
}

fn number_command(command: &'static [u8], build: &'static Fn(f64) -> Command) -> Parser<u8, Command> {
    start_command(command) * number().map(build)
}

fn bool_command(command: &'static [u8], build: &'static Fn(bool) -> Command) -> Parser<u8, Command> {
    start_command(command) * boolean().map(build)
}

fn bbox_command(command: &'static [u8], build: &'static Fn(Bbox<f64>) -> Command) -> Parser<u8, Command> {
    start_command(command) * bbox().map(build)
}

fn num_num_command(command: &'static [u8], build: &'static Fn(f64,f64) -> Command) -> Parser<u8, Command> {
    start_command(command) * (number() - space() + number()).map(move |(a,b)| build(a,b))
}

// Kern Pairs
fn kp_cmd() -> Parser<u8, KernPair> {
    let cmd = seq(b"KP") - space();
    let names = name() - space() + name() - space();
    let nums = number() - space() + number();
    cmd * (names + nums).map(move |((n1, n2),(numx,numy))| {
        KernPair {first_kern_character: n1,
                  second_kern_character: n2,
                  x: numx,
                  y: numy
        }
    })
}

static ERROR_MSG: Result<String, &str> = Err("hex value not in ascii");

fn hex_string() -> Parser<u8, String> {
    let hex_bytes = is_a(char_class::hex_digit).repeat(2).collect();
    let hex_int = hex_bytes.convert(|b| u32::from_str_radix(str::from_utf8(&b).unwrap(), 16));
    let hex_char = hex_int.convert(|u| char::from_u32(u).ok_or(ERROR_MSG.clone()));
    sym(b'<') * hex_char.repeat(0..).map(|v| v.into_iter().collect()) - sym(b'>')
}

fn kph_cmd() -> Parser<u8, KernPair> {
    let cmd = seq(b"KPH") - space();
    let names = hex_string() - space() + hex_string() - space();
    let nums = number() - space() + number();
    cmd * (names + nums).map(move |((n1, n2),(numx,numy))| {
        KernPair {first_kern_character: n1,
                  second_kern_character: n2,
                  x: numx,
                  y: numy
        }
    })
}

fn kpx_cmd() -> Parser<u8, KernPair> {
    let cmd = seq(b"KPX") - space();
    let names = name() - space() + name() - space();
    let num = number();
    cmd * (names + num).map(|((name1, name2),num)| {
        KernPair {first_kern_character: name1,
                  second_kern_character: name2,
                  x: num,
                  y: 0.0
        }
    })
}

fn kpy_cmd() -> Parser<u8, KernPair> {
    let cmd = seq(b"KPY") - space();
    let names = name() - space() + name() - space();
    let num = number();
    cmd * (names + num).map(|((name1, name2),num)| {
        KernPair {first_kern_character: name1,
                  second_kern_character: name2,
                  x: 0.0,
                  y: num
        }
    })
}

fn kern_pair() -> Parser<u8, KernPair> {
    kp_cmd() | kph_cmd() | kpx_cmd() | kpy_cmd()
}

fn kern_pairs() -> Parser<u8, (Option<u8>, Vec<KernPair>)> {
    let command = seq(b"StartKernPairs") * one_of(b"01").opt();
    command - space() + uinteger() - eol().repeat(1..) >> move |(idx, len)| {
        let end = seq(b"EndKernPairs") - eol().repeat(1..);
        let pairs = (kern_pair() - eol().repeat(1..)).repeat(len as usize);
        pairs.map(move |pairs| (idx, pairs)) - end
    }
}

fn track_kern() -> Parser<u8, TrackKern> {
    let begin = seq(b"TrackKern") - space();
    let num_space = || { number() - space() };
    let content = integer() - space() + num_space() + num_space() + num_space() + num_space();
    begin * content.map(|((((deg, min_size), min_kern), max_size), max_kern)| {
        TrackKern {
            degree: deg,
            min_point_size: min_size,
            min_kern: min_kern,
            max_point_size: max_size,
            max_kern: max_kern
        }
    })
}

fn track_kerns() -> Parser<u8, Vec<TrackKern>> {
    let begin = (seq(b"StartTrackKern") - space()) * uinteger() - eol();
    begin >> move |len| {
        (track_kern() - eol()).repeat(len as usize) - seq(b"EndTrackKern") - eol()
    }
}

fn kern_data() -> Parser<u8, Vec<KernDataCmd>> {
    let kernpairs = kern_pairs().map(|(idx,pairs)| {
        match idx {
            Some(0) => KernDataCmd::Kernpairs0(pairs),
            Some(1) => KernDataCmd::Kernpairs1(pairs),
            Some(_) => { unreachable!() }
            None => KernDataCmd::Kernpairs(pairs)
        }
    });
    let trackkern = track_kerns().map(|kerns| KernDataCmd::TrackKern(kerns));
    let begin = seq(b"StartKernData") - eol().repeat(1..);
    let content = (kernpairs | trackkern).repeat(1..);
    begin * content - seq(b"EndKernData")
}


// Composites

fn composite_part() -> Parser<u8, CompositePart> {
    let params = (name() - space() + integer() - space() + integer())
        .map(|((n,x),y)| CompositePart{name: n, x_displacement: x, y_displacement: y});
    (seq(b"PCC") - space()) * params
}

fn composite() -> Parser<u8, Composite> {
    (seq(b"CC") - space()) * name() - space() + uinteger() >>
        |(name, len):(String,u32)| {
            (space() * composite_part()).repeat(len as usize).map(move |parts| {
                Composite {name: name.to_owned(), parts: parts}
            })
        }
}

fn composites() -> Parser<u8, Vec<Composite>> {
    (seq(b"StartComposites") - space()) * uinteger() - eol().repeat(1..) >>
        move |len| {
            (composite() - eol().repeat(1..)).repeat(len as usize) - seq(b"EndComposites")
        }
}


// Char Metrics

#[derive(PartialEq, Debug)]
enum CharMetricCommand {
    C(i32),
    WX(f64),
    W0X(f64),
    W1X(f64),
    WY(f64),
    W0Y(f64),
    W1Y(f64),
    W(f64, f64),
    W0(f64, f64),
    W1(f64, f64),
    VV(f64, f64),
    N(String),
    B(Bbox<f64>),
    L(Ligature)
}

fn ligature() -> Parser<u8, Ligature> {
    (name() - space() + name()).map(|(s,l)| Ligature {successor: s, ligature: l})
}

fn charcommand() -> Parser<u8, CharMetricCommand> {
    (sym(b'C') - space()) * integer().map(&CharMetricCommand::C)
        | (seq(b"CH") - space()) * hex_integer().map(&CharMetricCommand::C)
        | (seq(b"WX") - space()) * number().map(&CharMetricCommand::WX)
        | (seq(b"W0X") - space()) * number().map(&CharMetricCommand::W0X)
        | (seq(b"W1X") - space()) * number().map(&CharMetricCommand::W1X)
        | (seq(b"WY") - space()) * number().map(&CharMetricCommand::WY)
        | (seq(b"W0Y") - space()) * number().map(&CharMetricCommand::W0Y)
        | (seq(b"W1Y") - space()) * number().map(&CharMetricCommand::W1Y)
        | (seq(b"W") - space()) * (number() - space() + number()).map(|(x,y)| CharMetricCommand::W(x,y))
        | (seq(b"W0") - space()) * (number() - space() + number()).map(|(x,y)| CharMetricCommand::W0(x,y))
        | (seq(b"W1") - space()) * (number() - space() + number()).map(|(x,y)| CharMetricCommand::W1(x,y))
        | (seq(b"VV") - space()) * (number() - space() + number()).map(|(x,y)| CharMetricCommand::VV(x,y))
        | (sym(b'N') - space()) * name().map(CharMetricCommand::N)
        | (sym(b'B') - space()) * bbox().map(CharMetricCommand::B)
        | (sym(b'L') - space()) * ligature().map(CharMetricCommand::L)
}

fn char_metric() -> Parser<u8, CharMetric> {
    let seperator = || { space().opt() * sym(b';') - space().opt() };
    let cmds = list(charcommand(), seperator()) - seperator().opt();
    cmds.map(|commands| commands.into_iter().fold(
        CharMetric::default(),
        |mut metric: CharMetric, command: CharMetricCommand| {
            match command {
                CharMetricCommand::C(c) => metric.character_code = c,
                CharMetricCommand::WX(wx) => metric.wx = wx,
                CharMetricCommand::W0X(w0x) => metric.w0x = w0x,
                CharMetricCommand::W1X(w1x) => metric.w1x = w1x,
                CharMetricCommand::WY(wy) => metric.wy = wy,
                CharMetricCommand::W0Y(w0y) => metric.w0y = w0y,
                CharMetricCommand::W1Y(w1y) => metric.w1y = w1y,
                CharMetricCommand::W(w1, w2) => metric.w = (w1, w2),
                CharMetricCommand::W0(w1, w2) => metric.w0 = (w1, w2),
                CharMetricCommand::W1(w1, w2) => metric.w1 = (w1, w2),
                CharMetricCommand::VV(vv1, vv2) => metric.vv = (vv1, vv2),
                CharMetricCommand::N(name) => metric.name = name,
                CharMetricCommand::B(bbox) => metric.bbox = bbox,
                CharMetricCommand::L(lig) => metric.ligatures.push(lig)
            }
            metric
        }
    ))
}

fn char_metrics() -> Parser<u8, Vec<CharMetric>> {
    let begin = seq(b"StartCharMetrics") * space() * uinteger() - eol().repeat(1..);
    begin >> move |len: u32| {
        (char_metric() - eol().repeat(1..)).repeat(len as usize) - seq(b"EndCharMetrics")
    }
}

fn comment() -> Parser<u8, Command> {
    let cmd = (seq(b"Comment") - space()) * string().opt().map(|o| o.unwrap_or_else(|| String::new()));
    cmd.map(Command::Comment)
}

// Commands

#[derive(PartialEq, Debug)]
enum Command {
//    AfmVersion((i32, i32)),
    MetricsSet(i32),
    FontName(String),
    FullName(String),
    FamilyName(String),
    Weight(String),
    FontBBox(Bbox<f64>),
    Version(String),
    Notice(String),
    EncodingScheme(String),
    MappingScheme(u32),
    EscChar(u32),
    CharacterSet(String),
    Characters(u32),
    IsBaseFont(bool),
    VVector(f64, f64),
    IsFixedV(bool),
    CapHeight(f64),
    XHeight(f64),
    Ascender(f64),
    Descender(f64),
    StdHW(f64),
    StdVW(f64),
    Comment(String),
    UnderlinePosition(f64),
    UnderlineThickness(f64),
    ItalicAngle(f64),
    CharWidth(f64, f64),
    IsFixedPitch(bool),
    CharMetrics(Vec<CharMetric>),
    Composites(Vec<Composite>),
    KernData(Vec<KernDataCmd>)
}

#[derive(PartialEq, Debug)]
enum KernDataCmd {
    TrackKern(Vec<TrackKern>),
    Kernpairs(Vec<KernPair>),
    Kernpairs0(Vec<KernPair>),
    Kernpairs1(Vec<KernPair>)
}

fn command<'a>() -> Parser<u8, Command> {
    comment()
    | string_command(b"Version", &Command::Version)
    | integer_command(b"MetricsSet", &Command::MetricsSet)
    | string_command(b"FontName", &Command::FontName)
    | string_command(b"FullName", &Command::FullName)
    | string_command(b"FamilyName", &Command::FamilyName)
    | string_command(b"Weight", &Command::Weight)
    | bbox_command(b"FontBBox", &Command::FontBBox)
    | string_command(b"Weight", &Command::Weight)
    | string_command(b"Version", &Command::Version)
    | string_command(b"Notice", &Command::Notice)
    | string_command(b"EncodingScheme", &Command::EncodingScheme)
    | uinteger_command(b"MappingScheme", &Command::MappingScheme)
    | uinteger_command(b"EscChar", &Command::EscChar)
    | string_command(b"CharacterSet", &Command::CharacterSet)
    | uinteger_command(b"Characters", &Command::Characters)
    | bool_command(b"IsBaseFont", &Command::IsBaseFont)
    | num_num_command(b"VVector", &Command::VVector)
    | bool_command(b"IsFixedV", &Command::IsFixedV)
    | number_command(b"CapHeight", &Command::CapHeight)
    | number_command(b"XHeight", &Command::XHeight)
    | number_command(b"Ascender", &Command::Ascender)
    | number_command(b"Descender", &Command::Descender)
    | number_command(b"StdHW", &Command::StdHW)
    | number_command(b"StdVW", &Command::StdVW)
    | number_command(b"UnderlinePosition", &Command::UnderlinePosition)
    | number_command(b"UnderlineThickness", &Command::UnderlineThickness)
    | number_command(b"ItalicAngle", &Command::ItalicAngle)
    | num_num_command(b"CharWidth", &Command::CharWidth)
    | bool_command(b"IsFixedPitch", &Command::IsFixedPitch)
    | char_metrics().map(Command::CharMetrics)
    | composites().map(Command::Composites)
    | kern_data().map(Command::KernData)
}

// Public functions


pub fn afm() -> Parser<u8, FontMetrics> {
    let begin = start_command(b"StartFontMetrics") * (is_a(digit) - sym(b'.') + is_a(digit)) - eol();
    let end = eol().opt() * seq(b"EndFontMetrics") * eol().repeat(0..) * end();
    let elems = list(command(), eol());

    let commands = begin * elems.expect("AFM commands") - end.expect("EndFontMetrics");
    commands.map(|commands| commands.into_iter().fold(
        FontMetrics::default(),
        |mut metric: FontMetrics, command: Command| {
            match command {
                Command::MetricsSet(metric_sets) => metric.metric_sets = metric_sets,
                Command::FontName(name) => metric.font_name = name,
                Command::FullName(name) => metric.full_name = name,
                Command::FamilyName(name) => metric.family_name = name,
                Command::Weight(weight) => metric.weight = weight,
                Command::FontBBox(bbox) => metric.font_bbox = bbox,
                Command::Version(version) => metric.font_version = version,
                Command::Notice(notice) => metric.notice = notice,
                Command::EncodingScheme(scheme) => metric.encoding_scheme = scheme,
                Command::MappingScheme(scheme) => metric.mapping_scheme = scheme,
                Command::EscChar(c) => metric.esc_char = c,
                Command::CharacterSet(charset) => metric.character_set = charset,
                Command::Characters(c) => metric.characters = c,
                Command::IsBaseFont(base_font) => metric.is_base_font = base_font,
                Command::VVector(v1,v2) => metric.v_vector = (v1,v2),
                Command::IsFixedV(fixed) => metric.is_fixed_v = fixed,
                Command::CapHeight(height) => metric.cap_height = height,
                Command::XHeight(height) => metric.x_height = height,
                Command::Ascender(asc) => metric.ascender = asc,
                Command::Descender(desc) => metric.descender = desc,
                Command::StdHW(stdhw) => metric.standard_horizontal_width = stdhw,
                Command::StdVW(stdvw) => metric.standard_vertical_width = stdvw,
                Command::Comment(comment) => metric.comments.push(comment),
                Command::UnderlinePosition(pos) => metric.underline_position = pos,
                Command::UnderlineThickness(thickness) => metric.underline_thickness = thickness,
                Command::ItalicAngle(angle) => metric.italic_angle = angle,
                Command::CharWidth(w1, w2) => metric.char_width = (w1, w2),
                Command::IsFixedPitch(fixed) => metric.is_fixed_pitch = fixed,
                Command::CharMetrics(char_metrics) => metric.char_metrics = char_metrics,
                Command::Composites(composites) => metric.composites = composites,
                Command::KernData(cmds) => {
                    for cmd in cmds {
                        match cmd {
                            KernDataCmd::TrackKern(kerns) => metric.track_kern = kerns,
                            KernDataCmd::Kernpairs(pairs) => metric.kern_pairs = pairs,
                            KernDataCmd::Kernpairs0(pairs) => metric.kern_pairs0 = pairs,
                            KernDataCmd::Kernpairs1(pairs) => metric.kern_pairs1 = pairs
                        }
                    }
                }
            }
            metric
        }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pom::DataInput;
    use std::path::Path;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn parse_demo_file() {
        let input = br#"StartFontMetrics 4.1
Comment UniqueID 43050
Comment Copyright (c) 1989, 1990, 1991, 1992, 1993, 1997 Adobe Systems Incorporated.  All Rights Reserved.
Comment Creation Date: Thu May  1 17:27:09 1997
Comment VMusage 39754 50779
FontName Courier
FullName Courier
FamilyName Courier
Weight Medium
ItalicAngle 0
IsFixedPitch true
CharacterSet ExtendedRoman
FontBBox -23 -250 715 805
UnderlinePosition -100
UnderlineThickness 50
Version 003.000
Notice Copyright (c) 1989, 1990, 1991, 1992, 1993, 1997 Adobe Systems Incorporated.  All Rights Reserved.
EncodingScheme AdobeStandardEncoding
CapHeight 562
XHeight 426
Ascender 629
Descender -157
StdHW 51
StdVW 51
StartCharMetrics 5
C 32 ; WX 600 ; N space ; B 0 0 0 0 ;
C 33 ; WX 600 ; N exclam ; B 236 -15 364 572 ;
C 34 ; WX 600 ; N quotedbl ; B 187 328 413 562 ;
C 35 ; WX 600 ; N numbersign ; B 93 -32 507 639 ;
C 36 ; WX 600 ; N dollar ; B 105 -126 496 662 ;
EndCharMetrics
StartKernData
StartKernPairs 3
KPX A Cacute -30
KPX ecaron period -15
KPX zdotaccent ohungarumlaut -15
EndKernPairs
EndKernData
EndFontMetrics
"#;
        let mut buf = DataInput::new(input);
        let parser = afm().parse(&mut buf);
        assert!(parser.is_ok());
        let r = parser.unwrap();
        assert_eq!(r.font_name, "Courier");
        assert_eq!(r.family_name, "Courier");
        assert_eq!(r.full_name, "Courier");
        assert_eq!(r.font_version, "003.000");
        assert_eq!(r.comments[0], "UniqueID 43050");
        assert_eq!(r.comments.len(), 4);
        assert_eq!(r.weight, "Medium");
        assert_eq!(r.italic_angle, 0.0);
        assert_eq!(r.is_fixed_pitch, true);
        assert_eq!(r.character_set, "ExtendedRoman");
        assert_eq!(r.underline_position, -100.0);
        assert_eq!(r.underline_thickness, 50.0);
        assert_eq!(r.font_version, "003.000");
        assert_eq!(r.notice.len(), 98);
        assert_eq!(r.cap_height, 562.0);
        assert_eq!(r.x_height, 426.0);
        assert_eq!(r.ascender, 629.0);
        assert_eq!(r.descender, -157.0);
        assert_eq!(r.standard_horizontal_width, 51.0);
        assert_eq!(r.standard_vertical_width, 51.0);
        assert_eq!(r.char_metrics.len(), 5);
        assert_eq!(r.kern_pairs.len(), 3);
    }

    #[test]
    fn parse_standard_14_pdf_fonts() {
        use std::fs;
        use std::ffi::OsStr;

        let assets_dir = Path::new("assets");
        for file in fs::read_dir(assets_dir).unwrap() {
            let path = file.expect("I/O error when reading a asset").path();
            if path.is_file() && path.extension() == Some(OsStr::new("afm")) {
                println!("Parse file {}", path.display());
                let mut file = File::open(&path).expect("Could not open an asset file");
                let mut v = Vec::new();
                file.read_to_end(&mut v).expect(&format!("Could not read {}", path.display()));

                let mut buf = DataInput::new(&v);
                let parse_result = afm().parse(&mut buf);
                assert!(parse_result.is_ok(), "Could not parse the asset file {}", path.display());
            }
        }
    }
}
