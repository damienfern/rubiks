#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    White,
    Blue,
    Yellow,
    Orange,
    Green
}

#[derive(Debug, Clone, Copy)]
enum Move {
    U,
    UPrime,
    F,
    FPrime,
    L,
    LPrime,
    R,
    RPrime,
    D,
    DPrime,
    B,
    BPrime
}

#[derive(Debug)]
struct Face {
    top_left: Color,
    top: Color, 
    top_right: Color, 
    center_left: Color, 
    center: Color, 
    center_right: Color, 
    bottom_left: Color, 
    bottom: Color, 
    bottom_right: Color
}

impl Face {
    #[warn(dead_code)]
    fn new(top_left: Color, top: Color, top_right: Color, center_left: Color, center: Color, center_right: Color, bottom_left: Color, bottom: Color, bottom_right: Color) -> Face {
        Face {
            top_left, top, top_right, center_left, center, center_right, bottom_left, bottom, bottom_right
        }
    }
    fn new_completed(color: Color) -> Face {
        Face {
            top_left: color, top: color, top_right: color, center_left: color, center: color, center_right: color, bottom_left: color, bottom: color, bottom_right: color
        }
    }
    
    fn set_color (&mut self, new_color: Color) {
        self.top_left = new_color;
    }
}

#[derive(Debug)]
enum TypeFace {
    Front,
    Left,
    Right,
    Back,
    Down,
    Up
}

#[derive(Debug)]
struct Cube {
    front: Face,
    left: Face,
    right: Face,
    back: Face,
    down: Face,
    up: Face
}

impl Cube {
    fn new (front: Face, left: Face, right: Face, back: Face, down: Face, up: Face) -> Self {
        Self {
            front,
            left,
            right,
            back,
            down,
            up
        }
    }
    fn execute_move (&self, type_face: TypeFace, move_to_execute: Move) {
        // face.top_left = Color::Yellow;
        let face = match type_face {
            TypeFace::Front => &self.front,
            TypeFace::Back => &self.back,
            TypeFace::Left => &self.left,
            TypeFace::Right => &self.right,
            TypeFace::Up => &self.up,
            TypeFace::Down => &self.down,
        };
        face.set_color(Color::Yellow);
        println!("{:?}, {:?}", face, move_to_execute);
    }
}

fn main() {
    let front_face = Face::new_completed(Color::Red);
    let left_face = Face::new_completed(Color::White);
    let right_face = Face::new_completed(Color::Yellow);
    let back_face = Face::new_completed(Color::Orange);
    let down_face = Face::new_completed(Color::Green);
    let up_face = Face::new_completed(Color::Blue);
    
    let cube = Cube::new(front_face, left_face, right_face, back_face, down_face, up_face);
    cube.execute_move(TypeFace::Front, Move::U);
    cube.execute_move(TypeFace::Left, Move::L);
    cube.execute_move(TypeFace::Right, Move::RPrime);
    // println!("{:?}", cube);
}
