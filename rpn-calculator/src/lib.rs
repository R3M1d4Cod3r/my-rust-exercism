#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut buffer = Vec::new();
    for el in inputs{
        match el {
            CalculatorInput::Add         =>  {if buffer.len() > 1 {let a = buffer.pop().unwrap(); let b = buffer.pop().unwrap(); buffer.push(b+a);} else {return None;}},
            CalculatorInput::Subtract    =>  {if buffer.len() > 1 {let a = buffer.pop().unwrap(); let b = buffer.pop().unwrap(); buffer.push(b-a);} else {return None;}},
            CalculatorInput::Multiply    =>  {if buffer.len() > 1 {let a = buffer.pop().unwrap(); let b = buffer.pop().unwrap(); buffer.push(b*a);} else {return None;}},
            CalculatorInput::Divide      =>  {if buffer.len() > 1 {let a = buffer.pop().unwrap(); let b = buffer.pop().unwrap(); buffer.push(b/a);} else {return None;}},
            CalculatorInput::Value(x)    =>  {buffer.push(*x);},

        }
    }

    if buffer.len()==1 {buffer.pop()} else {None}
}
