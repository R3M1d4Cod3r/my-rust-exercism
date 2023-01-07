use enum_iterator::{all};
#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    //unimplemented!("convert a color into a numerical representation");
    let mut result=0;
    let tmp = all::<ResistorColor>().collect::<Vec<_>>();
    for value in tmp{
        if value == ResistorColor{
            return result;
        }
        result+=1;
    }
}

pub fn value_to_color_string(value: u32) -> String {
    unimplemented!(
        "convert the value {} into a string representation of color",
        value
    )
}

pub fn colors() -> Vec<ResistorColor> {
    unimplemented!("return a list of all the colors ordered by resistance")
}
