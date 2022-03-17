#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Ampersand {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Ampersand {
    /// Returns `true` if the given [`SyntaxKind`] is a [`AMPERSAND`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`AMPERSAND`]: crate::SyntaxKind::AMPERSAND
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AMPERSAND
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`AMPERSAND`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`AMPERSAND`]: crate::SyntaxKind::AMPERSAND
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct AmpersandEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for AmpersandEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`AMPERSAND_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`AMPERSAND_EQ`]: crate::SyntaxKind::AMPERSAND_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AMPERSAND_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`AMPERSAND_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`AMPERSAND_EQ`]: crate::SyntaxKind::AMPERSAND_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct And {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for And {
    /// Returns `true` if the given [`SyntaxKind`] is a [`AND_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`AND_TOKEN`]: crate::SyntaxKind::AND_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AND_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`AND_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`AND_TOKEN`]: crate::SyntaxKind::AND_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct As {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for As {
    /// Returns `true` if the given [`SyntaxKind`] is a [`AS_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`AS_TOKEN`]: crate::SyntaxKind::AS_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AS_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`AS_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`AS_TOKEN`]: crate::SyntaxKind::AS_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Bang {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Bang {
    /// Returns `true` if the given [`SyntaxKind`] is a [`BANG`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`BANG`]: crate::SyntaxKind::BANG
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BANG
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`BANG`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`BANG`]: crate::SyntaxKind::BANG
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Break {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Break {
    /// Returns `true` if the given [`SyntaxKind`] is a [`BREAK_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`BREAK_TOKEN`]: crate::SyntaxKind::BREAK_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BREAK_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`BREAK_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`BREAK_TOKEN`]: crate::SyntaxKind::BREAK_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Caret {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Caret {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CARET`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CARET`]: crate::SyntaxKind::CARET
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CARET
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CARET`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CARET`]: crate::SyntaxKind::CARET
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct CaretEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for CaretEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CARET_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CARET_EQ`]: crate::SyntaxKind::CARET_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CARET_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CARET_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CARET_EQ`]: crate::SyntaxKind::CARET_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct CharLiteral {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for CharLiteral {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CHAR_LITERAL`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CHAR_LITERAL`]: crate::SyntaxKind::CHAR_LITERAL
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CHAR_LITERAL
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CHAR_LITERAL`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CHAR_LITERAL`]: crate::SyntaxKind::CHAR_LITERAL
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Colon {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Colon {
    /// Returns `true` if the given [`SyntaxKind`] is a [`COLON`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`COLON`]: crate::SyntaxKind::COLON
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COLON
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`COLON`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`COLON`]: crate::SyntaxKind::COLON
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Comma {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Comma {
    /// Returns `true` if the given [`SyntaxKind`] is a [`COMMA`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`COMMA`]: crate::SyntaxKind::COMMA
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COMMA
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`COMMA`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`COMMA`]: crate::SyntaxKind::COMMA
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Comment {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Comment {
    /// Returns `true` if the given [`SyntaxKind`] is a [`COMMENT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`COMMENT`]: crate::SyntaxKind::COMMENT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COMMENT
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`COMMENT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`COMMENT`]: crate::SyntaxKind::COMMENT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Const {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Const {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CONST_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CONST_TOKEN`]: crate::SyntaxKind::CONST_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONST_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CONST_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CONST_TOKEN`]: crate::SyntaxKind::CONST_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Continue {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Continue {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CONTINUE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CONTINUE_TOKEN`]: crate::SyntaxKind::CONTINUE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONTINUE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CONTINUE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CONTINUE_TOKEN`]: crate::SyntaxKind::CONTINUE_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Dot {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Dot {
    /// Returns `true` if the given [`SyntaxKind`] is a [`DOT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`DOT`]: crate::SyntaxKind::DOT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::DOT
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`DOT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`DOT`]: crate::SyntaxKind::DOT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct DoubleColon {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for DoubleColon {
    /// Returns `true` if the given [`SyntaxKind`] is a [`DOUBLE_COLON`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`DOUBLE_COLON`]: crate::SyntaxKind::DOUBLE_COLON
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::DOUBLE_COLON
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`DOUBLE_COLON`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`DOUBLE_COLON`]: crate::SyntaxKind::DOUBLE_COLON
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct DoubleDot {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for DoubleDot {
    /// Returns `true` if the given [`SyntaxKind`] is a [`DOUBLE_DOT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`DOUBLE_DOT`]: crate::SyntaxKind::DOUBLE_DOT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::DOUBLE_DOT
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`DOUBLE_DOT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`DOUBLE_DOT`]: crate::SyntaxKind::DOUBLE_DOT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct DoubleDotEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for DoubleDotEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`DOUBLE_DOT_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`DOUBLE_DOT_EQ`]: crate::SyntaxKind::DOUBLE_DOT_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::DOUBLE_DOT_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`DOUBLE_DOT_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`DOUBLE_DOT_EQ`]: crate::SyntaxKind::DOUBLE_DOT_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Else {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Else {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ELSE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ELSE_TOKEN`]: crate::SyntaxKind::ELSE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ELSE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`ELSE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`ELSE_TOKEN`]: crate::SyntaxKind::ELSE_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Enum {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Enum {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ENUM_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ENUM_TOKEN`]: crate::SyntaxKind::ENUM_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`ENUM_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`ENUM_TOKEN`]: crate::SyntaxKind::ENUM_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Eof {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Eof {
    /// Returns `true` if the given [`SyntaxKind`] is a [`EOF`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`EOF`]: crate::SyntaxKind::EOF
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EOF
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`EOF`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`EOF`]: crate::SyntaxKind::EOF
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Eq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Eq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`EQ`]: crate::SyntaxKind::EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`EQ`]: crate::SyntaxKind::EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Eqeq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Eqeq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`EQEQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`EQEQ`]: crate::SyntaxKind::EQEQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EQEQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`EQEQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`EQEQ`]: crate::SyntaxKind::EQEQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Error {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Error {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ERROR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ERROR`]: crate::SyntaxKind::ERROR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ERROR
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`ERROR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`ERROR`]: crate::SyntaxKind::ERROR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct False {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for False {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FALSE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FALSE_TOKEN`]: crate::SyntaxKind::FALSE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FALSE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`FALSE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`FALSE_TOKEN`]: crate::SyntaxKind::FALSE_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Fn {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Fn {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FN_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FN_TOKEN`]: crate::SyntaxKind::FN_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FN_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`FN_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`FN_TOKEN`]: crate::SyntaxKind::FN_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct For {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for For {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FOR_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FOR_TOKEN`]: crate::SyntaxKind::FOR_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FOR_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`FOR_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`FOR_TOKEN`]: crate::SyntaxKind::FOR_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct HashBrack {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for HashBrack {
    /// Returns `true` if the given [`SyntaxKind`] is a [`HASH_BRACK`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`HASH_BRACK`]: crate::SyntaxKind::HASH_BRACK
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::HASH_BRACK
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`HASH_BRACK`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`HASH_BRACK`]: crate::SyntaxKind::HASH_BRACK
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Ident {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Ident {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IDENT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IDENT`]: crate::SyntaxKind::IDENT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IDENT
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IDENT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IDENT`]: crate::SyntaxKind::IDENT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct If {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for If {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IF_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IF_TOKEN`]: crate::SyntaxKind::IF_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IF_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IF_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IF_TOKEN`]: crate::SyntaxKind::IF_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Impl {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Impl {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IMPL_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IMPL_TOKEN`]: crate::SyntaxKind::IMPL_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IMPL_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IMPL_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IMPL_TOKEN`]: crate::SyntaxKind::IMPL_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct In {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for In {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IN_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IN_TOKEN`]: crate::SyntaxKind::IN_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IN_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IN_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IN_TOKEN`]: crate::SyntaxKind::IN_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct LAngle {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LAngle {
    /// Returns `true` if the given [`SyntaxKind`] is a [`L_ANGLE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`L_ANGLE`]: crate::SyntaxKind::L_ANGLE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_ANGLE
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`L_ANGLE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`L_ANGLE`]: crate::SyntaxKind::L_ANGLE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct LAngleEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LAngleEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`L_ANGLE_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`L_ANGLE_EQ`]: crate::SyntaxKind::L_ANGLE_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_ANGLE_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`L_ANGLE_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`L_ANGLE_EQ`]: crate::SyntaxKind::L_ANGLE_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct LBrack {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LBrack {
    /// Returns `true` if the given [`SyntaxKind`] is a [`L_BRACK`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`L_BRACK`]: crate::SyntaxKind::L_BRACK
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_BRACK
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`L_BRACK`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`L_BRACK`]: crate::SyntaxKind::L_BRACK
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct LCurly {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LCurly {
    /// Returns `true` if the given [`SyntaxKind`] is a [`L_CURLY`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`L_CURLY`]: crate::SyntaxKind::L_CURLY
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_CURLY
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`L_CURLY`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`L_CURLY`]: crate::SyntaxKind::L_CURLY
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct LParen {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LParen {
    /// Returns `true` if the given [`SyntaxKind`] is a [`L_PAREN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`L_PAREN`]: crate::SyntaxKind::L_PAREN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_PAREN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`L_PAREN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`L_PAREN`]: crate::SyntaxKind::L_PAREN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Let {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Let {
    /// Returns `true` if the given [`SyntaxKind`] is a [`LET_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`LET_TOKEN`]: crate::SyntaxKind::LET_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LET_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`LET_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`LET_TOKEN`]: crate::SyntaxKind::LET_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Loop {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Loop {
    /// Returns `true` if the given [`SyntaxKind`] is a [`LOOP_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`LOOP_TOKEN`]: crate::SyntaxKind::LOOP_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LOOP_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`LOOP_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`LOOP_TOKEN`]: crate::SyntaxKind::LOOP_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Match {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Match {
    /// Returns `true` if the given [`SyntaxKind`] is a [`MATCH_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`MATCH_TOKEN`]: crate::SyntaxKind::MATCH_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MATCH_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`MATCH_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`MATCH_TOKEN`]: crate::SyntaxKind::MATCH_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Minus {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Minus {
    /// Returns `true` if the given [`SyntaxKind`] is a [`MINUS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`MINUS`]: crate::SyntaxKind::MINUS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MINUS
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`MINUS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`MINUS`]: crate::SyntaxKind::MINUS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct MinusEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for MinusEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`MINUS_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`MINUS_EQ`]: crate::SyntaxKind::MINUS_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MINUS_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`MINUS_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`MINUS_EQ`]: crate::SyntaxKind::MINUS_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Neq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Neq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`NEQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`NEQ`]: crate::SyntaxKind::NEQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::NEQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`NEQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`NEQ`]: crate::SyntaxKind::NEQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct NumberLiteral {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for NumberLiteral {
    /// Returns `true` if the given [`SyntaxKind`] is a [`NUMBER_LITERAL`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`NUMBER_LITERAL`]: crate::SyntaxKind::NUMBER_LITERAL
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::NUMBER_LITERAL
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`NUMBER_LITERAL`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`NUMBER_LITERAL`]: crate::SyntaxKind::NUMBER_LITERAL
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Or {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Or {
    /// Returns `true` if the given [`SyntaxKind`] is a [`OR_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`OR_TOKEN`]: crate::SyntaxKind::OR_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::OR_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`OR_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`OR_TOKEN`]: crate::SyntaxKind::OR_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Percent {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Percent {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PERCENT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PERCENT`]: crate::SyntaxKind::PERCENT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PERCENT
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PERCENT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PERCENT`]: crate::SyntaxKind::PERCENT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct PercentEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PercentEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PERCENT_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PERCENT_EQ`]: crate::SyntaxKind::PERCENT_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PERCENT_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PERCENT_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PERCENT_EQ`]: crate::SyntaxKind::PERCENT_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Pipe {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Pipe {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PIPE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PIPE`]: crate::SyntaxKind::PIPE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PIPE
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PIPE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PIPE`]: crate::SyntaxKind::PIPE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct PipeEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PipeEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PIPE_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PIPE_EQ`]: crate::SyntaxKind::PIPE_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PIPE_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PIPE_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PIPE_EQ`]: crate::SyntaxKind::PIPE_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Plus {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Plus {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PLUS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PLUS`]: crate::SyntaxKind::PLUS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PLUS
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PLUS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PLUS`]: crate::SyntaxKind::PLUS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct PlusEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PlusEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PLUS_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PLUS_EQ`]: crate::SyntaxKind::PLUS_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PLUS_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PLUS_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PLUS_EQ`]: crate::SyntaxKind::PLUS_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Pub {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Pub {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PUB_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PUB_TOKEN`]: crate::SyntaxKind::PUB_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PUB_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PUB_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PUB_TOKEN`]: crate::SyntaxKind::PUB_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RAngle {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RAngle {
    /// Returns `true` if the given [`SyntaxKind`] is a [`R_ANGLE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`R_ANGLE`]: crate::SyntaxKind::R_ANGLE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_ANGLE
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`R_ANGLE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`R_ANGLE`]: crate::SyntaxKind::R_ANGLE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RAngleEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RAngleEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`R_ANGLE_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`R_ANGLE_EQ`]: crate::SyntaxKind::R_ANGLE_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_ANGLE_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`R_ANGLE_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`R_ANGLE_EQ`]: crate::SyntaxKind::R_ANGLE_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RBrack {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RBrack {
    /// Returns `true` if the given [`SyntaxKind`] is a [`R_BRACK`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`R_BRACK`]: crate::SyntaxKind::R_BRACK
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_BRACK
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`R_BRACK`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`R_BRACK`]: crate::SyntaxKind::R_BRACK
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RCurly {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RCurly {
    /// Returns `true` if the given [`SyntaxKind`] is a [`R_CURLY`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`R_CURLY`]: crate::SyntaxKind::R_CURLY
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_CURLY
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`R_CURLY`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`R_CURLY`]: crate::SyntaxKind::R_CURLY
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RParen {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RParen {
    /// Returns `true` if the given [`SyntaxKind`] is a [`R_PAREN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`R_PAREN`]: crate::SyntaxKind::R_PAREN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_PAREN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`R_PAREN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`R_PAREN`]: crate::SyntaxKind::R_PAREN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Return {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Return {
    /// Returns `true` if the given [`SyntaxKind`] is a [`RETURN_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`RETURN_TOKEN`]: crate::SyntaxKind::RETURN_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RETURN_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`RETURN_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`RETURN_TOKEN`]: crate::SyntaxKind::RETURN_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RightArrow {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RightArrow {
    /// Returns `true` if the given [`SyntaxKind`] is a [`RIGHT_ARROW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`RIGHT_ARROW`]: crate::SyntaxKind::RIGHT_ARROW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RIGHT_ARROW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`RIGHT_ARROW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`RIGHT_ARROW`]: crate::SyntaxKind::RIGHT_ARROW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RightRocket {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RightRocket {
    /// Returns `true` if the given [`SyntaxKind`] is a [`RIGHT_ROCKET`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`RIGHT_ROCKET`]: crate::SyntaxKind::RIGHT_ROCKET
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RIGHT_ROCKET
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`RIGHT_ROCKET`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`RIGHT_ROCKET`]: crate::SyntaxKind::RIGHT_ROCKET
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Semicolon {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Semicolon {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SEMICOLON`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SEMICOLON`]: crate::SyntaxKind::SEMICOLON
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SEMICOLON
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SEMICOLON`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SEMICOLON`]: crate::SyntaxKind::SEMICOLON
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Shl {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Shl {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SHL`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SHL`]: crate::SyntaxKind::SHL
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHL
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SHL`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SHL`]: crate::SyntaxKind::SHL
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct ShlEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ShlEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SHL_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SHL_EQ`]: crate::SyntaxKind::SHL_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHL_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SHL_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SHL_EQ`]: crate::SyntaxKind::SHL_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Shr {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Shr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SHR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SHR`]: crate::SyntaxKind::SHR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHR
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SHR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SHR`]: crate::SyntaxKind::SHR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct ShrEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ShrEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SHR_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SHR_EQ`]: crate::SyntaxKind::SHR_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHR_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SHR_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SHR_EQ`]: crate::SyntaxKind::SHR_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Slash {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Slash {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SLASH`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SLASH`]: crate::SyntaxKind::SLASH
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SLASH
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SLASH`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SLASH`]: crate::SyntaxKind::SLASH
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct SlashEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for SlashEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SLASH_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SLASH_EQ`]: crate::SyntaxKind::SLASH_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SLASH_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SLASH_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SLASH_EQ`]: crate::SyntaxKind::SLASH_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Star {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Star {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STAR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STAR`]: crate::SyntaxKind::STAR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STAR
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`STAR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`STAR`]: crate::SyntaxKind::STAR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct StarEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for StarEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STAR_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STAR_EQ`]: crate::SyntaxKind::STAR_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STAR_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`STAR_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`STAR_EQ`]: crate::SyntaxKind::STAR_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct StringLiteral {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for StringLiteral {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRING_LITERAL`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRING_LITERAL`]: crate::SyntaxKind::STRING_LITERAL
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRING_LITERAL
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`STRING_LITERAL`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`STRING_LITERAL`]: crate::SyntaxKind::STRING_LITERAL
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Struct {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Struct {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRUCT_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRUCT_TOKEN`]: crate::SyntaxKind::STRUCT_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`STRUCT_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`STRUCT_TOKEN`]: crate::SyntaxKind::STRUCT_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Tombstone {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Tombstone {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TOMBSTONE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TOMBSTONE`]: crate::SyntaxKind::TOMBSTONE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TOMBSTONE
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`TOMBSTONE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`TOMBSTONE`]: crate::SyntaxKind::TOMBSTONE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct True {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for True {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TRUE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TRUE_TOKEN`]: crate::SyntaxKind::TRUE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TRUE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`TRUE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`TRUE_TOKEN`]: crate::SyntaxKind::TRUE_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Type {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Type {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TYPE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TYPE_TOKEN`]: crate::SyntaxKind::TYPE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TYPE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`TYPE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`TYPE_TOKEN`]: crate::SyntaxKind::TYPE_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Use {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Use {
    /// Returns `true` if the given [`SyntaxKind`] is a [`USE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`USE_TOKEN`]: crate::SyntaxKind::USE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`USE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`USE_TOKEN`]: crate::SyntaxKind::USE_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct While {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for While {
    /// Returns `true` if the given [`SyntaxKind`] is a [`WHILE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`WHILE_TOKEN`]: crate::SyntaxKind::WHILE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::WHILE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`WHILE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`WHILE_TOKEN`]: crate::SyntaxKind::WHILE_TOKEN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Whitespace {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Whitespace {
    /// Returns `true` if the given [`SyntaxKind`] is a [`WHITESPACE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`WHITESPACE`]: crate::SyntaxKind::WHITESPACE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::WHITESPACE
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`WHITESPACE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`WHITESPACE`]: crate::SyntaxKind::WHITESPACE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum AssignOp {
    AmpersandEq(crate::ast::tokens::AmpersandEq),
    CaretEq(crate::ast::tokens::CaretEq),
    Eq(crate::ast::tokens::Eq),
    MinusEq(crate::ast::tokens::MinusEq),
    PercentEq(crate::ast::tokens::PercentEq),
    PipeEq(crate::ast::tokens::PipeEq),
    PlusEq(crate::ast::tokens::PlusEq),
    ShlEq(crate::ast::tokens::ShlEq),
    ShrEq(crate::ast::tokens::ShrEq),
    SlashEq(crate::ast::tokens::SlashEq),
    StarEq(crate::ast::tokens::StarEq),
}
impl AssignOp {
    pub fn is_ampersand_eq(&self) -> bool {
        ::core::matches!(self, Self::AmpersandEq(..))
    }
    pub fn is_caret_eq(&self) -> bool {
        ::core::matches!(self, Self::CaretEq(..))
    }
    pub fn is_eq(&self) -> bool {
        ::core::matches!(self, Self::Eq(..))
    }
    pub fn is_minus_eq(&self) -> bool {
        ::core::matches!(self, Self::MinusEq(..))
    }
    pub fn is_percent_eq(&self) -> bool {
        ::core::matches!(self, Self::PercentEq(..))
    }
    pub fn is_pipe_eq(&self) -> bool {
        ::core::matches!(self, Self::PipeEq(..))
    }
    pub fn is_plus_eq(&self) -> bool {
        ::core::matches!(self, Self::PlusEq(..))
    }
    pub fn is_shl_eq(&self) -> bool {
        ::core::matches!(self, Self::ShlEq(..))
    }
    pub fn is_shr_eq(&self) -> bool {
        ::core::matches!(self, Self::ShrEq(..))
    }
    pub fn is_slash_eq(&self) -> bool {
        ::core::matches!(self, Self::SlashEq(..))
    }
    pub fn is_star_eq(&self) -> bool {
        ::core::matches!(self, Self::StarEq(..))
    }
    pub fn as_ampersand_eq(&self) -> ::core::option::Option<&crate::ast::tokens::AmpersandEq> {
        if let Self::AmpersandEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_caret_eq(&self) -> ::core::option::Option<&crate::ast::tokens::CaretEq> {
        if let Self::CaretEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_eq(&self) -> ::core::option::Option<&crate::ast::tokens::Eq> {
        if let Self::Eq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_minus_eq(&self) -> ::core::option::Option<&crate::ast::tokens::MinusEq> {
        if let Self::MinusEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_percent_eq(&self) -> ::core::option::Option<&crate::ast::tokens::PercentEq> {
        if let Self::PercentEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_pipe_eq(&self) -> ::core::option::Option<&crate::ast::tokens::PipeEq> {
        if let Self::PipeEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_plus_eq(&self) -> ::core::option::Option<&crate::ast::tokens::PlusEq> {
        if let Self::PlusEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_shl_eq(&self) -> ::core::option::Option<&crate::ast::tokens::ShlEq> {
        if let Self::ShlEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_shr_eq(&self) -> ::core::option::Option<&crate::ast::tokens::ShrEq> {
        if let Self::ShrEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_slash_eq(&self) -> ::core::option::Option<&crate::ast::tokens::SlashEq> {
        if let Self::SlashEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_star_eq(&self) -> ::core::option::Option<&crate::ast::tokens::StarEq> {
        if let Self::StarEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_ampersand_eq(
        self,
    ) -> ::core::result::Result<crate::ast::tokens::AmpersandEq, Self> {
        if let Self::AmpersandEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_caret_eq(self) -> ::core::result::Result<crate::ast::tokens::CaretEq, Self> {
        if let Self::CaretEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_eq(self) -> ::core::result::Result<crate::ast::tokens::Eq, Self> {
        if let Self::Eq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_minus_eq(self) -> ::core::result::Result<crate::ast::tokens::MinusEq, Self> {
        if let Self::MinusEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_percent_eq(self) -> ::core::result::Result<crate::ast::tokens::PercentEq, Self> {
        if let Self::PercentEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_pipe_eq(self) -> ::core::result::Result<crate::ast::tokens::PipeEq, Self> {
        if let Self::PipeEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_plus_eq(self) -> ::core::result::Result<crate::ast::tokens::PlusEq, Self> {
        if let Self::PlusEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_shl_eq(self) -> ::core::result::Result<crate::ast::tokens::ShlEq, Self> {
        if let Self::ShlEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_shr_eq(self) -> ::core::result::Result<crate::ast::tokens::ShrEq, Self> {
        if let Self::ShrEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_slash_eq(self) -> ::core::result::Result<crate::ast::tokens::SlashEq, Self> {
        if let Self::SlashEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_star_eq(self) -> ::core::result::Result<crate::ast::tokens::StarEq, Self> {
        if let Self::StarEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    #[track_caller]
    pub fn to_ampersand_eq(self) -> crate::ast::tokens::AmpersandEq {
        if let Self::AmpersandEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "AmpersandEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_caret_eq(self) -> crate::ast::tokens::CaretEq {
        if let Self::CaretEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "CaretEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_eq(self) -> crate::ast::tokens::Eq {
        if let Self::Eq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "Eq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_minus_eq(self) -> crate::ast::tokens::MinusEq {
        if let Self::MinusEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "MinusEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_percent_eq(self) -> crate::ast::tokens::PercentEq {
        if let Self::PercentEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "PercentEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_pipe_eq(self) -> crate::ast::tokens::PipeEq {
        if let Self::PipeEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "PipeEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_plus_eq(self) -> crate::ast::tokens::PlusEq {
        if let Self::PlusEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "PlusEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_shl_eq(self) -> crate::ast::tokens::ShlEq {
        if let Self::ShlEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "ShlEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_shr_eq(self) -> crate::ast::tokens::ShrEq {
        if let Self::ShrEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "ShrEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_slash_eq(self) -> crate::ast::tokens::SlashEq {
        if let Self::SlashEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "SlashEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_star_eq(self) -> crate::ast::tokens::StarEq {
        if let Self::StarEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "AssignOp",
                "StarEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
}
impl crate::ast::AstToken for AssignOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::tokens::AmpersandEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::CaretEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Eq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::MinusEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::PercentEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::PipeEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::PlusEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::ShlEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::ShrEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::SlashEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::StarEq as crate::ast::AstToken>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            kind if <crate::ast::tokens::AmpersandEq as crate::ast::AstToken>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::tokens::AmpersandEq as crate::ast::AstToken>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::AmpersandEq(node)))
            }
            kind if <crate::ast::tokens::CaretEq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::CaretEq as crate::ast::AstToken>::cast(syntax)
                {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::CaretEq(node)))
            }
            kind if <crate::ast::tokens::Eq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Eq as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Eq(node)))
            }
            kind if <crate::ast::tokens::MinusEq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::MinusEq as crate::ast::AstToken>::cast(syntax)
                {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::MinusEq(node)))
            }
            kind if <crate::ast::tokens::PercentEq as crate::ast::AstToken>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::tokens::PercentEq as crate::ast::AstToken>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::PercentEq(node)))
            }
            kind if <crate::ast::tokens::PipeEq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::PipeEq as crate::ast::AstToken>::cast(syntax)
                {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::PipeEq(node)))
            }
            kind if <crate::ast::tokens::PlusEq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::PlusEq as crate::ast::AstToken>::cast(syntax)
                {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::PlusEq(node)))
            }
            kind if <crate::ast::tokens::ShlEq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::ShlEq as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ShlEq(node)))
            }
            kind if <crate::ast::tokens::ShrEq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::ShrEq as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ShrEq(node)))
            }
            kind if <crate::ast::tokens::SlashEq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::SlashEq as crate::ast::AstToken>::cast(syntax)
                {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::SlashEq(node)))
            }
            kind if <crate::ast::tokens::StarEq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::StarEq as crate::ast::AstToken>::cast(syntax)
                {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::StarEq(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::AmpersandEq(syntax) => {
                <crate::ast::tokens::AmpersandEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::CaretEq(syntax) => {
                <crate::ast::tokens::CaretEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::Eq(syntax) => <crate::ast::tokens::Eq as crate::ast::AstToken>::syntax(syntax),
            Self::MinusEq(syntax) => {
                <crate::ast::tokens::MinusEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::PercentEq(syntax) => {
                <crate::ast::tokens::PercentEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::PipeEq(syntax) => {
                <crate::ast::tokens::PipeEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::PlusEq(syntax) => {
                <crate::ast::tokens::PlusEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::ShlEq(syntax) => {
                <crate::ast::tokens::ShlEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::ShrEq(syntax) => {
                <crate::ast::tokens::ShrEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::SlashEq(syntax) => {
                <crate::ast::tokens::SlashEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::StarEq(syntax) => {
                <crate::ast::tokens::StarEq as crate::ast::AstToken>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::AmpersandEq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::AmpersandEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::CaretEq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::CaretEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::Eq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::Eq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::MinusEq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::MinusEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::PercentEq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::PercentEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::PipeEq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::PipeEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::PlusEq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::PlusEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::ShlEq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::ShlEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::ShrEq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::ShrEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::SlashEq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::SlashEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<AssignOp> for crate::ast::tokens::StarEq {
    type Error = AssignOp;
    #[inline]
    fn try_from(value: AssignOp) -> ::core::result::Result<Self, Self::Error> {
        if let AssignOp::StarEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::tokens::AmpersandEq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::AmpersandEq) -> Self {
        Self::AmpersandEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::CaretEq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::CaretEq) -> Self {
        Self::CaretEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Eq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::Eq) -> Self {
        Self::Eq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::MinusEq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::MinusEq) -> Self {
        Self::MinusEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::PercentEq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::PercentEq) -> Self {
        Self::PercentEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::PipeEq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::PipeEq) -> Self {
        Self::PipeEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::PlusEq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::PlusEq) -> Self {
        Self::PlusEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::ShlEq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::ShlEq) -> Self {
        Self::ShlEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::ShrEq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::ShrEq) -> Self {
        Self::ShrEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::SlashEq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::SlashEq) -> Self {
        Self::SlashEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::StarEq> for AssignOp {
    #[inline]
    fn from(value: crate::ast::tokens::StarEq) -> Self {
        Self::StarEq(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum BinOp {
    Ampersand(crate::ast::tokens::Ampersand),
    And(crate::ast::tokens::And),
    Caret(crate::ast::tokens::Caret),
    Eqeq(crate::ast::tokens::Eqeq),
    LAngle(crate::ast::tokens::LAngle),
    LAngleEq(crate::ast::tokens::LAngleEq),
    Minus(crate::ast::tokens::Minus),
    Neq(crate::ast::tokens::Neq),
    Or(crate::ast::tokens::Or),
    Percent(crate::ast::tokens::Percent),
    Pipe(crate::ast::tokens::Pipe),
    Plus(crate::ast::tokens::Plus),
    RAngle(crate::ast::tokens::RAngle),
    RAngleEq(crate::ast::tokens::RAngleEq),
    Shl(crate::ast::tokens::Shl),
    Shr(crate::ast::tokens::Shr),
    Slash(crate::ast::tokens::Slash),
    Star(crate::ast::tokens::Star),
}
impl BinOp {
    pub fn is_ampersand(&self) -> bool {
        ::core::matches!(self, Self::Ampersand(..))
    }
    pub fn is_and(&self) -> bool {
        ::core::matches!(self, Self::And(..))
    }
    pub fn is_caret(&self) -> bool {
        ::core::matches!(self, Self::Caret(..))
    }
    pub fn is_eqeq(&self) -> bool {
        ::core::matches!(self, Self::Eqeq(..))
    }
    pub fn is_l_angle(&self) -> bool {
        ::core::matches!(self, Self::LAngle(..))
    }
    pub fn is_l_angle_eq(&self) -> bool {
        ::core::matches!(self, Self::LAngleEq(..))
    }
    pub fn is_minus(&self) -> bool {
        ::core::matches!(self, Self::Minus(..))
    }
    pub fn is_neq(&self) -> bool {
        ::core::matches!(self, Self::Neq(..))
    }
    pub fn is_or(&self) -> bool {
        ::core::matches!(self, Self::Or(..))
    }
    pub fn is_percent(&self) -> bool {
        ::core::matches!(self, Self::Percent(..))
    }
    pub fn is_pipe(&self) -> bool {
        ::core::matches!(self, Self::Pipe(..))
    }
    pub fn is_plus(&self) -> bool {
        ::core::matches!(self, Self::Plus(..))
    }
    pub fn is_r_angle(&self) -> bool {
        ::core::matches!(self, Self::RAngle(..))
    }
    pub fn is_r_angle_eq(&self) -> bool {
        ::core::matches!(self, Self::RAngleEq(..))
    }
    pub fn is_shl(&self) -> bool {
        ::core::matches!(self, Self::Shl(..))
    }
    pub fn is_shr(&self) -> bool {
        ::core::matches!(self, Self::Shr(..))
    }
    pub fn is_slash(&self) -> bool {
        ::core::matches!(self, Self::Slash(..))
    }
    pub fn is_star(&self) -> bool {
        ::core::matches!(self, Self::Star(..))
    }
    pub fn as_ampersand(&self) -> ::core::option::Option<&crate::ast::tokens::Ampersand> {
        if let Self::Ampersand(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_and(&self) -> ::core::option::Option<&crate::ast::tokens::And> {
        if let Self::And(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_caret(&self) -> ::core::option::Option<&crate::ast::tokens::Caret> {
        if let Self::Caret(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_eqeq(&self) -> ::core::option::Option<&crate::ast::tokens::Eqeq> {
        if let Self::Eqeq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_l_angle(&self) -> ::core::option::Option<&crate::ast::tokens::LAngle> {
        if let Self::LAngle(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_l_angle_eq(&self) -> ::core::option::Option<&crate::ast::tokens::LAngleEq> {
        if let Self::LAngleEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_minus(&self) -> ::core::option::Option<&crate::ast::tokens::Minus> {
        if let Self::Minus(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_neq(&self) -> ::core::option::Option<&crate::ast::tokens::Neq> {
        if let Self::Neq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_or(&self) -> ::core::option::Option<&crate::ast::tokens::Or> {
        if let Self::Or(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_percent(&self) -> ::core::option::Option<&crate::ast::tokens::Percent> {
        if let Self::Percent(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_pipe(&self) -> ::core::option::Option<&crate::ast::tokens::Pipe> {
        if let Self::Pipe(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_plus(&self) -> ::core::option::Option<&crate::ast::tokens::Plus> {
        if let Self::Plus(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_r_angle(&self) -> ::core::option::Option<&crate::ast::tokens::RAngle> {
        if let Self::RAngle(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_r_angle_eq(&self) -> ::core::option::Option<&crate::ast::tokens::RAngleEq> {
        if let Self::RAngleEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_shl(&self) -> ::core::option::Option<&crate::ast::tokens::Shl> {
        if let Self::Shl(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_shr(&self) -> ::core::option::Option<&crate::ast::tokens::Shr> {
        if let Self::Shr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_slash(&self) -> ::core::option::Option<&crate::ast::tokens::Slash> {
        if let Self::Slash(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_star(&self) -> ::core::option::Option<&crate::ast::tokens::Star> {
        if let Self::Star(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_ampersand(self) -> ::core::result::Result<crate::ast::tokens::Ampersand, Self> {
        if let Self::Ampersand(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_and(self) -> ::core::result::Result<crate::ast::tokens::And, Self> {
        if let Self::And(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_caret(self) -> ::core::result::Result<crate::ast::tokens::Caret, Self> {
        if let Self::Caret(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_eqeq(self) -> ::core::result::Result<crate::ast::tokens::Eqeq, Self> {
        if let Self::Eqeq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_l_angle(self) -> ::core::result::Result<crate::ast::tokens::LAngle, Self> {
        if let Self::LAngle(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_l_angle_eq(self) -> ::core::result::Result<crate::ast::tokens::LAngleEq, Self> {
        if let Self::LAngleEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_minus(self) -> ::core::result::Result<crate::ast::tokens::Minus, Self> {
        if let Self::Minus(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_neq(self) -> ::core::result::Result<crate::ast::tokens::Neq, Self> {
        if let Self::Neq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_or(self) -> ::core::result::Result<crate::ast::tokens::Or, Self> {
        if let Self::Or(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_percent(self) -> ::core::result::Result<crate::ast::tokens::Percent, Self> {
        if let Self::Percent(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_pipe(self) -> ::core::result::Result<crate::ast::tokens::Pipe, Self> {
        if let Self::Pipe(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_plus(self) -> ::core::result::Result<crate::ast::tokens::Plus, Self> {
        if let Self::Plus(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_r_angle(self) -> ::core::result::Result<crate::ast::tokens::RAngle, Self> {
        if let Self::RAngle(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_r_angle_eq(self) -> ::core::result::Result<crate::ast::tokens::RAngleEq, Self> {
        if let Self::RAngleEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_shl(self) -> ::core::result::Result<crate::ast::tokens::Shl, Self> {
        if let Self::Shl(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_shr(self) -> ::core::result::Result<crate::ast::tokens::Shr, Self> {
        if let Self::Shr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_slash(self) -> ::core::result::Result<crate::ast::tokens::Slash, Self> {
        if let Self::Slash(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_star(self) -> ::core::result::Result<crate::ast::tokens::Star, Self> {
        if let Self::Star(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    #[track_caller]
    pub fn to_ampersand(self) -> crate::ast::tokens::Ampersand {
        if let Self::Ampersand(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Ampersand",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_and(self) -> crate::ast::tokens::And {
        if let Self::And(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "And",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_caret(self) -> crate::ast::tokens::Caret {
        if let Self::Caret(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Caret",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_eqeq(self) -> crate::ast::tokens::Eqeq {
        if let Self::Eqeq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Eqeq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_l_angle(self) -> crate::ast::tokens::LAngle {
        if let Self::LAngle(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "LAngle",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_l_angle_eq(self) -> crate::ast::tokens::LAngleEq {
        if let Self::LAngleEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "LAngleEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_minus(self) -> crate::ast::tokens::Minus {
        if let Self::Minus(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Minus",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_neq(self) -> crate::ast::tokens::Neq {
        if let Self::Neq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Neq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_or(self) -> crate::ast::tokens::Or {
        if let Self::Or(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Or",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_percent(self) -> crate::ast::tokens::Percent {
        if let Self::Percent(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Percent",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_pipe(self) -> crate::ast::tokens::Pipe {
        if let Self::Pipe(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Pipe",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_plus(self) -> crate::ast::tokens::Plus {
        if let Self::Plus(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Plus",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_r_angle(self) -> crate::ast::tokens::RAngle {
        if let Self::RAngle(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "RAngle",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_r_angle_eq(self) -> crate::ast::tokens::RAngleEq {
        if let Self::RAngleEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "RAngleEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_shl(self) -> crate::ast::tokens::Shl {
        if let Self::Shl(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Shl",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_shr(self) -> crate::ast::tokens::Shr {
        if let Self::Shr(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Shr",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_slash(self) -> crate::ast::tokens::Slash {
        if let Self::Slash(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Slash",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_star(self) -> crate::ast::tokens::Star {
        if let Self::Star(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "BinOp",
                "Star",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
}
impl crate::ast::AstToken for BinOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::tokens::Ampersand as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::And as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Caret as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Eqeq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::LAngle as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::LAngleEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Minus as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Neq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Or as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Percent as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Pipe as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Plus as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::RAngle as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::RAngleEq as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Shl as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Shr as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Slash as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Star as crate::ast::AstToken>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            kind if <crate::ast::tokens::Ampersand as crate::ast::AstToken>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::tokens::Ampersand as crate::ast::AstToken>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Ampersand(node)))
            }
            kind if <crate::ast::tokens::And as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::And as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::And(node)))
            }
            kind if <crate::ast::tokens::Caret as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Caret as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Caret(node)))
            }
            kind if <crate::ast::tokens::Eqeq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Eqeq as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Eqeq(node)))
            }
            kind if <crate::ast::tokens::LAngle as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::LAngle as crate::ast::AstToken>::cast(syntax)
                {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::LAngle(node)))
            }
            kind if <crate::ast::tokens::LAngleEq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node =
                    match <crate::ast::tokens::LAngleEq as crate::ast::AstToken>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::LAngleEq(node)))
            }
            kind if <crate::ast::tokens::Minus as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Minus as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Minus(node)))
            }
            kind if <crate::ast::tokens::Neq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Neq as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Neq(node)))
            }
            kind if <crate::ast::tokens::Or as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Or as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Or(node)))
            }
            kind if <crate::ast::tokens::Percent as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Percent as crate::ast::AstToken>::cast(syntax)
                {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Percent(node)))
            }
            kind if <crate::ast::tokens::Pipe as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Pipe as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Pipe(node)))
            }
            kind if <crate::ast::tokens::Plus as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Plus as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Plus(node)))
            }
            kind if <crate::ast::tokens::RAngle as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::RAngle as crate::ast::AstToken>::cast(syntax)
                {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RAngle(node)))
            }
            kind if <crate::ast::tokens::RAngleEq as crate::ast::AstToken>::can_cast_from(kind) => {
                let node =
                    match <crate::ast::tokens::RAngleEq as crate::ast::AstToken>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RAngleEq(node)))
            }
            kind if <crate::ast::tokens::Shl as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Shl as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Shl(node)))
            }
            kind if <crate::ast::tokens::Shr as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Shr as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Shr(node)))
            }
            kind if <crate::ast::tokens::Slash as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Slash as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Slash(node)))
            }
            kind if <crate::ast::tokens::Star as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Star as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Star(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::Ampersand(syntax) => {
                <crate::ast::tokens::Ampersand as crate::ast::AstToken>::syntax(syntax)
            }
            Self::And(syntax) => <crate::ast::tokens::And as crate::ast::AstToken>::syntax(syntax),
            Self::Caret(syntax) => {
                <crate::ast::tokens::Caret as crate::ast::AstToken>::syntax(syntax)
            }
            Self::Eqeq(syntax) => {
                <crate::ast::tokens::Eqeq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::LAngle(syntax) => {
                <crate::ast::tokens::LAngle as crate::ast::AstToken>::syntax(syntax)
            }
            Self::LAngleEq(syntax) => {
                <crate::ast::tokens::LAngleEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::Minus(syntax) => {
                <crate::ast::tokens::Minus as crate::ast::AstToken>::syntax(syntax)
            }
            Self::Neq(syntax) => <crate::ast::tokens::Neq as crate::ast::AstToken>::syntax(syntax),
            Self::Or(syntax) => <crate::ast::tokens::Or as crate::ast::AstToken>::syntax(syntax),
            Self::Percent(syntax) => {
                <crate::ast::tokens::Percent as crate::ast::AstToken>::syntax(syntax)
            }
            Self::Pipe(syntax) => {
                <crate::ast::tokens::Pipe as crate::ast::AstToken>::syntax(syntax)
            }
            Self::Plus(syntax) => {
                <crate::ast::tokens::Plus as crate::ast::AstToken>::syntax(syntax)
            }
            Self::RAngle(syntax) => {
                <crate::ast::tokens::RAngle as crate::ast::AstToken>::syntax(syntax)
            }
            Self::RAngleEq(syntax) => {
                <crate::ast::tokens::RAngleEq as crate::ast::AstToken>::syntax(syntax)
            }
            Self::Shl(syntax) => <crate::ast::tokens::Shl as crate::ast::AstToken>::syntax(syntax),
            Self::Shr(syntax) => <crate::ast::tokens::Shr as crate::ast::AstToken>::syntax(syntax),
            Self::Slash(syntax) => {
                <crate::ast::tokens::Slash as crate::ast::AstToken>::syntax(syntax)
            }
            Self::Star(syntax) => {
                <crate::ast::tokens::Star as crate::ast::AstToken>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Ampersand {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Ampersand(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::And {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::And(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Caret {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Caret(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Eqeq {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Eqeq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::LAngle {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::LAngle(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::LAngleEq {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::LAngleEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Minus {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Minus(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Neq {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Neq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Or {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Or(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Percent {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Percent(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Pipe {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Pipe(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Plus {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Plus(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::RAngle {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::RAngle(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::RAngleEq {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::RAngleEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Shl {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Shl(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Shr {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Shr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Slash {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Slash(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<BinOp> for crate::ast::tokens::Star {
    type Error = BinOp;
    #[inline]
    fn try_from(value: BinOp) -> ::core::result::Result<Self, Self::Error> {
        if let BinOp::Star(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::tokens::Ampersand> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Ampersand) -> Self {
        Self::Ampersand(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::And> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::And) -> Self {
        Self::And(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Caret> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Caret) -> Self {
        Self::Caret(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Eqeq> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Eqeq) -> Self {
        Self::Eqeq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::LAngle> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::LAngle) -> Self {
        Self::LAngle(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::LAngleEq> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::LAngleEq) -> Self {
        Self::LAngleEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Minus> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Minus) -> Self {
        Self::Minus(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Neq> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Neq) -> Self {
        Self::Neq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Or> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Or) -> Self {
        Self::Or(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Percent> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Percent) -> Self {
        Self::Percent(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Pipe> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Pipe) -> Self {
        Self::Pipe(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Plus> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Plus) -> Self {
        Self::Plus(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::RAngle> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::RAngle) -> Self {
        Self::RAngle(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::RAngleEq> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::RAngleEq) -> Self {
        Self::RAngleEq(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Shl> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Shl) -> Self {
        Self::Shl(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Shr> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Shr) -> Self {
        Self::Shr(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Slash> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Slash) -> Self {
        Self::Slash(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Star> for BinOp {
    #[inline]
    fn from(value: crate::ast::tokens::Star) -> Self {
        Self::Star(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Bool {
    False(crate::ast::tokens::False),
    True(crate::ast::tokens::True),
}
impl Bool {
    pub fn is_false(&self) -> bool {
        ::core::matches!(self, Self::False(..))
    }
    pub fn is_true(&self) -> bool {
        ::core::matches!(self, Self::True(..))
    }
    pub fn as_false(&self) -> ::core::option::Option<&crate::ast::tokens::False> {
        if let Self::False(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_true(&self) -> ::core::option::Option<&crate::ast::tokens::True> {
        if let Self::True(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_false(self) -> ::core::result::Result<crate::ast::tokens::False, Self> {
        if let Self::False(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_true(self) -> ::core::result::Result<crate::ast::tokens::True, Self> {
        if let Self::True(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    #[track_caller]
    pub fn to_false(self) -> crate::ast::tokens::False {
        if let Self::False(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "Bool",
                "False",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_true(self) -> crate::ast::tokens::True {
        if let Self::True(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "Bool",
                "True",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
}
impl crate::ast::AstToken for Bool {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::tokens::False as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::True as crate::ast::AstToken>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            kind if <crate::ast::tokens::False as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::False as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::False(node)))
            }
            kind if <crate::ast::tokens::True as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::True as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::True(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::False(syntax) => {
                <crate::ast::tokens::False as crate::ast::AstToken>::syntax(syntax)
            }
            Self::True(syntax) => {
                <crate::ast::tokens::True as crate::ast::AstToken>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<Bool> for crate::ast::tokens::False {
    type Error = Bool;
    #[inline]
    fn try_from(value: Bool) -> ::core::result::Result<Self, Self::Error> {
        if let Bool::False(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Bool> for crate::ast::tokens::True {
    type Error = Bool;
    #[inline]
    fn try_from(value: Bool) -> ::core::result::Result<Self, Self::Error> {
        if let Bool::True(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::tokens::False> for Bool {
    #[inline]
    fn from(value: crate::ast::tokens::False) -> Self {
        Self::False(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::True> for Bool {
    #[inline]
    fn from(value: crate::ast::tokens::True) -> Self {
        Self::True(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum RangeOp {
    DoubleDot(crate::ast::tokens::DoubleDot),
    DoubleDotEq(crate::ast::tokens::DoubleDotEq),
}
impl RangeOp {
    pub fn is_double_dot(&self) -> bool {
        ::core::matches!(self, Self::DoubleDot(..))
    }
    pub fn is_double_dot_eq(&self) -> bool {
        ::core::matches!(self, Self::DoubleDotEq(..))
    }
    pub fn as_double_dot(&self) -> ::core::option::Option<&crate::ast::tokens::DoubleDot> {
        if let Self::DoubleDot(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_double_dot_eq(&self) -> ::core::option::Option<&crate::ast::tokens::DoubleDotEq> {
        if let Self::DoubleDotEq(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_double_dot(self) -> ::core::result::Result<crate::ast::tokens::DoubleDot, Self> {
        if let Self::DoubleDot(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_double_dot_eq(
        self,
    ) -> ::core::result::Result<crate::ast::tokens::DoubleDotEq, Self> {
        if let Self::DoubleDotEq(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    #[track_caller]
    pub fn to_double_dot(self) -> crate::ast::tokens::DoubleDot {
        if let Self::DoubleDot(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "RangeOp",
                "DoubleDot",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_double_dot_eq(self) -> crate::ast::tokens::DoubleDotEq {
        if let Self::DoubleDotEq(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "RangeOp",
                "DoubleDotEq",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
}
impl crate::ast::AstToken for RangeOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::tokens::DoubleDot as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::DoubleDotEq as crate::ast::AstToken>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            kind if <crate::ast::tokens::DoubleDot as crate::ast::AstToken>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::tokens::DoubleDot as crate::ast::AstToken>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::DoubleDot(node)))
            }
            kind if <crate::ast::tokens::DoubleDotEq as crate::ast::AstToken>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::tokens::DoubleDotEq as crate::ast::AstToken>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::DoubleDotEq(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::DoubleDot(syntax) => {
                <crate::ast::tokens::DoubleDot as crate::ast::AstToken>::syntax(syntax)
            }
            Self::DoubleDotEq(syntax) => {
                <crate::ast::tokens::DoubleDotEq as crate::ast::AstToken>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<RangeOp> for crate::ast::tokens::DoubleDot {
    type Error = RangeOp;
    #[inline]
    fn try_from(value: RangeOp) -> ::core::result::Result<Self, Self::Error> {
        if let RangeOp::DoubleDot(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<RangeOp> for crate::ast::tokens::DoubleDotEq {
    type Error = RangeOp;
    #[inline]
    fn try_from(value: RangeOp) -> ::core::result::Result<Self, Self::Error> {
        if let RangeOp::DoubleDotEq(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::tokens::DoubleDot> for RangeOp {
    #[inline]
    fn from(value: crate::ast::tokens::DoubleDot) -> Self {
        Self::DoubleDot(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::DoubleDotEq> for RangeOp {
    #[inline]
    fn from(value: crate::ast::tokens::DoubleDotEq) -> Self {
        Self::DoubleDotEq(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum UnaryOp {
    Bang(crate::ast::tokens::Bang),
    Minus(crate::ast::tokens::Minus),
}
impl UnaryOp {
    pub fn is_bang(&self) -> bool {
        ::core::matches!(self, Self::Bang(..))
    }
    pub fn is_minus(&self) -> bool {
        ::core::matches!(self, Self::Minus(..))
    }
    pub fn as_bang(&self) -> ::core::option::Option<&crate::ast::tokens::Bang> {
        if let Self::Bang(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_minus(&self) -> ::core::option::Option<&crate::ast::tokens::Minus> {
        if let Self::Minus(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_bang(self) -> ::core::result::Result<crate::ast::tokens::Bang, Self> {
        if let Self::Bang(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_minus(self) -> ::core::result::Result<crate::ast::tokens::Minus, Self> {
        if let Self::Minus(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    #[track_caller]
    pub fn to_bang(self) -> crate::ast::tokens::Bang {
        if let Self::Bang(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "UnaryOp",
                "Bang",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
    #[track_caller]
    pub fn to_minus(self) -> crate::ast::tokens::Minus {
        if let Self::Minus(syntax) = self {
            syntax
        } else {
            crate::ast::support::failed_enum_to_node_cast(
                "UnaryOp",
                "Minus",
                crate::SyntaxToken::kind(<Self as crate::ast::AstToken>::syntax(&self)),
            )
        }
    }
}
impl crate::ast::AstToken for UnaryOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::tokens::Bang as crate::ast::AstToken>::can_cast_from(kind)
            || <crate::ast::tokens::Minus as crate::ast::AstToken>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            kind if <crate::ast::tokens::Bang as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Bang as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Bang(node)))
            }
            kind if <crate::ast::tokens::Minus as crate::ast::AstToken>::can_cast_from(kind) => {
                let node = match <crate::ast::tokens::Minus as crate::ast::AstToken>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Minus(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::Bang(syntax) => {
                <crate::ast::tokens::Bang as crate::ast::AstToken>::syntax(syntax)
            }
            Self::Minus(syntax) => {
                <crate::ast::tokens::Minus as crate::ast::AstToken>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<UnaryOp> for crate::ast::tokens::Bang {
    type Error = UnaryOp;
    #[inline]
    fn try_from(value: UnaryOp) -> ::core::result::Result<Self, Self::Error> {
        if let UnaryOp::Bang(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<UnaryOp> for crate::ast::tokens::Minus {
    type Error = UnaryOp;
    #[inline]
    fn try_from(value: UnaryOp) -> ::core::result::Result<Self, Self::Error> {
        if let UnaryOp::Minus(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::tokens::Bang> for UnaryOp {
    #[inline]
    fn from(value: crate::ast::tokens::Bang) -> Self {
        Self::Bang(value)
    }
}
impl ::core::convert::From<crate::ast::tokens::Minus> for UnaryOp {
    #[inline]
    fn from(value: crate::ast::tokens::Minus) -> Self {
        Self::Minus(value)
    }
}
