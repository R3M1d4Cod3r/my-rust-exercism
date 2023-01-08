use enum_iterator::{all,Sequence};
use int_enum::IntEnum;
#[repr(u32)]
#[derive(Clone, Copy,Debug, PartialEq,Sequence,IntEnum)]
pub enum ResistorColor {
    Black=0,
    Brown=1,
    Red=2,
    Orange=3,
    Yellow=4,
    Green=5,
    Blue=6,
    Violet=7,
    Grey=8,
    White=9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
     _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
     match value{
    0 =>format!("{:?}",ResistorColor::Black ),
    1 =>format!("{:?}",ResistorColor::Brown ),
    2 =>format!("{:?}",ResistorColor::Red ),
    3 =>format!("{:?}",ResistorColor::Orange ),
    4 =>format!("{:?}",ResistorColor::Yellow ),
    5 =>format!("{:?}",ResistorColor::Green ),
    6 =>format!("{:?}",ResistorColor::Blue ),
    7 =>format!("{:?}",ResistorColor::Violet ),
    8 =>format!("{:?}",ResistorColor::Grey ),
    9 =>format!("{:?}",ResistorColor::White ),
    10_u32.. => String::from("value out of range"),
    }


    
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().into_iter().collect::<Vec<_>>()
}
