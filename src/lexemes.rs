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
    None,
}

#[derive(Debug)]
enum State {
    // Intermediate
    /// \t, spaces, \n
    Whitespace,
    Base,
    /// '(',')','[',']', '{', '}', ';'
    Separator(char),
    /// Generic state for letters
    Letter(char),
    /// State for identifier characters
    Identifier(char),
    /// Fallback state for characters
    Character(char),
    /// Generic state for numbers
    Number(char),
    /// _, used for variable name starts
    Underscore,

    OperatorEnd,
    KeywordEnd,
    StringLiteral(char),
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
    /// auto
    AutoU,
    AutoT,
    /// bool
    BoolO,
    BoolO2,
    /// break
    BreakR,
    BreakE,
    BreakA,
    /// case | catch
    CaA,
    /// case
    CaseS,
    /// catch
    CatchT,
    CatchC,
    /// char
    CharH,
    CharA,
    /// class
    ClassL,
    ClassA,
    ClassS,
    /// compl | concept | const
    CoO,
    /// compl
    ComplM,
    ComplP,
    /// concept | const
    ConN,
    /// concept
    ConceptC,
    ConceptE,
    ConceptP,
    /// const
    ConstS,
    /// default | delete
    DeE,
    DefaultF,
    DefaultA,
    DefaultU,
    DefaultL,
    DeleteL,
    DeleteE,
    DeleteT,
    /// do | double
    DoO,
    /// double
    DoubleU,
    DoubleB,
    DoubleL,
    /// else
    ElseL,
    ElseS,
    /// enum
    EnumN,
    EnumU,
    /// export | extern
    ExX,
    /// export
    ExportP,
    ExportO,
    ExportR,
    /// extern
    ExternT,
    ExternE,
    ExternR,
    /// false
    FalseA,
    FalseL,
    FalseS,
    /// float
    FloatL,
    FloatO,
    FloatA,
    /// for
    ForO,
    /// friend
    FriendR,
    FriendI,
    FriendE,
    FriendN,
    /// goto
    GotoO,
    GotoT,
    /// if
    IfF,
    /// int | inline
    InN,
    /// inline
    InlineN,
    InlineL,
    InlineI,
    InlineN2,
    /// int
    IntN,
    /// long
    LongO,
    LongN,
    /// mutable
    MutableU,
    MutableT,
    MutableA,
    MutableB,
    MutableL,
    /// namespace
    NamespaceA,
    NamespaceM,
    NamespaceE,
    NamespaceS,
    NamespaceP,
    NamespaceA2,
    NamespaceC,
    NewE,
    /// nullptr
    NullptrU,
    NullptrL,
    NullptrL2,
    NullptrP,
    NullptrT,
    /// operator
    OperatorP,
    OperatorE,
    OperatorR,
    OperatorA,
    OperatorT,
    OperatorO,
    /// private | protected
    PrR,
    /// private
    PrivateR,
    PrivateI,
    PrivateV,
    PrivateA,
    PrivateT,
    /// protected
    ProtectedR,
    ProtectedO,
    ProtectedT,
    ProtectedE,
    ProtectedC,
    ProtectedT2,
    ProtectedE2,
    /// public
    PublicU,
    PublicB,
    PublicL,
    PublicI,
    /// return
    ReturnE,
    ReturnT,
    ReturnU,
    ReturnR,
    /// short
    ShortH,
    ShortO,
    ShortR,
    /// signed || sizeof
    SiI,
    /// signed
    SignedG,
    SignedN,
    SignedE,
    /// sizeof
    SizeofZ,
    SizeofE,
    SizeofO,
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
    /// switch
    SwitchW,
    SwitchI,
    SwitchT,
    SwitchC,
    /// template
    TemplateE,
    TemplateM,
    TemplateP,
    TemplateL,
    TemplateA,
    TemplateT,
    /// this | throw
    ThH,
    /// this
    ThisI,
    /// throw
    ThrowR,
    ThrowO,
    /// true | try
    TrR,
    /// true
    TrueU,
    /// try
    /// union | unsigned
    UnN,
    /// union
    UnionI,
    UnionO,
    /// unsigned
    UnsignedS,
    UnsignedI,
    UnsignedG,
    UnsignedN,
    UnsignedE,
    /// using
    UsingS,
    UsingI,
    UsingN,
    /// virtual
    VirtualI,
    VirtualR,
    VirtualT,
    VirtualU,
    VirtualA,
    /// void
    VoidO,
    VoidI,
    /// while
    WhileH,
    WhileI,
    WhileL,
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
    let mut token_type: TokenType = TokenType::Identifier;
    let mut buff: String = String::new();
    let mut is_writable: bool = false;
    let mut current_idx: usize = 0;
    let mut current: char;
    while current_idx < text.chars().count() {
        current = text.as_str().as_bytes()[current_idx] as char;
        location.char = current;
        if current == '\n' {
            location.line += 1;
            location.column += 1;
        }
        match state {
            State::Whitespace => {
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
                    c if c.is_alphabetic() => {
                        buff.push(current);
                        token_type = TokenType::Identifier;
                        State::Letter(c)
                    }
                    c if c.is_numeric() => {
                        buff.push(current);
                        token_type = TokenType::ConstValue;
                        State::Number(c)
                    }
                    '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        buff.push(current);
                        is_writable = true;
                        State::Separator(current)
                    }
                    _ => State::Character(current),
                };
            }
            State::Separator(char) => {
                buff.push(current);
                is_writable = true;
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
                    c if c.is_alphabetic() => {
                        token_type = TokenType::Identifier;
                        State::Letter(c)
                    }
                    c if c.is_numeric() => {
                        token_type = TokenType::ConstValue;
                        State::Number(c)
                    }
                    '(' | ')' | '[' | ']' | '{' | '}' | ';' => State::Separator(current),
                    _ => State::Character(current),
                }
            }
            // State::Number(char) => {
            //     match current {
            //     }
            // }
            State::Underscore => match current {
                c if c.is_alphabetic() => state = State::Letter(c),
                ' ' | '\n' | '\t' => {
                    is_writable = true;
                    state = State::Whitespace
                }
                _ => return Err(Error::IncorrectIdentifier(location)),
            },
            State::Add => match current {
                '=' => {
                    buff.push(current);
                    state = State::AddAssign
                }
                '+' => {
                    buff.push(current);
                    state = State::Incr
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::Sub => match current {
                '=' => {
                    buff.push(current);
                    state = State::SubAssign
                }
                '-' => {
                    buff.push(current);
                    state = State::Decr
                }
                '>' => {
                    buff.push(current);
                    state = State::Arrow
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::Mul => match current {
                '=' => {
                    is_writable = true;
                    state = State::MulAssign
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::Div => match current {
                '=' => {
                    is_writable = true;
                    state = State::DivAssign
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::Mod => match current {
                '=' => {
                    is_writable = true;
                    state = State::ModAssign
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::Shl => match current {
                '=' => {
                    is_writable = true;
                    state = State::ShlAssign
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::Shr => match current {
                '=' => {
                    is_writable = true;
                    state = State::ShrAssign
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::And => match current {
                '&' => {
                    is_writable = true;
                    state = State::BitAnd;
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::Or => match current {
                '|' => {
                    is_writable = true;
                    state = State::BitOr;
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::BitXor => match current {
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::BitAnd => match current {
                '=' => {
                    is_writable = true;
                    state = State::BitAndAssign
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::BitOr => match current {
                '=' => {
                    is_writable = true;
                    state = State::BitOrAssign
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::Incr => match current {
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::Decr => match current {
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::LT => match current {
                '=' => {
                    buff.push(current);
                    state = State::LE
                }
                '<' => {
                    buff.push(current);
                    state = State::Shl
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::GT => match current {
                '=' => {
                    buff.push(current);
                    state = State::GE
                }
                '>' => {
                    buff.push(current);
                    state = State::Shr
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::LE => match current {
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    buff.push(current);
                    state = State::Character(current);
                }
            },
            State::GE => match current {
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    buff.push(current);
                    state = State::Character(current);
                }
            },
            State::Eq => match current {
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    buff.push(current);
                    state = State::Character(current);
                }
            },
            State::NEq => match current {
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    buff.push(current);
                    state = State::Character(current);
                }
            },
            State::Neg => match current {
                '=' => {
                    buff.push(current);
                    state = State::NEq
                }
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    buff.push(current);
                    state = State::Character(current);
                }
            },
            State::Assign => {
                match current {
                    '=' => {
                        buff.push(current);
                        state = State::Eq
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    }
                };
            }
            State::AddAssign => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    }
                };
            }
            State::SubAssign => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        State::Character(current);
                    }
                };
            }
            State::MulAssign => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        State::Character(current);
                    }
                };
            }
            State::DivAssign => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        State::Character(current);
                    }
                };
            }
            State::ModAssign => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        State::Character(current);
                    }
                };
            }
            State::ShlAssign => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    }
                };
            }
            State::ShrAssign => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    }
                };
            }
            State::BitAndAssign => match current {
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    buff.push(current);
                    state = State::Character(current)
                }
            },
            State::BitOrAssign => match current {
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    buff.push(current);
                    state = State::Character(current)
                }
            },
            State::Comma => {
                match current {
                    ' ' | '\n' | '\t' => state = State::Whitespace,
                    _ => {
                        buff.push(current);
                        state = State::Character(current)
                    }
                };
            }
            State::Arrow => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        state = State::Character(current)
                    }
                };
            }
            State::Letter('a') => {
                match current {
                    's' => {
                        buff.push(current);
                        state = State::AsmS
                    }
                    'u' => {
                        buff.push(current);
                        state = State::AutoU;
                    }
                    c if c.is_alphanumeric() || c == '_' => {
                        buff.push(current);
                        state = State::Identifier(c)
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        state = State::Character(current)
                    }
                };
            }
            State::AsmS => {
                match current {
                    'm' => {
                        buff.push(current);
                        state = State::KeywordEnd
                    }
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        state = State::Identifier(c)
                    }
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        state = State::Character(current)
                    }
                };
            }
            State::AutoU => {
                match current {
                    't' => {
                        buff.push(current);
                        state = State::AutoT;
                    }
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        state = State::Identifier(c);
                    }
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    }
                };
            }
            State::AutoT => {
                match current {
                    'o' => {
                        buff.push(current);
                        state = State::KeywordEnd
                    }
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' => state = State::Whitespace,

                    _ => {
                        buff.push(current);
                        state = State::Character(current)
                    }
                };
            }
            // State::AutoO => {
            //     match current {
            //         c if c.is_alphanumeric() => {
            //             buff.push(current);
            //             state = State::Identifier(c)
            //         },
            //         ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
            //             current_idx -= 1;
            //             state = State::KeywordEnd
            //         },
            //         _ => {
            //             buff.push(current);
            //             Character(current)
            //         }
            //     };
            // }
            State::Letter('b') => match current {
                'o' => {
                    buff.push(current);
                    state = State::BoolO
                }
                'r' => {
                    buff.push(current);
                    state = State::BreakR
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::BoolO => match current {
                'o' => {
                    state = State::BoolO2;
                }
                c if c.is_alphanumeric() || c == '_' => {
                    state = State::Identifier(c);
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => {
                    state = State::Character(current);
                }
            },
            State::BoolO2 => {
                match current {
                    'l' => {
                        buff.push(current);
                        state = State::KeywordEnd
                    }
                    c if c.is_alphanumeric() => state = State::Letter(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current),
                };
            }
            // State::BoolL => {
            //     match current {
            //         c if c.is_alphanumeric() => {
            //             buff.push(current);
            //             Letter(c)
            //         },
            //         ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
            //             current_idx -= 1;
            //             State::KeywordEnd
            //         },
            //         _ => {
            //             buff.push(current);
            //             Character(current)
            //         }
            //     }
            // }
            State::BreakR => match current {
                'e' => {
                    buff.push(current);
                    state = State::BreakE
                }
                c if c.is_alphanumeric() => state = State::Identifier(c),
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::BreakE => match current {
                'a' => {
                    buff.push(current);
                    state = State::BreakA
                }
                c if c.is_alphanumeric() => state = State::Letter(c),
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::BreakA => match current {
                'k' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() => state = State::Letter(c),
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::BreakK => {
            //     match current {
            //         c if c.is_alphanumeric() => {
            //             buff.push(current);
            //             state = State::Identifier(c)
            //         },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         },
            //         _ => {
            //             buff.push(current);
            //             state = State::Character(current)
            //         }
            //     }
            // }
            State::Letter('c') => match current {
                'a' => {
                    buff.push(current);
                    state = State::CaA
                }
                'h' => {
                    buff.push(current);
                    state = State::CharH
                }
                'o' => {
                    buff.push(current);
                    state = State::CoO
                }
                c if c.is_alphanumeric() => state = State::Letter(current),
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::CaA => match current {
                's' => {
                    buff.push(current);
                    state = State::CaseS
                }
                't' => {
                    buff.push(current);
                    state = State::CatchT
                }
                c if c.is_alphanumeric() => state = State::Identifier(c),
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::CaseS => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() => state = State::Identifier(c),
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::CaseE => {
            //     match current {
            //         c if c.is_alphanumeric() => {
            //             buff.push(c);
            //             Letter(c)
            //         }
            //         ' ' | '\n' | '\t' | '('|')'|'['|']'| '{'| '}'| ';' => {
            //             State::KeywordEnd
            //         }
            //         _ => Character(current)
            //     }
            // }
            State::CatchT => match current {
                'c' => {
                    buff.push(current);
                    state = State::CatchC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::CatchC => match current {
                'h' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::CatchH => {
            //     match current {
            //         c if c.is_alphanumeric() => {
            //             buff.push(current);
            //             State::Identifier(c)
            //         }
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             State::KeywordEnd
            //         }
            //         _ => State::Character(current)
            //      }
            // }
            State::CharH => match current {
                'a' => {
                    buff.push(current);
                    state = State::CharA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::CharA => match current {
                'r' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::CharR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => {
            //            buff.push(c);
            //            State::Identifier(c)
            //         }
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             State::KeywordEnd
            //         }
            //         _ => State::Character(current)
            //     }
            // }
            State::ClassL => match current {
                'a' => {
                    buff.push(current);
                    state = State::ClassA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ClassA => match current {
                's' => {
                    buff.push(current);
                    state = State::ClassS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ClassS => match current {
                's' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ClassS2 => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             State::Whitespace
            //         }
            //         _ => State::Character(current)
            //     }
            // }
            State::CoO => match current {
                'm' => {
                    buff.push(current);
                    state = State::ComplM
                }
                'n' => {
                    buff.push(current);
                    state = State::ConN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ComplM => match current {
                'p' => {
                    buff.push(current);
                    state = State::ComplP
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ComplP => match current {
                'l' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ComplL => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            // is_writable = true;
            //         }
            //         _ => State::Character(current)
            //     }
            // }
            State::ConN => match current {
                'c' => {
                    buff.push(current);
                    state = State::ConceptC
                }
                's' => {
                    buff.push(current);
                    state = State::ConstS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ConceptC => match current {
                'e' => {
                    buff.push(current);
                    state = State::ConceptE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ConceptE => match current {
                'p' => {
                    buff.push(current);
                    state = State::ConceptP
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ConceptP => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ConceptT => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            // is_writable = true;
            //         }
            //         _ => State::Character(current)
            //     }
            // }
            State::ConstS => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ConstT => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            // is_writable = true;
            //         }
            //         _ => State::Character(current)
            //     }
            // }
            State::Letter('d') => match current {
                'e' => {
                    buff.push(current);
                    state = State::DeE
                }
                'o' => {
                    buff.push(current);
                    state = State::DoO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::DeE => match current {
                'f' => {
                    buff.push(current);
                    state = State::DefaultF
                }
                'l' => {
                    buff.push(current);
                    state = State::DeleteE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::DefaultF => match current {
                'a' => {
                    buff.push(current);
                    state = State::DefaultF
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::DefaultA => match current {
                'u' => {
                    buff.push(current);
                    state = State::DefaultU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::DefaultU => match current {
                'l' => {
                    buff.push(current);
                    state = State::DefaultL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::DefaultL => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::DefaultT => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::DeleteL => match current {
                'e' => {
                    buff.push(current);
                    state = State::DeleteE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::DeleteE => match current {
                't' => {
                    buff.push(current);
                    state = State::DeleteT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::DeleteT => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::DeleteE2 => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::DoO => match current {
                'u' => {
                    buff.push(current);
                    state = State::DoubleU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::DoubleU => match current {
                'b' => {
                    buff.push(current);
                    state = State::DoubleB
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::DoubleB => match current {
                'l' => {
                    buff.push(current);
                    state = State::DoubleL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::DoubleL => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::DoubleE => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('e') => match current {
                'l' => {
                    buff.push(current);
                    state = State::ElseL
                }
                'n' => {
                    buff.push(current);
                    state = State::EnumN
                }
                'x' => {
                    buff.push(current);
                    state = State::ExX
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ElseL => match current {
                's' => {
                    buff.push(current);
                    state = State::ElseS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ElseS => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ElseE => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::EnumN => match current {
                'u' => {
                    buff.push(current);
                    state = State::EnumU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::EnumU => match current {
                'm' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::EnumM => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExX => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExportP => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExportO => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExportR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExportT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExternT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExternE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExternR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExternN => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('f') => match current {
                'a' => {
                    buff.push(current);
                    state = State::FalseA
                }
                'l' => {
                    buff.push(current);
                    state = State::FloatL
                }
                'o' => {
                    buff.push(current);
                    state = State::ForO
                }
                'r' => {
                    buff.push(current);
                    state = State::FriendR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::FalseA => match current {
                'l' => {
                    buff.push(current);
                    state = State::FalseL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::FalseL => match current {
                's' => {
                    buff.push(current);
                    state = State::FalseS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::FalseS => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::FalseE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::FloatL => match current {
                'o' => {
                    buff.push(current);
                    state = State::FloatO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::FloatO => match current {
                'a' => {
                    buff.push(current);
                    state = State::FloatA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::FloatA => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::FloatT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::ForO => match current {
                'r' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ForR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::FriendR => match current {
                'i' => {
                    buff.push(current);
                    state = State::FriendI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::FriendI => match current {
                'e' => {
                    buff.push(current);
                    state = State::FriendE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::FriendE => match current {
                'n' => {
                    buff.push(current);
                    state = State::FriendN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::FriendN => match current {
                'd' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::Letter('g') => match current {
                'o' => {
                    buff.push(current);
                    state = State::GotoO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::GotoO => match current {
                't' => {
                    buff.push(current);
                    state = State::GotoT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::GotoT => match current {
                'o' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::GotoO2 => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('i') => match current {
                'f' => {
                    buff.push(current);
                    state = State::IfF
                }
                'n' => {
                    buff.push(current);
                    state = State::InN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::IfF => match current {
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::InN => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                'l' => {
                    buff.push(current);
                    state = State::InlineL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::InlineL => match current {
                'i' => {
                    buff.push(current);
                    state = State::InlineI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::InlineI => match current {
                'n' => {
                    buff.push(current);
                    state = State::InlineN2
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::InlineN2 => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::InlineE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::IntT => {
            //     match current {
            //         c if c.is_alphanumeric() => {
            //             buff.push(current);
            //             State::Letter(c)
            //         }
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             State::KeywordEnd
            //         },
            //         _ => {
            //             buff.push(current);
            //             State::Character(current)
            //         }
            //     }
            // }
            State::Letter('l') => match current {
                'f' => {
                    buff.push(current);
                    state = State::LongO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::LongO => match current {
                'n' => {
                    buff.push(current);
                    state = State::LongN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::LongN => match current {
                'g' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::LongG => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('m') => match current {
                'u' => {
                    buff.push(current);
                    state = State::MutableU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::MutableU => match current {
                't' => {
                    buff.push(current);
                    state = State::MutableT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::MutableT => match current {
                'a' => {
                    buff.push(current);
                    state = State::MutableA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::MutableA => match current {
                'b' => {
                    buff.push(current);
                    state = State::MutableB
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::MutableB => match current {
                'l' => {
                    buff.push(current);
                    state = State::MutableL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::MutableL => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::MutableE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('n') => match current {
                'a' => {
                    buff.push(current);
                    state = State::NamespaceA
                }
                'e' => {
                    buff.push(current);
                    state = State::NewE
                }
                'u' => {
                    buff.push(current);
                    state = State::NullptrU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NamespaceA => match current {
                'm' => {
                    buff.push(current);
                    state = State::NamespaceM
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NamespaceM => match current {
                'e' => {
                    buff.push(current);
                    state = State::NamespaceE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NamespaceE => match current {
                's' => {
                    buff.push(current);
                    state = State::NamespaceS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NamespaceS => match current {
                'p' => {
                    buff.push(current);
                    state = State::NamespaceP
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NamespaceP => match current {
                'a' => {
                    buff.push(current);
                    state = State::NamespaceA2
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NamespaceA2 => match current {
                'c' => {
                    buff.push(current);
                    state = State::NamespaceC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NamespaceC => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::NamespaceE2 => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::NewE => match current {
                'w' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::NewW => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::NullptrU => match current {
                'l' => {
                    buff.push(current);
                    state = State::NullptrL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NullptrL => match current {
                'l' => {
                    buff.push(current);
                    state = State::NullptrL2
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NullptrL2 => match current {
                'p' => {
                    buff.push(current);
                    state = State::NullptrP
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NullptrP => match current {
                't' => {
                    buff.push(current);
                    state = State::NullptrT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::NullptrT => match current {
                'r' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::NullptrR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::OperatorP => match current {
                'e' => {
                    buff.push(current);
                    state = State::OperatorE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::OperatorE => match current {
                'r' => {
                    buff.push(current);
                    state = State::OperatorR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::OperatorR => match current {
                'a' => {
                    buff.push(current);
                    state = State::OperatorA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::OperatorA => match current {
                't' => {
                    buff.push(current);
                    state = State::OperatorT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::OperatorT => match current {
                'o' => {
                    buff.push(current);
                    state = State::OperatorO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::OperatorO => match current {
                'r' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::OperatorR2 => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('p') => match current {
                'r' => {
                    buff.push(current);
                    state = State::PrR
                }
                'u' => {
                    buff.push(current);
                    state = State::PublicU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::PrR => match current {
                'o' => {
                    buff.push(current);
                    state = State::ProtectedO
                }
                'i' => {
                    buff.push(current);
                    state = State::PrivateI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::PrivateR => {
            //     match current {
            //         'i' => { buff.push(current); state = State::PrivateI },
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::PrivateI => match current {
                'v' => {
                    buff.push(current);
                    state = State::PrivateV
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::PrivateV => match current {
                'a' => {
                    buff.push(current);
                    state = State::PrivateA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::PrivateA => match current {
                't' => {
                    buff.push(current);
                    state = State::PrivateT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::PrivateT => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::PrivateE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ProtectedR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::ProtectedO => match current {
                't' => {
                    buff.push(current);
                    state = State::ProtectedT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ProtectedT => match current {
                'e' => {
                    buff.push(current);
                    state = State::ProtectedC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ProtectedE => match current {
                'c' => {
                    buff.push(current);
                    state = State::ProtectedC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ProtectedC => match current {
                't' => {
                    buff.push(current);
                    state = State::ProtectedT2
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ProtectedT2 => match current {
                'e' => {
                    buff.push(current);
                    state = State::ProtectedE2
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ProtectedE2 => match current {
                'd' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ProtectedD => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::PublicU => match current {
                'b' => {
                    buff.push(current);
                    state = State::PublicB
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::PublicB => match current {
                'l' => {
                    buff.push(current);
                    state = State::PublicL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::PublicL => match current {
                'i' => {
                    buff.push(current);
                    state = State::PublicI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::PublicI => match current {
                'c' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::PublicC => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('r') => match current {
                'e' => {
                    buff.push(current);
                    state = State::ReturnE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ReturnE => match current {
                't' => {
                    buff.push(current);
                    state = State::ReturnT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ReturnT => match current {
                'u' => {
                    buff.push(current);
                    state = State::ReturnU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ReturnU => match current {
                'r' => {
                    buff.push(current);
                    state = State::ReturnR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ReturnR => match current {
                'n' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ReturnN => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('s') => match current {
                'h' => {
                    buff.push(current);
                    state = State::ShortH
                }
                'i' => {
                    buff.push(current);
                    state = State::SiI
                }
                't' => {
                    buff.push(current);
                    state = State::StT
                }
                'w' => {
                    buff.push(current);
                    state = State::SwitchW
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ShortH => match current {
                'o' => {
                    buff.push(current);
                    state = State::ShortO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ShortO => match current {
                'r' => {
                    buff.push(current);
                    state = State::ShortR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ShortR => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ShortT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::SiI => match current {
                'g' => {
                    buff.push(current);
                    state = State::SignedG
                }
                'z' => {
                    buff.push(current);
                    state = State::SizeofZ
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::SignedG => match current {
                'n' => {
                    buff.push(current);
                    state = State::SignedN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::SignedN => match current {
                'e' => {
                    buff.push(current);
                    state = State::SignedE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::SignedE => match current {
                'd' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::SignedD => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::SizeofZ => match current {
                'e' => {
                    buff.push(current);
                    state = State::SizeofE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::SizeofE => match current {
                'o' => {
                    buff.push(current);
                    state = State::SizeofO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::SizeofO => match current {
                'f' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::SizeofF => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::StT => match current {
                'a' => {
                    buff.push(current);
                    state = State::StaticA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::StaticA => match current {
                't' => {
                    buff.push(current);
                    state = State::StaticT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::StaticT => match current {
                'i' => {
                    buff.push(current);
                    state = State::StaticI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::StaticI => match current {
                'c' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::StaticC => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::StructR => match current {
                'u' => {
                    buff.push(current);
                    state = State::StructU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::StructU => match current {
                'c' => {
                    buff.push(current);
                    state = State::StructC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::StructC => match current {
                't' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::StructT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::SwitchW => match current {
                'i' => {
                    buff.push(current);
                    state = State::SwitchI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::SwitchI => match current {
                't' => {
                    buff.push(current);
                    state = State::SwitchT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::SwitchT => match current {
                'c' => {
                    buff.push(current);
                    state = State::SwitchC
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::SwitchC => match current {
                'h' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::SwitchH => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('t') => match current {
                'e' => {
                    buff.push(current);
                    state = State::TemplateE
                }
                'h' => {
                    buff.push(current);
                    state = State::ThH
                }
                'r' => {
                    buff.push(current);
                    state = State::TrR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::TemplateE => match current {
                'm' => {
                    buff.push(current);
                    state = State::TemplateM
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::TemplateM => match current {
                'p' => {
                    buff.push(current);
                    state = State::TemplateP
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::TemplateP => match current {
                'l' => {
                    buff.push(current);
                    state = State::TemplateL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::TemplateL => match current {
                'a' => {
                    buff.push(current);
                    state = State::TemplateA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::TemplateA => match current {
                't' => {
                    buff.push(current);
                    state = State::TemplateT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::TemplateT => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::TemplateE2 => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::ThH => match current {
                'i' => {
                    buff.push(current);
                    state = State::ThisI
                }
                'r' => {
                    buff.push(current);
                    state = State::ThrowR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ThisI => match current {
                's' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ThisS => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::ThrowR => match current {
                'o' => {
                    buff.push(current);
                    state = State::ThrowO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::ThrowO => match current {
                'w' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::ThrowW => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::TrR => match current {
                'y' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                'u' => {
                    buff.push(current);
                    state = State::TrueU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::TrueU => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::TrueE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::TryY => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('u') => match current {
                'n' => {
                    buff.push(current);
                    state = State::UnN
                }
                'h' => {
                    buff.push(current);
                    state = State::UsingS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::UnN => match current {
                'i' => {
                    buff.push(current);
                    state = State::UnionI
                }
                's' => {
                    buff.push(current);
                    state = State::UnsignedS
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::UnionI => match current {
                'o' => {
                    buff.push(current);
                    state = State::UnionO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::UnionO => match current {
                'n' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::UnionN => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::UnsignedS => match current {
                'i' => {
                    buff.push(current);
                    state = State::UnsignedI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::UnsignedI => match current {
                'g' => {
                    buff.push(current);
                    state = State::UnsignedG
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::UnsignedG => match current {
                'n' => {
                    buff.push(current);
                    state = State::UnsignedN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::UnsignedN => match current {
                'e' => {
                    buff.push(current);
                    state = State::UnsignedE
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::UnsignedE => match current {
                'd' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::UnsignedD => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::UsingS => match current {
                'i' => {
                    buff.push(current);
                    state = State::UsingI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::UsingI => match current {
                'n' => {
                    buff.push(current);
                    state = State::UsingN
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::UsingN => match current {
                'g' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::UsingG => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('v') => match current {
                'i' => {
                    buff.push(current);
                    state = State::VirtualI
                }
                'o' => {
                    buff.push(current);
                    state = State::VoidO
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::VirtualI => match current {
                'r' => {
                    buff.push(current);
                    state = State::VirtualR
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::VirtualR => match current {
                't' => {
                    buff.push(current);
                    state = State::VirtualT
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::VirtualT => match current {
                'u' => {
                    buff.push(current);
                    state = State::VirtualU
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::VirtualU => match current {
                'a' => {
                    buff.push(current);
                    state = State::VirtualA
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::VirtualA => match current {
                'l' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::VirtualL => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::VoidO => match current {
                'i' => {
                    buff.push(current);
                    state = State::VoidI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::VoidI => match current {
                'd' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::VoidD => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('w') => match current {
                'h' => {
                    buff.push(current);
                    state = State::WhileH
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::WhileH => match current {
                'h' => {
                    buff.push(current);
                    state = State::WhileI
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::WhileI => match current {
                'l' => {
                    buff.push(current);
                    state = State::WhileL
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::WhileL => match current {
                'e' => {
                    buff.push(current);
                    state = State::KeywordEnd
                }
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            // State::WhileE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => { buff.push(current); state = State::Identifier(c) },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::KeywordEnd => match current {
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },

            State::Separator(s) => match current {
                ' ' | '\n' | '\t' => {
                    current_idx -= 1;
                    is_writable = true;
                    state = State::Whitespace
                }
                '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                    current_idx -= 1;
                    is_writable = true;
                    state = State::Separator(current)
                }
                _ => state = State::Character(current),
            },
            State::Identifier(i) => match current {
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::Character('"') => {
                buff.push(current);
                match current {
                    _ => state = State::StringLiteral(current),
                }
            }
            State::StringLiteral('"') => match current {
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            State::StringLiteral(c) => match current {
                '"' => state = State::StringLiteral('"'),
                c => {
                    buff.push(current);
                    state = State::StringLiteral(current);
                }
            },
            State::Number(n) => match current {
                c if c.is_numeric() || c == '_' || c == 'e' => {
                    buff.push(c);
                    state = State::Number(c);
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' => {
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },

            State::Letter(l) => match current {
                c if c.is_alphanumeric() || c == '_' => {
                    buff.push(current);
                    state = State::Identifier(c)
                }
                ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                    buff.push(current);
                    current_idx -= 1;
                    is_writable = true;
                }
                _ => state = State::Character(current),
            },
            _ => {}
        }
        println!("{} : {:?}, {}", current, state, is_writable);
        current_idx += 1;

        if is_writable {
            let token_type = match state {
                State::Identifier(_) | State::Letter(_) => TokenType::Identifier,
                State::KeywordEnd => TokenType::Keyword,
                State::Number(_) => TokenType::ConstValue,
                State::Separator(_) => TokenType::Separator,
                State::StringLiteral(_) => TokenType::StringLiteral,
                _ => TokenType::None,
            };
            state = State::Whitespace;
            is_writable = false;
            tokens.push(Token {
                token_type,
                token: buff.clone(),
            });
            buff.clear();
        }
    }
    Ok(tokens)
}
