
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
        println!("{} : {:?}", current, state);
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
                    c if c.is_alphabetic() => {
                        token_type = TokenType::Identifier;
                        State::Letter(c)
                    }
                    c if c.is_numeric() => {
                        token_type = TokenType::ConstValue;
                        State::Number(c)
                    }
                    '(' | ')' | '[' | ']' | '{' | '}' | ';' => State::Separator(current),
                    _ => State::Character(current)
                };
            }
            // State::Separator(char) => {
            //     match current {

            //     }
            // }
            // State::Number(char) => {
            //     match current {
            //     }
            // }
            State::Underscore => {
                buff.push(current);
                match current {
                    c if c.is_alphabetic() => {
                        state = State::Letter(c)
                    },
                    ' ' | '\n' | '\t' => {
                        is_writable = true;
                        state = State::Whitespace
                    },
                    _ => return Err(Error::IncorrectIdentifier(location))
                }
            }
            State::Add => {
                buff.push(current);
                match current {
                    '=' => {
                        state = State::AddAssign
                    },
                    '+' => {
                        state = State::Incr
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::Sub => {
                buff.push(current);
                match current {
                    '=' => {
                        state = State::SubAssign
                    }
                    '-' => {
                        state = State::Decr
                    }
                    '>' => {
                        state = State::Arrow
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::Mul => {
                buff.push(current);
                match current {
                    '=' => {
                        is_writable = true;
                        state = State::MulAssign
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::Div => {
                buff.push(current);
                match current {
                    '=' => {
                        is_writable = true;
                        state = State::DivAssign
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::Mod => {
                buff.push(current);
                match current {
                    '=' => {
                        is_writable = true;
                        state = State::ModAssign
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::Shl => {
                buff.push(current);
                match current {
                    '=' => {
                        is_writable = true;
                        state = State::ShlAssign
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::Shr => {
                buff.push(current);
                match current {
                    '=' => {
                        is_writable = true;
                        state = State::ShrAssign
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::And => {
                buff.push(current);
                match current {
                    '&' => {
                        is_writable = true;
                        state = State::BitAnd;
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::Or => {
                buff.push(current);
                match current {
                    '|' => {
                        is_writable = true;
                        state = State::BitOr;
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::BitXor => {
                buff.push(current);
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::BitAnd => {
                buff.push(current);
                match current {
                    '=' => {
                        is_writable = true;
                        state = State::BitAndAssign
                    },
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::BitOr => {
                buff.push(current);
                match current {
                    '=' => {
                        is_writable = true;
                        state = State::BitOrAssign
                    },
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::Incr => {
                buff.push(current);
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::Decr => {
                buff.push(current);
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::LT => {
                buff.push(current);
                match current {
                    '=' => {
                        state = State::LE
                    },
                    '<' => {
                        state = State::Shl
                    },
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::GT => {
                buff.push(current);
                match current {
                    '=' => {
                        state = State::GE
                    },
                    '>' => {
                        state = State::Shr
                    },
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    },
                }
            }
            State::LE => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    },
                }
            }
            State::GE => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    },
                }
            }
            State::Eq => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    },
                }
            }
            State::NEq => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    },
                }
            }
            State::Neg => {
                buff.push(current);
                match current {
                    '=' => {
                        state = State::NEq
                    }
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    },
                }
            }
            State::Assign => {
                buff.push(current);
                match current {
                    '=' => {
                        state = State::Eq
                    },
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    },
                };
            }
            State::AddAssign => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
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
                    },
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
                    },
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
                    },
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
                    },
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
                    },
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
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    }
                };
            }
            State::BitAndAssign => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current)
                    }
                }
            }
            State::BitOrAssign => {
                match current {
                    ' ' | '\n' | '\t' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current)
                    }
                }
            }
            State::Comma => {
                match current {
                    ' ' | '\n' | '\t' => {
                        state = State::Whitespace
                    },
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
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current)
                    }
                };
            }
            State::Letter('a') => {
                buff.push(current);
                match current {
                    's' => State::AsmS,
                    'u' => State::AutoU,
                    ' ' | '\n' | '\t' => {
                        State::Whitespace
                    },
                    _ => {
                        buff.push(current);
                        State::Character(current)
                    }
                };
            }
            State::AsmS => {
                buff.push(current);
                match current {
                    'm' => {
                        state = State::KeywordEnd
                    },
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        state = State::Identifier(c)
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
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
                    },
                    c if c.is_alphanumeric() => {
                        buff.push(current);
                        state = State::Identifier(c);
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        buff.push(current);
                        state = State::Character(current);
                    }
                };
            }
            State::AutoT => {
                buff.push(current);
                match current {
                    'o' => State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => State::Identifier(c),
                    ' ' | '\n' | '\t' => {
                        State::Whitespace
                    },

                    _ => {
                        buff.push(current);
                        State::Character(current)
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
            State::Letter('b') => {
                buff.push(current);
                match current {
                    'o' => {
                        state = State::BoolO
                    },
                    'r' => {
                        state = State::BreakR
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    c if c.is_alphanumeric() || c == '_' => {
                        state = State::Identifier(c)
                    },
                    _ => {
                        state = State::Character(current);
                    }
                }
            }
            State::BoolO => {
                buff.push(current);
                match current {
                    'o' => {
                        state = State::BoolO2;
                    },
                    c if c.is_alphanumeric() || c == '_' => {
                        state = State::Identifier(c);
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current);
                    }
                }
            }
            State::BoolO2 => {
                buff.push(current);
                match current {
                    'l' => {
                        state = State::KeywordEnd
                    },
                    c if c.is_alphanumeric() => {
                        state = State::Letter(c)
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current)
                    }
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
            State::BreakR => {
                buff.push(current);
                match current {
                    'e' => {
                        state = State::BreakE
                    },
                    c if c.is_alphanumeric() => {
                        state = State::Identifier(c)
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current)
                    }
                }
            }
            State::BreakE => {
                buff.push(current);
                match current {
                    'a' => {
                        state = State::BreakA
                    },
                    c if c.is_alphanumeric() => {
                        state = State::Letter(c)
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current)
                    }
                }
            }
            State::BreakA => {
                buff.push(current);
                match current {
                    'k' => {
                        state = State::KeywordEnd
                    },
                    c if c.is_alphanumeric() => {
                        state = State::Letter(c)
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current)
                    }
                }
            }
            // State::BreakK => {
            //     match current {
            //         c if c.is_alphanumeric() => {
            //             buff.push(current);
            //             state = State::Identifier(c)
            //         },
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         },
            //         _ => {
            //             buff.push(current);
            //             state = State::Character(current)
            //         }
            //     }
            // }
            State::Letter('c') => {
                buff.push(current);
                match current {
                    'a' => {
                        state = State::CaA
                    },
                    'h' => {
                        state = State::CharH
                    },
                    'o' => {
                        state = State::CoO
                    },
                    c if c.is_alphanumeric() => {
                        state = State::Letter(current)
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current)
                    }
                }
            }
            State::CaA => {
                buff.push(current);
                match current {
                    's' => {
                        state = State::CaseS
                    },
                    't' => {
                        state = State::CatchT
                    },
                    c if c.is_alphanumeric() => {
                        state = State::Identifier(c)
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    },
                    _ => {
                        state = State::Character(current)
                    }
                }
            }
            State::CaseS => {
                buff.push(current);
                match current {
                    'e' => {
                        state = State::KeywordEnd
                    },
                    c if c.is_alphanumeric() => {
                        state = State::Identifier(c)
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => {
                        state = State::Character(current)
                    }
                }
            }
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
            State::CatchT => {
                buff.push(current);
                match current {
                    'c' => {
                        state = State::CatchC
                    },
                    c if c.is_alphanumeric() || c == '_' => {
                        state = State::Identifier(c)
                    },
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::CatchC => {
                buff.push(current);
                match current {
                    'h' => {
                        state = State::KeywordEnd
                    },
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::CatchH => {
            //     match current {
            //         c if c.is_alphanumeric() => {
            //             buff.push(current);
            //             State::Identifier(c)
            //         }
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             State::KeywordEnd
            //         }
            //         _ => State::Character(current)
            //      }
            // }
            State::CharH => {
                buff.push(current);
                match current {
                    'a' => state = State::CharA,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::CharA => {
                buff.push(current);
                match current {
                    'r' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::CharR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => {
            //            buff.push(c);
            //            State::Identifier(c)
            //         }
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             State::KeywordEnd
            //         }
            //         _ => State::Character(current)
            //     }
            // }
            State::ClassL => {
                buff.push(current);
                match current {
                    'a' => state = State::ClassA,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ClassA => {
                buff.push(current);
                match current {
                    's' => state = State::ClassS,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ClassS => {
                match current {
                    's' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ClassS2 => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             State::Whitespace
            //         }
            //         _ => State::Character(current)
            //     }
            // }
            State::CoO => {
                match current {
                    'm' => state = State::ComplM,
                    'n' => state = State::ConN,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ComplM => {
                match current {
                    'p' => state = State::ComplP,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ComplP => {
                match current {
                    'l' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ComplL => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            // is_writable = true;
            //         }
            //         _ => State::Character(current)
            //     }
            // }
            State::ConN => {
                match current {
                    'c' => state = State::ConceptC,
                    's' => state = State::ConstS,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ConceptC => {
                match current {
                    'e' => state = State::ConceptE,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ConceptE => {
                match current {
                    'p' => state = State::ConceptP,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ConceptP => {
                match current {
                    't' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ConceptT => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            // is_writable = true;
            //         }
            //         _ => State::Character(current)
            //     }
            // }
            State::ConstS => {
                match current {
                    't' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ConstT => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            // is_writable = true;
            //         }
            //         _ => State::Character(current)
            //     }
            // }
            State::Letter('d') => {
                match current {
                    'e' => state = State::DeE,
                    'o' => state = State::DoO,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::DeE => {
                match current {
                    'f' => state = State::DefaultF,
                    'l' => state = State::DeleteE,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::DefaultF => {
                match current {
                    'a' => state = State::DefaultF,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::DefaultA => {
                match current {
                    'u' => state = State::DefaultU,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::DefaultU => {
                match current {
                    'l' => state = State::DefaultL,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::DefaultL => {
                match current {
                    't' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::DefaultT => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::DeleteL => {
                match current {
                    'e' => state = State::DeleteE,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::DeleteE => {
                match current {
                    't' => state = State::DeleteT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::DeleteT => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::DeleteE2 => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::DoO => {
                match current {
                    'u' => state = State::DoubleU,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::DoubleU => {
                match current {
                    'b' => state = State::DoubleB,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::DoubleB => {
                match current {
                    'l' => state = State::DoubleL,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::DoubleL => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::DoubleE => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('e') => {
                match current {
                    'l' => state = State::ElseL,
                    'n' => state = State::EnumN,
                    'x' => state = State::ExX,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ElseL => {
                match current {
                    's' => state = State::ElseS,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ElseS => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ElseE => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::EnumN => {
                match current {
                    'u' => state = State::EnumU,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::EnumU => {
                match current {
                    'm' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::EnumM => {
            //     match current {
            //
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExX => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExportP => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExportO => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExportR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExportT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExternT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExternE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExternR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ExternN => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('f') => {
                match current {
                    'a' => state = State::FalseA,
                    'l' => state = State::FloatL,
                    'o' => state = State::ForO,
                    'r' => state = State::FriendR,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::FalseA => {
                match current {
                    'l' => state = State::FalseL,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::FalseL => {
                match current {
                    's' => state = State::FalseS,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::FalseS => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::FalseE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::FloatL => {
                match current {
                    'o' => state = State::FloatO,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::FloatO => {
                match current {
                    'a' => state = State::FloatA,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::FloatA => {
                match current {
                    't' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::FloatT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::ForO => {
                match current {
                    'r' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ForR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::FriendR => {
                match current {
                    'i' => state = State::FriendI,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::FriendI => {
                match current {
                    'e' => state = State::FriendE,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::FriendE => {
                match current {
                    'n' => state = State::FriendN,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::FriendN => {
                match current {
                    'd' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::Letter('g') => {
                match current {
                    'o' => state = State::GotoO,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::GotoO => {
                match current {
                    't' => state = State::GotoT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::GotoT => {
                match current {
                    'o' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::GotoO2 => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('i') => {
                match current {
                    'f' => state = State::IfF,
                    'n' => state = State::InN,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::IfF => {
                match current {
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::InN => {
                buff.push(current);
                match current {
                    't' => state = State::KeywordEnd,
                    'l' => state = State::InlineL,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::InlineL => {
                match current {
                    'i' => state = State::InlineI,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::InlineI => {
                match current {
                    'n' => state = State::InlineN2,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::InlineN2 => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::InlineE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
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
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             State::KeywordEnd
            //         },
            //         _ => {
            //             buff.push(current);
            //             State::Character(current)
            //         }
            //     }
            // }
            State::Letter('l') => {
                match current {
                    'f' => state = State::LongO,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::LongO => {
                match current {
                    'n' => state = State::LongN,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::LongN => {
                match current {
                    'g' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::LongG => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('m') => {
                match current {
                    'u' => state = State::MutableU,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::MutableU => {
                match current {
                    't' => state = State::MutableT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::MutableT => {
                match current {
                    'a' => state = State::MutableA,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::MutableA => {
                match current {
                    'b' => state = State::MutableB,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::MutableB => {
                match current {
                    'l' => state = State::MutableL,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::MutableL => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::MutableE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('n') => {
                match current {
                    'a' => state = State::NamespaceA,
                    'e' => state = State::NewE,
                    'u' => state = State::NullptrU,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NamespaceA => {
                match current {
                    'm' => state = State::NamespaceM,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NamespaceM => {
                match current {
                    'e' => state = State::NamespaceE,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NamespaceE => {
                match current {
                    's' => state = State::NamespaceS,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NamespaceS => {
                match current {
                    'p' => state = State::NamespaceP,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NamespaceP => {
                match current {
                    'a' => state = State::NamespaceA2,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NamespaceA2 => {
                match current {
                    'c' => state = State::NamespaceC,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NamespaceC => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::NamespaceE2 => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::NewE => {
                match current {
                    'w' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::NewW => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::NullptrU => {
                match current {
                    'l' => state = State::NullptrL,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NullptrL => {
                match current {
                    'l' => state = State::NullptrL2,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NullptrL2 => {
                match current {
                    'p' => state = State::NullptrP,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NullptrP => {
                match current {
                    't' => state = State::NullptrT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::NullptrT => {
                match current {
                    'r' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::NullptrR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::OperatorP => {
                match current {
                    'e' => state = State::OperatorE,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::OperatorE => {
                match current {
                    'r' => state = State::OperatorR,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::OperatorR => {
                match current {
                    'a' => state = State::OperatorA,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::OperatorA => {
                match current {
                    't' => state = State::OperatorT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::OperatorT => {
                match current {
                    'o' => state = State::OperatorO,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::OperatorO => {
                match current {
                    'r' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::OperatorR2 => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('p') => {
                match current {
                    'r' => state = State::PrR,
                    'u' => state = State::PublicU,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::PrR => {
                match current {
                    'o' => state = State::ProtectedO,
                    'i' => state = State::PrivateI,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::PrivateR => {
            //     match current {
            //         'i' => state = State::PrivateI,
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::PrivateI => {
                match current {
                    'v' => state = State::PrivateV,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::PrivateV => {
                match current {
                    'a' => state = State::PrivateA,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::PrivateA => {
                match current {
                    't' => state = State::PrivateT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::PrivateT => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::PrivateE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::ProtectedR => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::ProtectedO => {
                match current {
                    't' => state = State::ProtectedT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ProtectedT => {
                match current {
                    'e' => state = State::ProtectedC,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ProtectedE => {
                match current {
                    'c' => state = State::ProtectedC,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ProtectedC => {
                match current {
                    't' => state = State::ProtectedT2,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ProtectedT2 => {
                match current {
                    'e' => state = State::ProtectedE2,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ProtectedE2 => {
                match current {
                    'd' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ProtectedD => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::PublicU => {
                match current {
                    'b' => state = State::PublicB,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::PublicB => {
                match current {
                    'l' => state = State::PublicL,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::PublicL => {
                match current {
                    'i' => state = State::PublicI,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::PublicI => {
                match current {
                    'c' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::PublicC => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('r') => {
                match current {
                    'e' => state = State::ReturnE,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ReturnE => {
                match current {
                    't' => state = State::ReturnT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ReturnT => {
                match current {
                    'u' => state = State::ReturnU,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ReturnU => {
                match current {
                    'r' => state = State::ReturnR,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ReturnR => {
                match current {
                    'n' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ReturnN => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('s') => {
                match current {
                    'h' => state = State::ShortH,
                    'i' => state = State::SiI,
                    't' => state = State::StT,
                    'w' => state = State::SwitchW,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ShortH => {
                match current {
                    'o' => state = State::ShortO,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ShortO => {
                match current {
                    'r' => state = State::ShortR,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ShortR => {
                match current {
                    't' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ShortT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::SiI => {
                match current {
                    'g' => state = State::SignedG,
                    'z' => state = State::SizeofZ,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::SignedG => {
                match current {
                    'n' => state = State::SignedN,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::SignedN => {
                match current {
                    'e' => state = State::SignedE,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::SignedE => {
                match current {
                    'd' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::SignedD => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::SizeofZ => {
                match current {
                    'e' => state = State::SizeofE,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::SizeofE => {
                match current {
                    'o' => state = State::SizeofO,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::SizeofO => {
                match current {
                    'f' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::SizeofF => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::StT => {
                match current {
                    'a' => state = State::StaticA,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::StaticA => {
                match current {
                    't' => state = State::StaticT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::StaticT => {
                match current {
                    'i' => state = State::StaticI,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::StaticI => {
                match current {
                    'c' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::StaticC => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::StructR => {
                match current {
                    'u' => state = State::StructU,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::StructU => {
                match current {
                    'c' => state = State::StructC,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::StructC => {
                match current {
                    't' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::StructT => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::SwitchW => {
                match current {
                    'i' => state = State::SwitchI,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::SwitchI => {
                match current {
                    't' => state = State::SwitchT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::SwitchT => {
                match current {
                    'c' => state = State::SwitchC,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::SwitchC => {
                match current {
                    'h' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::SwitchH => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('t') => {
                match current {
                    'e' => state = State::TemplateE,
                    'h' => state = State::ThH,
                    'r' => state = State::TrR,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::TemplateE => {
                match current {
                    'm' => state = State::TemplateM,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::TemplateM => {
                match current {
                    'p' => state = State::TemplateP,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::TemplateP => {
                match current {
                    'l' => state = State::TemplateL,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::TemplateL => {
                match current {
                    'a' => state = State::TemplateA,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::TemplateA => {
                match current {
                    't' => state = State::TemplateT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::TemplateT => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::TemplateE2 => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::ThH => {
                match current {
                    'i' => state = State::ThisI,
                    'r' => state = State::ThrowR,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ThisI => {
                match current {
                    's' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ThisS => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::ThrowR => {
                match current {
                    'o' => state = State::ThrowO,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::ThrowO => {
                match current {
                    'w' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::ThrowW => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::TrR => {
                match current {
                    'y' => state = State::KeywordEnd,
                    'u' => state = State::TrueU,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::TrueU => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::TrueE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            // State::TryY => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('u') => {
                match current {
                    'n' => state = State::UnN,
                    'h' => state = State::UsingS,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::UnN => {
                match current {
                    'i' => state = State::UnionI,
                    's' => state = State::UnsignedS,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::UnionI => {
                match current {
                    'o' => state = State::UnionO,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::UnionO => {
                match current {
                    'n' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::UnionN => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::UnsignedS => {
                match current {
                    'i' => state = State::UnsignedI,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::UnsignedI => {
                match current {
                    'g' => state = State::UnsignedG,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::UnsignedG => {
                match current {
                    'n' => state = State::UnsignedN,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::UnsignedN => {
                match current {
                    'e' => state = State::UnsignedE,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::UnsignedE => {
                match current {
                    'd' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::UnsignedD => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::UsingS => {
                match current {
                    'i' => state = State::UsingI,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::UsingI => {
                match current {
                    'n' => state = State::UsingN,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::UsingN => {
                match current {
                    'g' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::UsingG => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('v') => {
                match current {
                    'i' => state = State::VirtualI,
                    'o' => state = State::VoidO,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::VirtualI => {
                match current {
                    'r' => state = State::VirtualR,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::VirtualR => {
                match current {
                    't' => state = State::VirtualT,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::VirtualT => {
                match current {
                    'u' => state = State::VirtualU,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::VirtualU => {
                match current {
                    'a' => state = State::VirtualA,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::VirtualA => {
                match current {
                    'l' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::VirtualL => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::VoidO => {
                match current {
                    'i' => state = State::VoidI,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::VoidI => {
                match current {
                    'd' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::VoidD => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }
            State::Letter('w') => {
                match current {
                    'h' => state = State::WhileH,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::WhileH => {
                match current {
                    'h' => state = State::WhileI,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::WhileI => {
                match current {
                    'l' => state = State::WhileL,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::WhileL => {
                match current {
                    'e' => state = State::KeywordEnd,
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            // State::WhileE => {
            //     match current {
            //         c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
            //         ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
            //             current_idx -= 1;
            //             is_writable = true;
            //         }
            //         _ => state = State::Character(current)
            //     }
            // }


            State::Separator(_) => {}
            State::Identifier(i) => {
                match current {
                    c if c.is_alphanumeric() || c == '_' => state = State::Identifier(c),
                    ' ' | '\n' | '\t' | '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                        current_idx -= 1;
                        is_writable = true;
                    }
                    _ => state = State::Character(current)
                }
            }
            State::Character(_) => {}
            State::Number(_) => {}


            State::Letter(_) => {}
            _ => {}
        }
        current_idx += 1;
    }
    Ok(tokens)
}
