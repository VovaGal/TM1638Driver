// mapped segments to bits, digits to image, cahracters to image so i dont have to write it in bits in other files

//to do:
//add lower case letters

//       A
//      ---
//  F  |   |  B
//      -G-
//  E  |   |  C
//      ---
//       D

//    0x3f,    // 0 0b00111111
//    0x06,    // 1 0b00000110
//    0x5b,    // 2 0b01011011
//    0x4f,    // 3 0b01001111
//    0x66,    // 4 0b01100110
//    0x6d,    // 5 0b01101101
//    0x7d,    // 6 0b01111101
//    0x07,    // 7 0b00000111
//    0x7f,    // 8 0b01111111
//    0x6f,    // 9 0b01101111
//    0x77,    // A 0b01110111
//    0x7c,    // b 0b01111100
//    0x39,    // C 0b00111001
//    0x5e,    // d 0b01011110
//    0x79,    // E 0b01111001
//    0x71,    // F 0b01110001
//    0x40,    // - 0b01000000
//    0x00,    // nothing 0b00000000
//    0x80     // dot 

// map segments to the bit
#[repr(u8)]
#[derive(Debug)]
pub enum SegmentBits {
    // A segment
    SegA = 0b00000001,
    // B segment
    SegB = 0b00000010,
    // C segment
    SegC = 0b00000100,
    // D segment
    SegD = 0b00001000,
    // E segment
    SegE = 0b00010000,
    // F segment
    SegF = 0b00100000,
    // G segment
    SegG = 0b01000000,
    // Dec point
    DecPoint = 0b10000000,
}    

// map digit to its image
#[repr(u8)]
#[derive(Debug)]
pub enum NumCharBits {
    // 0
    Zero = 0b00111111,
    // 1
    One = 0b00000110,
    // 2
    Two = 0b01011011,
    // 3
    Three = 0b01001111,
    // 4
    Four = 0b01100110,
    // 5
    Five = 0b01101101,
    // 6
    Six = 0b01111101,
    // 7
    Seven = 0b00000111,
    // 8
    Eight = 0b01111111,
    // 9
    Nine = 0b01101111,
}

// map characters to image
#[repr(u8)]
#[derive(Debug)]
pub enum CharBits {
    // Uppercase A
    UpA = 0x77,
    // Uppercase C
    UpC = 0x39,
    // Uppercase E
    UpE = 0x79,
    // Uppercase F    
    UpF = 0x71,
    // Uppercase G
    UpG = 0x3D,
    // Uppercase H
    UpH = 0x76,
    // Uppercase I
    UpI = 0x30,
    // Uppercase J
    UpJ = 0x1E,
    // Uppercase L
    UpL = 0x38,
    // Uppercase O
    UpO = 0x3F,
    // Uppercase P
    UpP = 0x73,
    // Uppercase S
    UpS = 0x6D,
    // Uppercase U
    UpU = 0x3E,
}

//special chars map
#[repr(u8)]
#[derive(Debug)]
pub enum SpecialCharBits {
    // Space symbol
    Space = 0,
    // Dash symbol
    Minus = SegmentBits::SegG as u8,
    // Underscore
    Underscore = SegmentBits::SegD as u8,
    // Equal sign
    Equals = SegmentBits::SegG as u8 | SegmentBits::SegD as u8,
    // Question mark
    QuestionMark = SegmentBits::SegA as u8
        | SegmentBits::SegB as u8
        | SegmentBits::SegG as u8
        | SegmentBits::SegE as u8,
    // Dot
    Dot = SegmentBits::DecPoint as u8,
}