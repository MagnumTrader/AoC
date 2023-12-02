#![allow(unused)]

use std::{ops::{BitOr, Deref}, fmt::Binary, collections::HashMap, rc::Rc, cell::RefCell, borrow::BorrowMut};
fn main() {

    let file = match std::fs::read_to_string("./src/input.txt") {
        Ok(x) => x,
        Err(e) => panic!("could not read file: Error {e}"),
    };

    

    //Part 1 
    let testinput = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";


    //
    // TODO this is probably a graph problem.
    // learn more about graphs!
    //
    //
    let signal = Component::Signal(BitSignal16::from_u16(456));

    let mut wire = Wire {
        label: "y",
        input: Some(signal),
        output: None,
    };
    let mut gate = Gate{ 
        gate_type: GateType::And, 
        inputs: todo!(), 
        output: todo!() 
    };


    println!("{wire:?}");
    //Part 2
    //

}

#[derive(Debug)]
enum Component {
    Gate(Rc<RefCell<Component>>),
    Wire(Rc<RefCell<Component>>),
    /// Not a reference beacuse a signal is the beginning of a wire network
    Signal(BitSignal16)
}
impl Component {
    // Behövs när vi ska vända och updatera vårat nätverk
}
// should have pointers to
/// dependant on the type of gate, we need different inputs, 
#[derive(Debug)]
struct Gate {
    gate_type: GateType,
    inputs: (Option<Component>, Option<Component>),
    /// 
    /// Options becuase maybe not completed yet follow that trial back
    output: Option<Component>
}
impl Gate {
    /// Walk backwards first, then forwards, 
    fn walk(&self) {
        // shoudl take 
        match self.gate_type {
            GateType::And => todo!(), // walk both inputs to recieve the signals
            GateType::Or => todo!(),
            GateType::Not => todo!(), // NOT requires only one fields
            GateType::Shift(_) => todo!(), // Also requires one
        }
    }
}
#[derive(Debug)]
struct Wire {
    /// Label for keeping track of all my damn wires
    label: &'static str,
    input: Option<Component>,
    output: Option<Component>
}
impl Wire {
    fn new() -> Wire  {
        Self { 
            label: "", 
            input: None, 
            output: None 
        }
    }
    fn label(&mut self, label: &'static str) {
        self.label = label;
    }

    fn input(&mut self, input: Component) {
        self.input = Some(input);
    }

    fn output(mut self, output: Component) {
        
        if let Component::Signal(_) = output {
            panic!("output can never be signal")
        }
        self.output = Some(output);
    }

    /// Walk down the current to update all fields
    /// Walk down, then walk up du update if i have added a component somewhere
    fn update_current(&mut self) -> Result<(),String> {
        todo!();
        Ok(())
    }
    
}

#[derive(Debug)]
enum GateType {
    And,
    Or,
    Shift(Shift),
    Not
}
#[derive(Debug)]
enum Shift {
    Right(i16),
    Left(i16)
}

// TODO gör detta till en struct eller struct
type BitSignal16 = [u8;16];

impl Signal for BitSignal16 {
    fn new() -> Self {
        [0;16]
    }

    fn bitwise_or(&self, other:Self) -> Self {
        todo!()
    }

    fn bitwise_and(&self, other: Self) -> Self {
        todo!()
    }

    fn bitwise_not(&self, other: Self) -> Self {
        todo!()
    }

    fn bitwise_shift(&self, other: Shift) -> Self {
        todo!()
    }

    fn from_u16(i: u16) -> Self {

        let mut bitsignal = BitSignal16::new();

        let mut remainder = i;
        let mut division = i;
        let mut index = bitsignal.len() - 1;

        loop {

            remainder = division % 2;
            division = division / 2;

            if remainder != 0 { bitsignal[index] = 1 } else {};

            if (division == 0) && (remainder == 0) {
                break;
            }

            index -= 1;
        }

        return bitsignal
    }
}

trait Signal {

    fn new() -> Self;
    fn from_u16(i: u16) -> Self;
    fn bitwise_or(&self, other: Self) -> Self;
    fn bitwise_and(&self, other: Self) -> Self;
    fn bitwise_not(&self, other: Self) -> Self;
    fn bitwise_shift(&self, shift: Shift) -> Self;
}

/// Shift command, 

fn part1(someinput: i32) {

}

fn part2(someinput: i32) {

}

