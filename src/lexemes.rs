use crate::lexemes::State::{Character, Letter, Number, Separator};

#[derive(Debug)]
pub enum Error {
    IncorrectIdentifier(Location),
    IncorrectKeyword(Location),
    IncorrectOperator(Location),
}

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub enum TokenType {
    Keyword,
    Identifier,
    Operator,
    ConstValue,
    StringLiteral,
    Separator,
}

#[derive(Debug)]
enum State {
    // Intermediate
    /// \t, spaces, \n
    Whitespace,
    /// '(',')','[',']', '{', '}', ';'
    Separator(char),
    /// Generic state for letters
    Letter(char),
    /// Fallback state for characters
    Character(char),
    /// Generic state for numbers
    Number(char),
    /// _, used for variable name starts
    Underscore,

    KeywordEnd,
    // Operators:
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// <<
    Shl,
    /// >>
    Shr,
    /// &&
    And,
    /// ||
    Or,
    /// ^
    BitXor,
    /// &
    BitAnd,
    /// |
    BitOr,
    /// ++
    Incr,
    /// --
    Decr,
    // Comparison operators:
    /// \<
    LT,
    /// \>
    GT,
    /// <=
    LE,
    /// >=
    GE,
    /// ==
    Eq,
    /// !=
    NEq,
    /// !
    Neg,
    // Assign operators:
    /// =
    Assign,
    /// +=
    AddAssign,
    /// -=
    SubAssign,
    /// *=
    MulAssign,
    /// /=
    DivAssign,
    /// %=
    ModAssign,
    /// <\<\=
    ShlAssign,
    /// \>\>\=
    ShrAssign,
    /// &=
    BitAndAssign,
    /// |=
    BitOrAssign,
    // Special operators:
    /// ,
    Comma,
    /// ->
    Arrow,

    // Keywords
    /// asm
    AsmS,
    AsmM,
    /// auto
    AutoU,
    AutoT,
    AutoO,
    /// bool
    BoolO,
    BoolO2,
    BoolL,
    /// break
    BreakR,
    BreakE,
    BreakA,
    BreakK,
    /// case | catch
    CaA,
    /// case
    CaseS,
    CaseE,
    /// catch
    CatchT,
    CatchC,
    CatchH,
    /// char
    CharH,
    CharA,
    CharR,
    /// class
    ClassL,
    ClassA,
    ClassS,
    ClassS2,
    /// compl | concept | const
    CoO,
    /// compl
    ComplM,
    ComplP,
    ComplL,
    /// concept | const
    ConN,
    /// concept
    ConceptC,
    ConceptE,
    ConceptP,
    ConceptT,
    /// const
    ConstS,
    ConstT,
    /// default | delete
    DeE,
    DefaultF,
    DefaultA,
    DefaultU,
    DefaultL,
    DefaultT,
    DeleteL,
    DeleteE,
    DeleteT,
    DeleteE2,
    /// do | double
    DoO,
    /// double
    DoubleU,
    DoubleB,
    DoubleL,
    DoubleE,
    /// else
    ElseL,
    ElseS,
    ElseE,
    /// enum
    EnumN,
    EnumU,
    EnumM,
    /// export | extern
    ExX,
    /// export
    ExportP,
    ExportO,
    ExportR,
    ExportT,
    /// extern
    ExternT,
    ExternE,
    ExternR,
    ExternN,
    /// false
    FalseA,
    FalseL,
    FalseS,
    FalseE,
    /// float
    FloatL,
    FloatO,
    FloatA,
    FloatT,
    /// for
    ForO,
    ForR,
    /// friend
    FriendR,
    FriendI,
    FriendE,
    FriendN,
    FriendD,
    /// goto
    GotoO,
    GotoT,
    GotoO2,
    /// if
    IfF,
    /// int | inline
    InN,
    /// inline
    InlineN,
    InlineL,
    InlineI,
    InlineN2,
    InlineE,
    /// int
    IntN,
    IntT,
    /// long
    LongO,
    LongN,
    LongG,
    /// mutable
    MutableU,
    MutableT,
    MutableA,
    MutableB,
    MutableL,
    MutableE,
    /// namespace
    NamespaceA,
    NamespaceM,
    NamespaceE,
    NamespaceS,
    NamespaceP,
    NamespaceA2,
    NamespaceC,
    NamespaceE2,
    NewE,
    NewW,
    /// nullptr
    NullptrU,
    NullptrL,
    NullptrL2,
    NullptrP,
    NullptrT,
    NullptrR,
    /// operator
    OperatorP,
    OperatorE,
    OperatorR,
    OperatorA,
    OperatorT,
    OperatorO,
    OperatorR2,
    /// private | protected
    PrR,
    /// private
    PrivateR,
    PrivateI,
    PrivateV,
    PrivateA,
    PrivateT,
    PrivateE,
    /// protected
    ProtectedR,
    ProtectedO,
    ProtectedT,
    ProtectedE,
    ProtectedC,
    ProtectedT2,
    ProtectedE2,
    ProtectedD,
    /// public
    PublicU,
    PublicB,
    PublicL,
    PublicI,
    PublicC,
    /// return
    ReturnE,
    ReturnT,
    ReturnU,
    ReturnR,
    ReturnN,
    /// short
    ShortH,
    ShortO,
    ShortR,
    ShortT,
    /// signed || sizeof
    SiI,
    /// signed
    SignedG,
    SignedN,
    SignedE,
    SignedD,
    /// sizeof
    SizeofZ,
    SizeofE,
    SizeofO,
    SizeofF,
    /// static | struct
    StT,
    /// static
    StaticA,
    StaticT,
    StaticI,
    StaticC,
    /// struct
    StructR,
    StructU,
    StructC,
    StructT,
    /// switch
    SwitchW,
    SwitchI,
    SwitchT,
    SwitchC,
    SwitchH,
    /// template
    TemplateE,
    TemplateM,
    TemplateP,
    TemplateL,
    TemplateA,
    TemplateT,
    TemplateE2,
    /// this | throw
    ThH,
    /// this
    ThisI,
    ThisS,
    /// throw
    ThrowR,
    ThrowO,
    ThrowW,
    /// true | try
    TrR,
    /// true
    TrueU,
    TrueE,
    /// try
    TryY,
    /// union | unsigned
    UnN,
    /// union
    UnionI,
    UnionO,
    UnionN,
    /// unsigned
    UnsignedS,
    UnsignedI,
    UnsignedG,
    UnsignedN,
    UnsignedE,
    UnsignedD,
    /// using
    UsingS,
    UsingI,
    UsingN,
    UsingG,
    /// virtual
    VirtualI,
    VirtualR,
    VirtualT,
    VirtualU,
    VirtualA,
    VirtualL,
    /// void
    VoidO,
    VoidI,
    VoidD,
    /// while
    WhileH,
    WhileI,
    WhileL,
    WhileE,
}

#[derive(Clone, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub char: char,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub token: String,
}

pub fn count_tokens(text: String) -> Result<Vec<Token>, Error> {
    let mut tokens = vec![];
    let mut location: Location = Location {
        line: 0,
        column: 0,
        char: ' ',
    };
    let mut state: State = State::Whitespace;
    let mut buff: String = String::new();
    let mut is_writable: bool = false;
    let mut current_idx: usize = 0;
    let mut current: char;
    while current_idx < text.chars().count() {
        current = text.as_str().as_bytes()[current_idx] as char;
        location.char = current;
        print!("{current}");
        if current == '\n' {
            location.line += 1;
            location.column += 1;
        }
        match state {
            State::Whitespace |
            State::KeywordEnd
            => {
                buff.push(current);
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    '+' => State::Add,
                    '-' => State::Sub,
                    '*' => State::Mul,
                    '/' => State::Div,
                    '%' => State::Mod,
                    '<' => State::LT,
                    '>' => State::GT,
                    '=' => State::Assign,
                    '!' => State::Neg,
                    '&' => State::BitAnd,
                    '|' => State::BitOr,
                    '^' => State::BitXor,
                    ',' => State::Comma,
                    '_' => State::Underscore,
                    c if c.is_alphabetic() => Letter(c),
                    c if c.is_numeric() => Number(c),
                    '(' | ')' | '[' | ']' | '{' | '}' | ';' => Separator(current),
                    _ => Character(current)
                }
            }
            State::Separator(char) => {
                state = match current {

                }
            }
            State::Number(char) => {
                state = match current {
                }
            }
            State::Underscore => {
                buff.push(current);
                state = match current {
                    c if c.is_alphabetic() => Letter(c),
                    ' ' | '\n' | '\t' => {
                        is_writable = true;
                        State::Whitespace
                    },
                    _ => return Err(Error::IncorrectIdentifier(location))
                }
            }
            State::Add => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        State::AddAssign
                    },
                    '+' => {
                        is_writable = true;
                        State::Incr
                    }
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::Sub => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        State::SubAssign
                    }
                    '-' => {
                        is_writable = true;
                        State::Decr
                    }
                    '>' => {
                        is_writable = true;
                        State::Arrow
                    }
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::Mul => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        State::MulAssign
                    }
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::Div => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        State::DivAssign
                    }
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::Mod => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        State::ModAssign
                    }
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::Shl => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        State::ShlAssign
                    }
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::Shr => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        State::ShrAssign
                    }
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::And => {
                buff.push(current);
                state = match current {
                    '&' => {
                        is_writable = true;
                        State::BitAnd
                    }
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::Or => {
                buff.push(current);
                state = match current {
                    '|' => {
                        is_writable = true;
                        State::BitOr
                    }
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::BitXor => {
                buff.push(current);
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::BitAnd => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        State::BitAndAssign
                    },
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::BitOr => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        State::BitOrAssign
                    },
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::Incr => {
                buff.push(current);
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::Decr => {
                buff.push(current);
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::LT => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        State::LE
                    },
                    '<' => State::Shl,
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::GT => {
                buff.push(current);
                state = match current {
                    '=' => State::GE,
                    '>' => State::Shr,
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => Character(current)
                }
            }
            State::LE => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::GE => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::Eq => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::NEq => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::Neg => {
                buff.push(current);
                state = match current {
                    '=' => {
                        is_writable = true;
                        state = State::NEq
                    }
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        Character(current)
                    }
                }
            }
            State::Assign => {
                buff.push(current);
                state = match current {
                    '=' => State::Eq,
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::AddAssign => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::SubAssign => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::MulAssign => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::DivAssign => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::ModAssign => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::ShlAssign => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::ShrAssign => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::BitAndAssign => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::BitOrAssign => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::Comma => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::Arrow => {
                state = match current {
                    ' ' | '\n' | '\t' => State::Whitespace,
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::Letter('a') => {
                buff.push(current);
                state = match current {
                    's' => State::AsmS,
                    'u' => State::AutoU,
                    ' ' | '\n' | '\t' => {
                        is_writable = true;
                        State::Whitespace
                    },
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::AsmS => {
                buff.push(current);
                state = match current {
                    'm' => {
                        State::AsmM
                    },
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        Letter(c)
                    },
                    ' ' | '\n' | '\t' => {
                        State::Whitespace
                    },
                    '('|')'|'['|']'| '{'| '}'| ';' => {
                        State::Separator(current)
                    }
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::AsmM => {
                state = match current {
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        Letter(c)
                    },
                    ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
                        current_idx -= 1;
                        State::KeywordEnd
                    },
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::AutoU => {
                state = match current {
                    't' => {
                        buff.push(current);
                        State::AutoT
                    },
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        Letter(c)
                    },
                    ' ' | '\n' | '\t' => {
                        State::Whitespace
                    },
                    '('|')'|'['|']'| '{'| '}'| ';' => {
                        State::Separator(current)
                    }
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::AutoT => {
                buff.push(current);
                state = match current {
                    'o' => State::AutoO,
                    c if c.is_alphanumeric() => Letter(c),
                    ' ' | '\n' | '\t' => {
                        State::Whitespace
                    },

                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::AutoO => {
                state = match current {
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        Letter(c)
                    },
                    ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
                        current_idx -= 1;
                        State::KeywordEnd
                    },
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::Letter('b') => {
                buff.push(current);
                state = match current {
                    'o' => State::BoolO,
                    'r' => State::BreakR,
                    ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
                        State::Whitespace
                    },
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::BoolO => {
                buff.push(current);
                state = match current {
                    'o' => State::BoolO2,
                    c if c.is_alphanumeric() => Letter(c),
                    ' ' | '\n' | '\t' => {
                        State::Whitespace
                    },

                    _ => {
                        Character(current)
                    }
                }
            }
            State::BoolO2 => {
                buff.push(current);
                state = match current {
                    'l' => State::BoolL,
                    c if c.is_alphanumeric() => Letter(c),
                    ' ' | '\n' | '\t' => {
                        State::Whitespace
                    },

                    _ => {
                        Character(current)
                    }
                }
            }
            State::BoolL => {
                state = match current {
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        Letter(c)
                    },
                    ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
                        current_idx -= 1;
                        State::KeywordEnd
                    },
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::BreakR => {
                buff.push(current);
                state = match current {
                    'e' => State::BreakE,
                    c if c.is_alphanumeric() => Letter(c),
                    ' ' | '\n' | '\t' => {
                        State::Whitespace
                    },

                    _ => {
                        Character(current)
                    }
                }
            }
            State::BreakE => {
                buff.push(current);
                state = match current {
                    'a' => State::BreakA,
                    c if c.is_alphanumeric() => Letter(c),
                    ' ' | '\n' | '\t' => {
                        State::Whitespace
                    },

                    _ => {
                        Character(current)
                    }
                }
            }
            State::BreakA => {
                buff.push(current);
                state = match current {
                    'k' => State::BreakK,
                    c if c.is_alphanumeric() => Letter(c),
                    ' ' | '\n' | '\t' => {
                        State::Whitespace
                    },

                    _ => {
                        Character(current)
                    }
                }
            }
            State::BreakK => {
                state = match current {
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        Letter(c)
                    },
                    ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
                        current_idx -= 1;
                        State::KeywordEnd
                    },
                    _ => {
                        buff.push(current);
                        Character(current)
                    }
                }
            }
            State::CaA => {
                buff.push(current);
                state = match current {
                    's' => State::CaseS,
                    't' => State::CatchT,
                    c if c.is_alphanumeric() => Letter(c),
                    ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
                        State::Whitespace
                    }
                    _ => Character(current)

                }
            }
            State::CaseS => {
                buff.push(current);
                state = match current {
                    'e' => State::CaseE,
                    c if c.is_alphanumeric() => Letter(c),
                    ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
                        State::Whitespace
                    }
                    _ => Character(current)
                }
            }
            State::CaseE => {
                state = match current {
                    c if c.is_alphanumeric() => Letter(c),
                    ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
                        State::KeywordEnd
                    }
                    _ => Character(current)
                }
            }
            State::CatchT => {
                state = match current {
                }
            }
            State::CatchC => {
                state = match current {
                }
            }
            State::CatchH => {
                state = match current {
                }
            }
            State::CharH => {
                state = match current {
                }
            }
            State::CharA => {
                state = match current {
                }
            }
            State::CharR => {
                state = match current {
                }
            }
            State::ClassL => {
                state = match current {
                }
            }
            State::ClassA => {
                state = match current {
                }
            }
            State::ClassS => {
                state = match current {
                }
            }
            State::ClassS2 => {
                state = match current {
                }
            }
            State::CoO => {
                state = match current {
                }
            }
            State::ComplM => {
                state = match current {
                }
            }
            State::ComplP => {
                state = match current {
                }
            }
            State::ComplL => {
                state = match current {
                }
            }
            State::ConN => {
                state = match current {
                }
            }
            State::ConceptC => {
                state = match current {
                }
            }
            State::ConceptE => {
                state = match current {
                }
            }
            State::ConceptP => {
                state = match current {
                }
            }
            State::ConceptT => {
                state = match current {
                }
            }
            State::ConstS => {
                state = match current {
                }
            }
            State::ConstT => {
                state = match current {
                }
            }
            State::DeE => {
                state = match current {
                }
            }
            State::DefaultF => {
                state = match current {
                }
            }
            State::DefaultA => {
                state = match current {
                }
            }
            State::DefaultU => {
                state = match current {
                }
            }
            State::DefaultL => {
                state = match current {
                }
            }
            State::DefaultT => {
                state = match current {
                }
            }
            State::DeleteL => {
                state = match current {
                }
            }
            State::DeleteE => {
                state = match current {
                }
            }
            State::DeleteT => {
                state = match current {
                }
            }
            State::DeleteE2 => {
                state = match current {
                }
            }
            State::DoO => {
                state = match current {
                }
            }
            State::DoubleU => {
                state = match current {
                }
            }
            State::DoubleB => {
                state = match current {
                }
            }
            State::DoubleL => {
                state = match current {
                }
            }
            State::DoubleE => {
                state = match current {
                }
            }
            State::ElseL => {
                state = match current {
                }
            }
            State::ElseS => {
                state = match current {
                }
            }
            State::ElseE => {
                state = match current {
                }
            }
            State::EnumN => {
                state = match current {
                }
            }
            State::EnumU => {
                state = match current {
                }
            }
            State::EnumM => {
                state = match current {
                }
            }
            State::ExX => {
                state = match current {
                }
            }
            State::ExportP => {
                state = match current {
                }
            }
            State::ExportO => {
                state = match current {
                }
            }
            State::ExportR => {
                state = match current {
                }
            }
            State::ExportT => {
                state = match current {
                }
            }
            State::ExternT => {
                state = match current {
                }
            }
            State::ExternE => {
                state = match current {
                }
            }
            State::ExternR => {
                state = match current {
                }
            }
            State::ExternN => {
                state = match current {
                }
            }
            State::FalseA => {
                state = match current {
                }
            }
            State::FalseL => {
                state = match current {
                }
            }
            State::FalseS => {
                state = match current {
                }
            }
            State::FalseE => {
                state = match current {
                }
            }
            State::FloatL => {
                state = match current {
                }
            }
            State::FloatO => {
                state = match current {
                }
            }
            State::FloatA => {
                state = match current {
                }
            }
            State::FloatT => {
                state = match current {
                }
            }
            State::ForO => {
                state = match current {
                }
            }
            State::ForR => {
                state = match current {
                }
            }
            State::FriendR => {
                state = match current {
                }
            }
            State::FriendI => {
                state = match current {
                }
            }
            State::FriendE => {
                state = match current {
                }
            }
            State::FriendN => {
                state = match current {
                }
            }
            State::FriendD => {
                state = match current {
                }
            }
            State::GotoO => {
                state = match current {
                }
            }
            State::GotoT => {
                state = match current {
                }
            }
            State::GotoO2 => {
                state = match current {
                }
            }
            State::IfF => {
                state = match current {
                }
            }
            State::InN => {
                state = match current {
                }
            }
            State::InlineN => {
                state = match current {
                }
            }
            State::InlineL => {
                state = match current {
                }
            }
            State::InlineI => {
                state = match current {
                }
            }
            State::InlineN2 => {
                state = match current {
                }
            }
            State::InlineE => {
                state = match current {
                }
            }
            State::IntN => {
                state = match current {
                }
            }
            State::IntT => {
                state = match current {
                }
            }
            State::LongO => {
                state = match current {
                }
            }
            State::LongN => {
                state = match current {
                }
            }
            State::LongG => {
                state = match current {
                }
            }
            State::MutableU => {
                state = match current {
                }
            }
            State::MutableT => {
                state = match current {
                }
            }
            State::MutableA => {
                state = match current {
                }
            }
            State::MutableB => {
                state = match current {
                }
            }
            State::MutableL => {
                state = match current {
                }
            }
            State::MutableE => {
                state = match current {
                }
            }
            State::NamespaceA => {
                state = match current {
                }
            }
            State::NamespaceM => {
                state = match current {
                }
            }
            State::NamespaceE => {
                state = match current {
                }
            }
            State::NamespaceS => {
                state = match current {
                }
            }
            State::NamespaceP => {
                state = match current {
                }
            }
            State::NamespaceA2 => {
                state = match current {
                }
            }
            State::NamespaceC => {
                state = match current {
                }
            }
            State::NamespaceE2 => {
                state = match current {
                }
            }
            State::NewE => {
                state = match current {
                }
            }
            State::NewW => {
                state = match current {
                }
            }
            State::NullptrU => {
                state = match current {
                }
            }
            State::NullptrL => {
                state = match current {
                }
            }
            State::NullptrL2 => {
                state = match current {
                }
            }
            State::NullptrP => {
                state = match current {
                }
            }
            State::NullptrT => {
                state = match current {
                }
            }
            State::NullptrR => {
                state = match current {
                }
            }
            State::OperatorP => {
                state = match current {
                }
            }
            State::OperatorE => {
                state = match current {
                }
            }
            State::OperatorR => {
                state = match current {
                }
            }
            State::OperatorA => {
                state = match current {
                }
            }
            State::OperatorT => {
                state = match current {
                }
            }
            State::OperatorO => {
                state = match current {
                }
            }
            State::OperatorR2 => {
                state = match current {
                }
            }
            State::PrR => {
                state = match current {
                }
            }
            State::PrivateR => {
                state = match current {
                }
            }
            State::PrivateI => {
                state = match current {
                }
            }
            State::PrivateV => {
                state = match current {
                }
            }
            State::PrivateA => {
                state = match current {
                }
            }
            State::PrivateT => {
                state = match current {
                }
            }
            State::PrivateE => {
                state = match current {
                }
            }
            State::ProtectedR => {
                state = match current {
                }
            }
            State::ProtectedO => {
                state = match current {
                }
            }
            State::ProtectedT => {
                state = match current {
                }
            }
            State::ProtectedE => {
                state = match current {
                }
            }
            State::ProtectedC => {
                state = match current {
                }
            }
            State::ProtectedT2 => {
                state = match current {
                }
            }
            State::ProtectedE2 => {
                state = match current {
                }
            }
            State::ProtectedD => {
                state = match current {
                }
            }
            State::PublicU => {
                state = match current {
                }
            }
            State::PublicB => {
                state = match current {
                }
            }
            State::PublicL => {
                state = match current {
                }
            }
            State::PublicI => {
                state = match current {
                }
            }
            State::PublicC => {
                state = match current {
                }
            }
            State::ReturnE => {
                state = match current {
                }
            }
            State::ReturnT => {
                state = match current {
                }
            }
            State::ReturnU => {
                state = match current {
                }
            }
            State::ReturnR => {
                state = match current {
                }
            }
            State::ReturnN => {
                state = match current {
                }
            }
            State::ShortH => {
                state = match current {
                }
            }
            State::ShortO => {
                state = match current {
                }
            }
            State::ShortR => {
                state = match current {
                }
            }
            State::ShortT => {
                state = match current {
                }
            }
            State::SiI => {
                state = match current {
                }
            }
            State::SignedG => {
                state = match current {
                }
            }
            State::SignedN => {
                state = match current {
                }
            }
            State::SignedE => {
                state = match current {
                }
            }
            State::SignedD => {
                state = match current {
                }
            }
            State::SizeofZ => {
                state = match current {
                }
            }
            State::SizeofE => {
                state = match current {
                }
            }
            State::SizeofO => {
                state = match current {
                }
            }
            State::SizeofF => {
                state = match current {
                }
            }
            State::StT => {
                state = match current {
                }
            }
            State::StaticA => {
                state = match current {
                }
            }
            State::StaticT => {
                state = match current {
                }
            }
            State::StaticI => {
                state = match current {
                }
            }
            State::StaticC => {
                state = match current {
                }
            }
            State::StructR => {
                state = match current {
                }
            }
            State::StructU => {
                state = match current {
                }
            }
            State::StructC => {
                state = match current {
                }
            }
            State::StructT => {
                state = match current {
                }
            }
            State::SwitchW => {
                state = match current {
                }
            }
            State::SwitchI => {
                state = match current {
                }
            }
            State::SwitchT => {
                state = match current {
                }
            }
            State::SwitchC => {
                state = match current {
                }
            }
            State::SwitchH => {
                state = match current {
                }
            }
            State::TemplateE => {
                state = match current {
                }
            }
            State::TemplateM => {
                state = match current {
                }
            }
            State::TemplateP => {
                state = match current {
                }
            }
            State::TemplateL => {
                state = match current {
                }
            }
            State::TemplateA => {
                state = match current {
                }
            }
            State::TemplateT => {
                state = match current {
                }
            }
            State::TemplateE2 => {
                state = match current {
                }
            }
            State::ThH => {
                state = match current {
                }
            }
            State::ThisI => {
                state = match current {
                }
            }
            State::ThisS => {
                state = match current {
                }
            }
            State::ThrowR => {
                state = match current {
                }
            }
            State::ThrowO => {
                state = match current {
                }
            }
            State::ThrowW => {
                state = match current {
                }
            }
            State::TrR => {
                state = match current {
                }
            }
            State::TrueU => {
                state = match current {
                }
            }
            State::TrueE => {
                state = match current {
                }
            }
            State::TryY => {
                state = match current {
                }
            }
            State::UnN => {
                state = match current {
                }
            }
            State::UnionI => {
                state = match current {
                }
            }
            State::UnionO => {
                state = match current {
                }
            }
            State::UnionN => {
                state = match current {
                }
            }
            State::UnsignedS => {
                state = match current {
                }
            }
            State::UnsignedI => {
                state = match current {
                }
            }
            State::UnsignedG => {
                state = match current {
                }
            }
            State::UnsignedN => {
                state = match current {
                }
            }
            State::UnsignedE => {
                state = match current {
                }
            }
            State::UnsignedD => {
                state = match current {
                }
            }
            State::UsingS => {
                state = match current {
                }
            }
            State::UsingI => {
                state = match current {
                }
            }
            State::UsingN => {
                state = match current {
                }
            }
            State::UsingG => {
                state = match current {
                }
            }
            State::VirtualI => {
                state = match current {
                }
            }
            State::VirtualR => {
                state = match current {
                }
            }
            State::VirtualT => {
                state = match current {
                }
            }
            State::VirtualU => {
                state = match current {
                }
            }
            State::VirtualA => {
                state = match current {
                }
            }
            State::VirtualL => {
                state = match current {
                }
            }
            State::VoidO => {
                state = match current {
                }
            }
            State::VoidI => {
                state = match current {
                }
            }
            State::VoidD => {
                state = match current {
                }
            }
            State::WhileH => {
                state = match current {
                }
            }
            State::WhileI => {
                state = match current {
                }
            }
            State::WhileL => {
                state = match current {
                }
            }
            State::WhileE => {
                state = match current {
                }
            }
        }
        current_idx += 1;
    }
    Ok(tokens)
}
