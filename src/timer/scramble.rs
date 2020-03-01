extern crate rand;

use rand::Rng;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

use gtk::prelude::*;



#[derive(FromPrimitive, PartialEq, Clone)]
enum Face{
    U = 0, F, R, D, L, B
}

#[derive(FromPrimitive, PartialEq, Clone)]
enum Turn{
    Clockwise, AntiClockwise, Double,
}

#[derive(FromPrimitive, PartialEq, Clone)]
enum Color{
    White, Yellow, Green, Blue, Orange, Red
}

#[derive(Clone)]
pub struct Move{
    face: Face,
    turn: Turn,
    layers: u32,
}

impl Move{
    fn to_string(&self) -> String{
        let face_str = match &self.face{
            Face::U => "U",
            Face::D => "D",
            Face::F => "F",
            Face::B => "B",
            Face::L => "L",
            Face::R => "R",
        };

        let turn_str = match &self.turn{
            Turn::Clockwise => "",
            Turn::AntiClockwise => "'",
            Turn::Double => "2",
        };

        let result = match &self.layers{
            1 => format!("{}{}", face_str, turn_str),
            2 => format!("{}w{}", face_str, turn_str),
            _ => format!("{}{}w{}",self.layers, face_str, turn_str)
        };
        
        return result;
        //return format!("{}{}", face_str, turn_str);
    }
}

#[derive(Clone)]
pub struct Scramble{
    pub scramble : Vec<Move>,
    pub label : gtk::Label,
    pub length : u32,
    pub max_layers : u32,
    pub faces : i32,
}

impl Scramble{
    fn new(_length : u32, _max_layers : u32, _faces : i32) -> Self{
        let mut result = Scramble{scramble : Vec::new(),
        label : gtk::Label::new(None),
        length : _length,
        max_layers : _max_layers,
        faces : _faces
        };
        result.label.set_line_wrap(true);
        result.generate();
        result.show();

        return result;
    }

    pub fn new2x2() -> Self{
        Scramble::new(10, 1, 3)
    }

    pub fn new3x3() -> Self{
        Scramble::new(20, 1, 6)
    }

    pub fn new4x4() -> Self{
        Scramble::new(40, 2, 6)
    }

    pub fn to2x2(&mut self){
        self.length = 10;
        self.faces = 3;
        self.max_layers = 1;
        self.generate();
        self.show();
    }

    pub fn to3x3(&mut self){
        self.length = 20;
        self.faces = 6;
        self.max_layers = 1;
        self.generate();
        self.show();
    }

    pub fn to4x4(&mut self){
        self.length = 40;
        self.faces = 6;
        self.max_layers = 2;
        self.generate();
        self.show();
    }

    pub fn generate(&mut self){
        self.scramble = Vec::new();
        let mut rng = rand::thread_rng();
        let mut last_face : Face = FromPrimitive::from_i32(rng.gen_range(0,self.faces)).unwrap();
        let mut face : Face = last_face.clone();
        let mut turn : Turn;
        let mut layers : u32;

        for _ in 0..self.length{
            while face == last_face{
                face = FromPrimitive::from_i32(rng.gen_range(0,self.faces)).unwrap();
            }
            turn = FromPrimitive::from_i32(rng.gen_range(0,3)).unwrap();
            layers = rng.gen_range(1, self.max_layers+1);
            self.scramble.push(Move{face : face.clone(), turn : turn.clone(), layers});
            last_face = face.clone();
        }
    }

    pub fn to_string(&self) -> String{
        self.scramble.iter().map(|m| m.to_string()).collect::<Vec<String>>().join(" ")
    }

    pub fn show(&mut self){
        // self.generate();
        let text = self.to_string();
        self.label.set_markup(format!("<span font_desc='Ubuntu 20'>{}</span>", text).as_str());
    }

    pub fn reset(&mut self){
        self.generate();
        self.show();
    }
}