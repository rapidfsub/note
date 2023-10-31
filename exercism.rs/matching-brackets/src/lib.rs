pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for bracket in string.chars().filter_map(Bracket::new) {
        match bracket.open_close {
            OpenClose::Open => stack.push(bracket),
            OpenClose::Close => match stack.pop().map(|last| last.is_matching(&bracket)) {
                Some(true) => continue,
                _ => return false,
            },
        }
    }
    stack.is_empty()
}

#[derive(PartialEq)]
enum BracketStyle {
    Round,
    Square,
    Curly,
}

#[derive(PartialEq)]
enum OpenClose {
    Open,
    Close,
}

struct Bracket {
    style: BracketStyle,
    open_close: OpenClose,
}

impl Bracket {
    fn new(letter: char) -> Option<Self> {
        match letter {
            '(' => Some(Bracket {
                style: BracketStyle::Round,
                open_close: OpenClose::Open,
            }),
            ')' => Some(Bracket {
                style: BracketStyle::Round,
                open_close: OpenClose::Close,
            }),
            '[' => Some(Bracket {
                style: BracketStyle::Square,
                open_close: OpenClose::Open,
            }),
            ']' => Some(Bracket {
                style: BracketStyle::Square,
                open_close: OpenClose::Close,
            }),
            '{' => Some(Bracket {
                style: BracketStyle::Curly,
                open_close: OpenClose::Open,
            }),
            '}' => Some(Bracket {
                style: BracketStyle::Curly,
                open_close: OpenClose::Close,
            }),
            _ => None,
        }
    }

    fn is_matching(&self, other: &Self) -> bool {
        self.style == other.style && self.open_close != other.open_close
    }
}
