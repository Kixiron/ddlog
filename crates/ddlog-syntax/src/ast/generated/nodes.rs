#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct ArrayAccess {
    syntax: crate::SyntaxNode,
}
impl ArrayAccess {
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn l_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LBrack>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn index(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::nth_child::<_, 1usize>(&self.syntax)
    }
    #[inline]
    pub fn r_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RBrack>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ArrayAccess {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ARRAY_ACCESS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ARRAY_ACCESS`]: crate::SyntaxKind::ARRAY_ACCESS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ARRAY_ACCESS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ARRAY_ACCESS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ARRAY_ACCESS`]: crate::SyntaxKind::ARRAY_ACCESS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ArrayExprElem {
    syntax: crate::SyntaxNode,
}
impl ArrayExprElem {
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for ArrayExprElem {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ARRAY_EXPR_ELEM`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ARRAY_EXPR_ELEM`]: crate::SyntaxKind::ARRAY_EXPR_ELEM
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ARRAY_EXPR_ELEM
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ARRAY_EXPR_ELEM`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ARRAY_EXPR_ELEM`]: crate::SyntaxKind::ARRAY_EXPR_ELEM
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ArrayInitExpr {
    syntax: crate::SyntaxNode,
}
impl ArrayInitExpr {
    #[inline]
    pub fn l_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LBrack>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn array_expr_elem(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::ArrayExprElem>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn r_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RBrack>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ArrayInitExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ARRAY_INIT_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ARRAY_INIT_EXPR`]: crate::SyntaxKind::ARRAY_INIT_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ARRAY_INIT_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ARRAY_INIT_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ARRAY_INIT_EXPR`]: crate::SyntaxKind::ARRAY_INIT_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct Assign {
    syntax: crate::SyntaxNode,
}
impl Assign {
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn assign_op(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::AssignOp>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn value(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for Assign {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ASSIGN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ASSIGN`]: crate::SyntaxKind::ASSIGN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ASSIGN
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ASSIGN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ASSIGN`]: crate::SyntaxKind::ASSIGN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct AttrPair {
    syntax: crate::SyntaxNode,
}
impl AttrPair {
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for AttrPair {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ATTR_PAIR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ATTR_PAIR`]: crate::SyntaxKind::ATTR_PAIR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ATTR_PAIR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ATTR_PAIR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ATTR_PAIR`]: crate::SyntaxKind::ATTR_PAIR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct Attribute {
    syntax: crate::SyntaxNode,
}
impl Attribute {
    #[inline]
    pub fn hash_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::HashBrack>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn attr_pairs(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::AttrPair> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RBrack>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Attribute {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ATTRIBUTE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ATTRIBUTE`]: crate::SyntaxKind::ATTRIBUTE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ATTRIBUTE
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ATTRIBUTE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ATTRIBUTE`]: crate::SyntaxKind::ATTRIBUTE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct BinExpr {
    syntax: crate::SyntaxNode,
}
impl BinExpr {
    #[inline]
    pub fn lhs(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn op(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::BinOp>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn rhs(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::nth_child::<_, 1usize>(&self.syntax)
    }
}
impl crate::ast::AstNode for BinExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`BIN_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`BIN_EXPR`]: crate::SyntaxKind::BIN_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BIN_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`BIN_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`BIN_EXPR`]: crate::SyntaxKind::BIN_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct Block {
    syntax: crate::SyntaxNode,
}
impl Block {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn statements(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Stmt> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Block {
    /// Returns `true` if the given [`SyntaxKind`] is a [`BLOCK`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`BLOCK`]: crate::SyntaxKind::BLOCK
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BLOCK
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`BLOCK`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`BLOCK`]: crate::SyntaxKind::BLOCK
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct BracketedStructField {
    syntax: crate::SyntaxNode,
}
impl BracketedStructField {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for BracketedStructField {
    /// Returns `true` if the given [`SyntaxKind`] is a [`BRACKETED_STRUCT_FIELD`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`BRACKETED_STRUCT_FIELD`]: crate::SyntaxKind::BRACKETED_STRUCT_FIELD
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BRACKETED_STRUCT_FIELD
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`BRACKETED_STRUCT_FIELD`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`BRACKETED_STRUCT_FIELD`]: crate::SyntaxKind::BRACKETED_STRUCT_FIELD
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct BracketedStructFields {
    syntax: crate::SyntaxNode,
}
impl BracketedStructFields {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn fields(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::BracketedStructField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for BracketedStructFields {
    /// Returns `true` if the given [`SyntaxKind`] is a [`BRACKETED_STRUCT_FIELDS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`BRACKETED_STRUCT_FIELDS`]: crate::SyntaxKind::BRACKETED_STRUCT_FIELDS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BRACKETED_STRUCT_FIELDS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`BRACKETED_STRUCT_FIELDS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`BRACKETED_STRUCT_FIELDS`]: crate::SyntaxKind::BRACKETED_STRUCT_FIELDS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct BreakExpr {
    syntax: crate::SyntaxNode,
}
impl BreakExpr {
    #[inline]
    pub fn break_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::BreakToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for BreakExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`BREAK_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`BREAK_EXPR`]: crate::SyntaxKind::BREAK_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BREAK_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`BREAK_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`BREAK_EXPR`]: crate::SyntaxKind::BREAK_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ClosureArg {
    syntax: crate::SyntaxNode,
}
impl ClosureArg {
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for ClosureArg {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CLOSURE_ARG`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CLOSURE_ARG`]: crate::SyntaxKind::CLOSURE_ARG
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CLOSURE_ARG
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`CLOSURE_ARG`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`CLOSURE_ARG`]: crate::SyntaxKind::CLOSURE_ARG
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ClosureExpr {
    syntax: crate::SyntaxNode,
}
impl ClosureExpr {
    #[inline]
    pub fn start_pipe(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Pipe>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn args(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::ClosureArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn end_pipe(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Pipe>> {
        crate::ast::support::nth_token::<_, 1usize>(&self.syntax)
    }
    #[inline]
    pub fn body(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for ClosureExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CLOSURE_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CLOSURE_EXPR`]: crate::SyntaxKind::CLOSURE_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CLOSURE_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`CLOSURE_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`CLOSURE_EXPR`]: crate::SyntaxKind::CLOSURE_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ConstDef {
    syntax: crate::SyntaxNode,
}
impl ConstDef {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::ConstToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn value(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Semicolon>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ConstDef {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CONST_DEF`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CONST_DEF`]: crate::SyntaxKind::CONST_DEF
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONST_DEF
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`CONST_DEF`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`CONST_DEF`]: crate::SyntaxKind::CONST_DEF
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ContinueExpr {
    syntax: crate::SyntaxNode,
}
impl ContinueExpr {
    #[inline]
    pub fn continue_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::ContinueToken>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ContinueExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CONTINUE_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CONTINUE_EXPR`]: crate::SyntaxKind::CONTINUE_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONTINUE_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`CONTINUE_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`CONTINUE_EXPR`]: crate::SyntaxKind::CONTINUE_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ElseBlock {
    syntax: crate::SyntaxNode,
}
impl ElseBlock {
    #[inline]
    pub fn else_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::ElseToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for ElseBlock {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ELSE_BLOCK`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ELSE_BLOCK`]: crate::SyntaxKind::ELSE_BLOCK
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ELSE_BLOCK
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ELSE_BLOCK`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ELSE_BLOCK`]: crate::SyntaxKind::ELSE_BLOCK
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct EnumDef {
    syntax: crate::SyntaxNode,
}
impl EnumDef {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::EnumToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn variants(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::EnumVariants>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for EnumDef {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ENUM_DEF`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ENUM_DEF`]: crate::SyntaxKind::ENUM_DEF
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_DEF
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ENUM_DEF`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ENUM_DEF`]: crate::SyntaxKind::ENUM_DEF
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct EnumVariant {
    syntax: crate::SyntaxNode,
}
impl EnumVariant {
    #[inline]
    pub fn variant(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn enum_variant_body(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::EnumVariantBody>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for EnumVariant {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ENUM_VARIANT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ENUM_VARIANT`]: crate::SyntaxKind::ENUM_VARIANT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_VARIANT
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ENUM_VARIANT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ENUM_VARIANT`]: crate::SyntaxKind::ENUM_VARIANT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct EnumVariants {
    syntax: crate::SyntaxNode,
}
impl EnumVariants {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn variants(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::EnumVariant> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for EnumVariants {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ENUM_VARIANTS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ENUM_VARIANTS`]: crate::SyntaxKind::ENUM_VARIANTS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_VARIANTS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ENUM_VARIANTS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ENUM_VARIANTS`]: crate::SyntaxKind::ENUM_VARIANTS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ExprStmt {
    syntax: crate::SyntaxNode,
}
impl ExprStmt {
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolons(
        &self,
    ) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Semicolon> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for ExprStmt {
    /// Returns `true` if the given [`SyntaxKind`] is a [`EXPR_STMT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`EXPR_STMT`]: crate::SyntaxKind::EXPR_STMT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EXPR_STMT
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`EXPR_STMT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`EXPR_STMT`]: crate::SyntaxKind::EXPR_STMT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FieldAccess {
    syntax: crate::SyntaxNode,
}
impl FieldAccess {
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn dot(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Dot>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn field_accessor(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FieldAccessor>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FieldAccess {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FIELD_ACCESS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FIELD_ACCESS`]: crate::SyntaxKind::FIELD_ACCESS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FIELD_ACCESS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FIELD_ACCESS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FIELD_ACCESS`]: crate::SyntaxKind::FIELD_ACCESS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FieldAccessorName {
    syntax: crate::SyntaxNode,
}
impl FieldAccessorName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FieldAccessorName {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FIELD_ACCESSOR_NAME`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FIELD_ACCESSOR_NAME`]: crate::SyntaxKind::FIELD_ACCESSOR_NAME
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FIELD_ACCESSOR_NAME
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FIELD_ACCESSOR_NAME`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FIELD_ACCESSOR_NAME`]: crate::SyntaxKind::FIELD_ACCESSOR_NAME
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ForExpr {
    syntax: crate::SyntaxNode,
}
impl ForExpr {
    #[inline]
    pub fn for_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::ForToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn in_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::InToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn iter(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for ForExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FOR_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FOR_EXPR`]: crate::SyntaxKind::FOR_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FOR_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FOR_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FOR_EXPR`]: crate::SyntaxKind::FOR_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FunctionArg {
    syntax: crate::SyntaxNode,
}
impl FunctionArg {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionArg {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FUNCTION_ARG`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FUNCTION_ARG`]: crate::SyntaxKind::FUNCTION_ARG
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_ARG
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FUNCTION_ARG`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FUNCTION_ARG`]: crate::SyntaxKind::FUNCTION_ARG
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FunctionArgs {
    syntax: crate::SyntaxNode,
}
impl FunctionArgs {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn args(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::FunctionArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionArgs {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FUNCTION_ARGS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FUNCTION_ARGS`]: crate::SyntaxKind::FUNCTION_ARGS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_ARGS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FUNCTION_ARGS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FUNCTION_ARGS`]: crate::SyntaxKind::FUNCTION_ARGS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FunctionCall {
    syntax: crate::SyntaxNode,
}
impl FunctionCall {
    #[inline]
    pub fn func(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn args(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::FunctionCallArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionCall {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FUNCTION_CALL`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FUNCTION_CALL`]: crate::SyntaxKind::FUNCTION_CALL
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_CALL
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FUNCTION_CALL`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FUNCTION_CALL`]: crate::SyntaxKind::FUNCTION_CALL
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FunctionCallArg {
    syntax: crate::SyntaxNode,
}
impl FunctionCallArg {
    #[inline]
    pub fn arg(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionCallArg {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FUNCTION_CALL_ARG`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FUNCTION_CALL_ARG`]: crate::SyntaxKind::FUNCTION_CALL_ARG
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_CALL_ARG
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FUNCTION_CALL_ARG`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FUNCTION_CALL_ARG`]: crate::SyntaxKind::FUNCTION_CALL_ARG
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FunctionDef {
    syntax: crate::SyntaxNode,
}
impl FunctionDef {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::FnToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn generics(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Generics>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn args(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FunctionArgs>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn ret(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FunctionReturn>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn body(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionDef {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FUNCTION_DEF`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FUNCTION_DEF`]: crate::SyntaxKind::FUNCTION_DEF
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_DEF
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FUNCTION_DEF`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FUNCTION_DEF`]: crate::SyntaxKind::FUNCTION_DEF
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FunctionReturn {
    syntax: crate::SyntaxNode,
}
impl FunctionReturn {
    #[inline]
    pub fn right_arrow(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RightArrow>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn return_ty(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionReturn {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FUNCTION_RETURN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FUNCTION_RETURN`]: crate::SyntaxKind::FUNCTION_RETURN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_RETURN
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FUNCTION_RETURN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FUNCTION_RETURN`]: crate::SyntaxKind::FUNCTION_RETURN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FunctionReturnType {
    syntax: crate::SyntaxNode,
}
impl FunctionReturnType {
    #[inline]
    pub fn right_arrow(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RightArrow>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionReturnType {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FUNCTION_RETURN_TYPE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FUNCTION_RETURN_TYPE`]: crate::SyntaxKind::FUNCTION_RETURN_TYPE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_RETURN_TYPE
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FUNCTION_RETURN_TYPE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FUNCTION_RETURN_TYPE`]: crate::SyntaxKind::FUNCTION_RETURN_TYPE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FunctionType {
    syntax: crate::SyntaxNode,
}
impl FunctionType {
    #[inline]
    pub fn fn_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::FnToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn args(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FunctionTypeArgs>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn ret(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FunctionReturnType>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionType {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FUNCTION_TYPE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FUNCTION_TYPE`]: crate::SyntaxKind::FUNCTION_TYPE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_TYPE
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FUNCTION_TYPE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FUNCTION_TYPE`]: crate::SyntaxKind::FUNCTION_TYPE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FunctionTypeArg {
    syntax: crate::SyntaxNode,
}
impl FunctionTypeArg {
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionTypeArg {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FUNCTION_TYPE_ARG`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FUNCTION_TYPE_ARG`]: crate::SyntaxKind::FUNCTION_TYPE_ARG
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_TYPE_ARG
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FUNCTION_TYPE_ARG`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FUNCTION_TYPE_ARG`]: crate::SyntaxKind::FUNCTION_TYPE_ARG
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct FunctionTypeArgs {
    syntax: crate::SyntaxNode,
}
impl FunctionTypeArgs {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn args(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::FunctionTypeArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionTypeArgs {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FUNCTION_TYPE_ARGS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FUNCTION_TYPE_ARGS`]: crate::SyntaxKind::FUNCTION_TYPE_ARGS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_TYPE_ARGS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`FUNCTION_TYPE_ARGS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`FUNCTION_TYPE_ARGS`]: crate::SyntaxKind::FUNCTION_TYPE_ARGS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct GenericArg {
    syntax: crate::SyntaxNode,
}
impl GenericArg {
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for GenericArg {
    /// Returns `true` if the given [`SyntaxKind`] is a [`GENERIC_ARG`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`GENERIC_ARG`]: crate::SyntaxKind::GENERIC_ARG
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::GENERIC_ARG
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`GENERIC_ARG`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`GENERIC_ARG`]: crate::SyntaxKind::GENERIC_ARG
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct GenericType {
    syntax: crate::SyntaxNode,
}
impl GenericType {
    #[inline]
    pub fn path(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Path>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn generics(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Generics>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for GenericType {
    /// Returns `true` if the given [`SyntaxKind`] is a [`GENERIC_TYPE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`GENERIC_TYPE`]: crate::SyntaxKind::GENERIC_TYPE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::GENERIC_TYPE
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`GENERIC_TYPE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`GENERIC_TYPE`]: crate::SyntaxKind::GENERIC_TYPE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct Generics {
    syntax: crate::SyntaxNode,
}
impl Generics {
    #[inline]
    pub fn l_angle(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LAngle>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn generics(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::GenericArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_angle(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RAngle>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Generics {
    /// Returns `true` if the given [`SyntaxKind`] is a [`GENERICS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`GENERICS`]: crate::SyntaxKind::GENERICS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::GENERICS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`GENERICS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`GENERICS`]: crate::SyntaxKind::GENERICS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct IfBlock {
    syntax: crate::SyntaxNode,
}
impl IfBlock {
    #[inline]
    pub fn leading_else(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::ElseToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn if_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::IfToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn cond(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for IfBlock {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IF_BLOCK`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IF_BLOCK`]: crate::SyntaxKind::IF_BLOCK
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IF_BLOCK
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`IF_BLOCK`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`IF_BLOCK`]: crate::SyntaxKind::IF_BLOCK
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct IfExpr {
    syntax: crate::SyntaxNode,
}
impl IfExpr {
    #[inline]
    pub fn if_blocks(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::IfBlock> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn else_block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::ElseBlock>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for IfExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IF_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IF_EXPR`]: crate::SyntaxKind::IF_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IF_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`IF_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`IF_EXPR`]: crate::SyntaxKind::IF_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ImplBlock {
    syntax: crate::SyntaxNode,
}
impl ImplBlock {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::ImplToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn target(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn contents(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::ImplBlockContents>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for ImplBlock {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IMPL_BLOCK`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IMPL_BLOCK`]: crate::SyntaxKind::IMPL_BLOCK
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IMPL_BLOCK
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`IMPL_BLOCK`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`IMPL_BLOCK`]: crate::SyntaxKind::IMPL_BLOCK
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ImplBlockContents {
    syntax: crate::SyntaxNode,
}
impl ImplBlockContents {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn items(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Item> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ImplBlockContents {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IMPL_BLOCK_CONTENTS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IMPL_BLOCK_CONTENTS`]: crate::SyntaxKind::IMPL_BLOCK_CONTENTS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IMPL_BLOCK_CONTENTS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`IMPL_BLOCK_CONTENTS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`IMPL_BLOCK_CONTENTS`]: crate::SyntaxKind::IMPL_BLOCK_CONTENTS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct LoopExpr {
    syntax: crate::SyntaxNode,
}
impl LoopExpr {
    #[inline]
    pub fn loop_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LoopToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for LoopExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`LOOP_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`LOOP_EXPR`]: crate::SyntaxKind::LOOP_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LOOP_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`LOOP_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`LOOP_EXPR`]: crate::SyntaxKind::LOOP_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct MatchArm {
    syntax: crate::SyntaxNode,
}
impl MatchArm {
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn right_rocket(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RightRocket>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn body(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for MatchArm {
    /// Returns `true` if the given [`SyntaxKind`] is a [`MATCH_ARM`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`MATCH_ARM`]: crate::SyntaxKind::MATCH_ARM
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MATCH_ARM
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`MATCH_ARM`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`MATCH_ARM`]: crate::SyntaxKind::MATCH_ARM
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct MatchExpr {
    syntax: crate::SyntaxNode,
}
impl MatchExpr {
    #[inline]
    pub fn match_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::MatchToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn scrutinee(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn match_arms(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::MatchArm> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for MatchExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`MATCH_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`MATCH_EXPR`]: crate::SyntaxKind::MATCH_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MATCH_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`MATCH_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`MATCH_EXPR`]: crate::SyntaxKind::MATCH_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct Modifier {
    syntax: crate::SyntaxNode,
}
impl Modifier {
    #[inline]
    pub fn pub_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::PubToken>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Modifier {
    /// Returns `true` if the given [`SyntaxKind`] is a [`MODIFIER`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`MODIFIER`]: crate::SyntaxKind::MODIFIER
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MODIFIER
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`MODIFIER`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`MODIFIER`]: crate::SyntaxKind::MODIFIER
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct Number {
    syntax: crate::SyntaxNode,
}
impl Number {
    #[inline]
    pub fn number_literal(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::NumberLiteral>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Number {
    /// Returns `true` if the given [`SyntaxKind`] is a [`NUMBER`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`NUMBER`]: crate::SyntaxKind::NUMBER
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::NUMBER
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`NUMBER`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`NUMBER`]: crate::SyntaxKind::NUMBER
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct ParenExpr {
    syntax: crate::SyntaxNode,
}
impl ParenExpr {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn inner(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ParenExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PAREN_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PAREN_EXPR`]: crate::SyntaxKind::PAREN_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PAREN_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`PAREN_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`PAREN_EXPR`]: crate::SyntaxKind::PAREN_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct Path {
    syntax: crate::SyntaxNode,
}
impl Path {
    #[inline]
    pub fn double_colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::DoubleColon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn head(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn tails(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::PathTail> {
        crate::ast::support::children(&self.syntax)
    }
}
impl crate::ast::AstNode for Path {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PATH`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PATH`]: crate::SyntaxKind::PATH
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PATH
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`PATH`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`PATH`]: crate::SyntaxKind::PATH
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct PathTail {
    syntax: crate::SyntaxNode,
}
impl PathTail {
    #[inline]
    pub fn double_colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::DoubleColon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn tail(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for PathTail {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PATH_TAIL`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PATH_TAIL`]: crate::SyntaxKind::PATH_TAIL
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PATH_TAIL
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`PATH_TAIL`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`PATH_TAIL`]: crate::SyntaxKind::PATH_TAIL
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct QualifiedRef {
    syntax: crate::SyntaxNode,
}
impl QualifiedRef {
    #[inline]
    pub fn path(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Path>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for QualifiedRef {
    /// Returns `true` if the given [`SyntaxKind`] is a [`QUALIFIED_REF`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`QUALIFIED_REF`]: crate::SyntaxKind::QUALIFIED_REF
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::QUALIFIED_REF
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`QUALIFIED_REF`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`QUALIFIED_REF`]: crate::SyntaxKind::QUALIFIED_REF
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct RangeExpr {
    syntax: crate::SyntaxNode,
}
impl RangeExpr {
    #[inline]
    pub fn start(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn range_op(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::RangeOp>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn end(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::nth_child::<_, 1usize>(&self.syntax)
    }
}
impl crate::ast::AstNode for RangeExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`RANGE_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`RANGE_EXPR`]: crate::SyntaxKind::RANGE_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RANGE_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`RANGE_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`RANGE_EXPR`]: crate::SyntaxKind::RANGE_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct RetExpr {
    syntax: crate::SyntaxNode,
}
impl RetExpr {
    #[inline]
    pub fn return_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::ReturnToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for RetExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`RET_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`RET_EXPR`]: crate::SyntaxKind::RET_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RET_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`RET_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`RET_EXPR`]: crate::SyntaxKind::RET_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct Root {
    syntax: crate::SyntaxNode,
}
impl Root {
    #[inline]
    pub fn items(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Item> {
        crate::ast::support::children(&self.syntax)
    }
}
impl crate::ast::AstNode for Root {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ROOT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ROOT`]: crate::SyntaxKind::ROOT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ROOT
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`ROOT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`ROOT`]: crate::SyntaxKind::ROOT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct String {
    syntax: crate::SyntaxNode,
}
impl String {
    #[inline]
    pub fn string_literal(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::StringLiteral>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for String {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRING`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRING`]: crate::SyntaxKind::STRING
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRING
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`STRING`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`STRING`]: crate::SyntaxKind::STRING
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct StructDef {
    syntax: crate::SyntaxNode,
}
impl StructDef {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::StructToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn fields(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::StructFields>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for StructDef {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRUCT_DEF`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRUCT_DEF`]: crate::SyntaxKind::STRUCT_DEF
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_DEF
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`STRUCT_DEF`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`STRUCT_DEF`]: crate::SyntaxKind::STRUCT_DEF
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct StructInitExpr {
    syntax: crate::SyntaxNode,
}
impl StructInitExpr {
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Path>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn fields(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::StructInitField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for StructInitExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRUCT_INIT_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRUCT_INIT_EXPR`]: crate::SyntaxKind::STRUCT_INIT_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_INIT_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`STRUCT_INIT_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`STRUCT_INIT_EXPR`]: crate::SyntaxKind::STRUCT_INIT_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct StructInitField {
    syntax: crate::SyntaxNode,
}
impl StructInitField {
    #[inline]
    pub fn field(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn value(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for StructInitField {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRUCT_INIT_FIELD`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRUCT_INIT_FIELD`]: crate::SyntaxKind::STRUCT_INIT_FIELD
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_INIT_FIELD
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`STRUCT_INIT_FIELD`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`STRUCT_INIT_FIELD`]: crate::SyntaxKind::STRUCT_INIT_FIELD
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct StructPattern {
    syntax: crate::SyntaxNode,
}
impl StructPattern {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn fields(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::StructPatternField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for StructPattern {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRUCT_PATTERN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRUCT_PATTERN`]: crate::SyntaxKind::STRUCT_PATTERN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_PATTERN
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`STRUCT_PATTERN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`STRUCT_PATTERN`]: crate::SyntaxKind::STRUCT_PATTERN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct StructPatternField {
    syntax: crate::SyntaxNode,
}
impl StructPatternField {
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn alias(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for StructPatternField {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRUCT_PATTERN_FIELD`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRUCT_PATTERN_FIELD`]: crate::SyntaxKind::STRUCT_PATTERN_FIELD
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_PATTERN_FIELD
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`STRUCT_PATTERN_FIELD`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`STRUCT_PATTERN_FIELD`]: crate::SyntaxKind::STRUCT_PATTERN_FIELD
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct TupleExprElem {
    syntax: crate::SyntaxNode,
}
impl TupleExprElem {
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleExprElem {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TUPLE_EXPR_ELEM`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TUPLE_EXPR_ELEM`]: crate::SyntaxKind::TUPLE_EXPR_ELEM
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_EXPR_ELEM
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`TUPLE_EXPR_ELEM`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`TUPLE_EXPR_ELEM`]: crate::SyntaxKind::TUPLE_EXPR_ELEM
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct TupleInitExpr {
    syntax: crate::SyntaxNode,
}
impl TupleInitExpr {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn elems(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::TupleExprElem> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleInitExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TUPLE_INIT_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TUPLE_INIT_EXPR`]: crate::SyntaxKind::TUPLE_INIT_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_INIT_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`TUPLE_INIT_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`TUPLE_INIT_EXPR`]: crate::SyntaxKind::TUPLE_INIT_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct TuplePattern {
    syntax: crate::SyntaxNode,
}
impl TuplePattern {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn elements(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::TuplePatternElem> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for TuplePattern {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TUPLE_PATTERN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TUPLE_PATTERN`]: crate::SyntaxKind::TUPLE_PATTERN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_PATTERN
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`TUPLE_PATTERN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`TUPLE_PATTERN`]: crate::SyntaxKind::TUPLE_PATTERN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct TuplePatternElem {
    syntax: crate::SyntaxNode,
}
impl TuplePatternElem {
    #[inline]
    pub fn pattern(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for TuplePatternElem {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TUPLE_PATTERN_ELEM`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TUPLE_PATTERN_ELEM`]: crate::SyntaxKind::TUPLE_PATTERN_ELEM
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_PATTERN_ELEM
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`TUPLE_PATTERN_ELEM`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`TUPLE_PATTERN_ELEM`]: crate::SyntaxKind::TUPLE_PATTERN_ELEM
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct TupleStructField {
    syntax: crate::SyntaxNode,
}
impl TupleStructField {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleStructField {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TUPLE_STRUCT_FIELD`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TUPLE_STRUCT_FIELD`]: crate::SyntaxKind::TUPLE_STRUCT_FIELD
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_STRUCT_FIELD
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`TUPLE_STRUCT_FIELD`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`TUPLE_STRUCT_FIELD`]: crate::SyntaxKind::TUPLE_STRUCT_FIELD
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct TupleStructFields {
    syntax: crate::SyntaxNode,
}
impl TupleStructFields {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn tuple_struct_fields(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::TupleStructField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn semicolons(
        &self,
    ) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Semicolon> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleStructFields {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TUPLE_STRUCT_FIELDS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TUPLE_STRUCT_FIELDS`]: crate::SyntaxKind::TUPLE_STRUCT_FIELDS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_STRUCT_FIELDS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`TUPLE_STRUCT_FIELDS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`TUPLE_STRUCT_FIELDS`]: crate::SyntaxKind::TUPLE_STRUCT_FIELDS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct TupleType {
    syntax: crate::SyntaxNode,
}
impl TupleType {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn elements(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::TupleTypeElem> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleType {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TUPLE_TYPE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TUPLE_TYPE`]: crate::SyntaxKind::TUPLE_TYPE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_TYPE
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`TUPLE_TYPE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`TUPLE_TYPE`]: crate::SyntaxKind::TUPLE_TYPE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct TupleTypeElem {
    syntax: crate::SyntaxNode,
}
impl TupleTypeElem {
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleTypeElem {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TUPLE_TYPE_ELEM`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TUPLE_TYPE_ELEM`]: crate::SyntaxKind::TUPLE_TYPE_ELEM
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_TYPE_ELEM
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`TUPLE_TYPE_ELEM`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`TUPLE_TYPE_ELEM`]: crate::SyntaxKind::TUPLE_TYPE_ELEM
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct TypeAlias {
    syntax: crate::SyntaxNode,
}
impl TypeAlias {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::TypeToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn alias(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn original(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::nth_child::<_, 1usize>(&self.syntax)
    }
    #[inline]
    pub fn semicolons(
        &self,
    ) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Semicolon> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for TypeAlias {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TYPE_ALIAS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TYPE_ALIAS`]: crate::SyntaxKind::TYPE_ALIAS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TYPE_ALIAS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`TYPE_ALIAS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`TYPE_ALIAS`]: crate::SyntaxKind::TYPE_ALIAS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct UnaryExpr {
    syntax: crate::SyntaxNode,
}
impl UnaryExpr {
    #[inline]
    pub fn op(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::UnaryOp>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for UnaryExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`UNARY_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`UNARY_EXPR`]: crate::SyntaxKind::UNARY_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::UNARY_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`UNARY_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`UNARY_EXPR`]: crate::SyntaxKind::UNARY_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct UseAlias {
    syntax: crate::SyntaxNode,
}
impl UseAlias {
    #[inline]
    pub fn as_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::AsToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn alias(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for UseAlias {
    /// Returns `true` if the given [`SyntaxKind`] is a [`USE_ALIAS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`USE_ALIAS`]: crate::SyntaxKind::USE_ALIAS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_ALIAS
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`USE_ALIAS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`USE_ALIAS`]: crate::SyntaxKind::USE_ALIAS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct UseBranch {
    syntax: crate::SyntaxNode,
}
impl UseBranch {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn paths(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Path> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn use_branch_or_alias(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::UseBranchOrAlias> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for UseBranch {
    /// Returns `true` if the given [`SyntaxKind`] is a [`USE_BRANCH`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`USE_BRANCH`]: crate::SyntaxKind::USE_BRANCH
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_BRANCH
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`USE_BRANCH`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`USE_BRANCH`]: crate::SyntaxKind::USE_BRANCH
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct UseDef {
    syntax: crate::SyntaxNode,
}
impl UseDef {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::UseToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Path>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn use_branch_or_alias(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::UseBranchOrAlias>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Semicolon>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for UseDef {
    /// Returns `true` if the given [`SyntaxKind`] is a [`USE_DEF`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`USE_DEF`]: crate::SyntaxKind::USE_DEF
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_DEF
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`USE_DEF`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`USE_DEF`]: crate::SyntaxKind::USE_DEF
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct VarDecl {
    syntax: crate::SyntaxNode,
}
impl VarDecl {
    #[inline]
    pub fn let_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LetToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn value(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolons(
        &self,
    ) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Semicolon> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for VarDecl {
    /// Returns `true` if the given [`SyntaxKind`] is a [`VAR_DECL`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`VAR_DECL`]: crate::SyntaxKind::VAR_DECL
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VAR_DECL
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`VAR_DECL`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`VAR_DECL`]: crate::SyntaxKind::VAR_DECL
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct VarRef {
    syntax: crate::SyntaxNode,
}
impl VarRef {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for VarRef {
    /// Returns `true` if the given [`SyntaxKind`] is a [`VAR_REF`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`VAR_REF`]: crate::SyntaxKind::VAR_REF
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VAR_REF
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`VAR_REF`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`VAR_REF`]: crate::SyntaxKind::VAR_REF
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct VariantStruct {
    syntax: crate::SyntaxNode,
}
impl VariantStruct {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn fields(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::VariantStructField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for VariantStruct {
    /// Returns `true` if the given [`SyntaxKind`] is a [`VARIANT_STRUCT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`VARIANT_STRUCT`]: crate::SyntaxKind::VARIANT_STRUCT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VARIANT_STRUCT
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`VARIANT_STRUCT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`VARIANT_STRUCT`]: crate::SyntaxKind::VARIANT_STRUCT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct VariantStructField {
    syntax: crate::SyntaxNode,
}
impl VariantStructField {
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for VariantStructField {
    /// Returns `true` if the given [`SyntaxKind`] is a [`VARIANT_STRUCT_FIELD`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`VARIANT_STRUCT_FIELD`]: crate::SyntaxKind::VARIANT_STRUCT_FIELD
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VARIANT_STRUCT_FIELD
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`VARIANT_STRUCT_FIELD`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`VARIANT_STRUCT_FIELD`]: crate::SyntaxKind::VARIANT_STRUCT_FIELD
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct VariantTuple {
    syntax: crate::SyntaxNode,
}
impl VariantTuple {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn variant_tuple_elems(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::VariantTupleElem> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for VariantTuple {
    /// Returns `true` if the given [`SyntaxKind`] is a [`VARIANT_TUPLE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`VARIANT_TUPLE`]: crate::SyntaxKind::VARIANT_TUPLE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VARIANT_TUPLE
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`VARIANT_TUPLE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`VARIANT_TUPLE`]: crate::SyntaxKind::VARIANT_TUPLE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct VariantTupleElem {
    syntax: crate::SyntaxNode,
}
impl VariantTupleElem {
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for VariantTupleElem {
    /// Returns `true` if the given [`SyntaxKind`] is a [`VARIANT_TUPLE_ELEM`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`VARIANT_TUPLE_ELEM`]: crate::SyntaxKind::VARIANT_TUPLE_ELEM
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VARIANT_TUPLE_ELEM
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`VARIANT_TUPLE_ELEM`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`VARIANT_TUPLE_ELEM`]: crate::SyntaxKind::VARIANT_TUPLE_ELEM
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct WhileExpr {
    syntax: crate::SyntaxNode,
}
impl WhileExpr {
    #[inline]
    pub fn while_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::WhileToken>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn cond(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for WhileExpr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`WHILE_EXPR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`WHILE_EXPR`]: crate::SyntaxKind::WHILE_EXPR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::WHILE_EXPR
    }
    /// Returns [`Some`] if the given [`SyntaxNode`] has the [`WHILE_EXPR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxNode`]: crate::SyntaxNode
    /// [`WHILE_EXPR`]: crate::SyntaxKind::WHILE_EXPR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct AssignOp {
    syntax: crate::SyntaxNode,
}
impl crate::ast::AstNode for AssignOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ASSIGN_OP
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct BinOp {
    syntax: crate::SyntaxNode,
}
impl crate::ast::AstNode for BinOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BIN_OP
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub struct Bool {
    syntax: crate::SyntaxNode,
}
impl crate::ast::AstNode for Bool {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BOOL
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub enum EnumVariantBody {
    VariantStruct(crate::ast::nodes::VariantStruct),
    VariantTuple(crate::ast::nodes::VariantTuple),
}
impl EnumVariantBody {
    pub fn is_variant_struct(&self) -> bool {
        ::core::matches!(self, Self::VariantStruct(..))
    }
    pub fn is_variant_tuple(&self) -> bool {
        ::core::matches!(self, Self::VariantTuple(..))
    }
    pub fn as_variant_struct(&self) -> ::core::option::Option<&crate::ast::nodes::VariantStruct> {
        if let Self::VariantStruct(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_variant_tuple(&self) -> ::core::option::Option<&crate::ast::nodes::VariantTuple> {
        if let Self::VariantTuple(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_variant_struct(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::VariantStruct, Self> {
        if let Self::VariantStruct(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_variant_tuple(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::VariantTuple, Self> {
        if let Self::VariantTuple(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
}
impl crate::ast::AstNode for EnumVariantBody {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::nodes::VariantStruct as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::VariantTuple as crate::ast::AstNode>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            kind if <crate::ast::nodes::VariantStruct as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::VariantStruct as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VariantStruct(node)))
            }
            kind if <crate::ast::nodes::VariantTuple as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::VariantTuple as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VariantTuple(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::VariantStruct(syntax) => {
                <crate::ast::nodes::VariantStruct as crate::ast::AstNode>::syntax(syntax)
            }
            Self::VariantTuple(syntax) => {
                <crate::ast::nodes::VariantTuple as crate::ast::AstNode>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<EnumVariantBody> for crate::ast::nodes::VariantStruct {
    type Error = EnumVariantBody;
    #[inline]
    fn try_from(value: EnumVariantBody) -> ::core::result::Result<Self, Self::Error> {
        if let EnumVariantBody::VariantStruct(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<EnumVariantBody> for crate::ast::nodes::VariantTuple {
    type Error = EnumVariantBody;
    #[inline]
    fn try_from(value: EnumVariantBody) -> ::core::result::Result<Self, Self::Error> {
        if let EnumVariantBody::VariantTuple(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::nodes::VariantStruct> for EnumVariantBody {
    #[inline]
    fn from(value: crate::ast::nodes::VariantStruct) -> Self {
        Self::VariantStruct(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::VariantTuple> for EnumVariantBody {
    #[inline]
    fn from(value: crate::ast::nodes::VariantTuple) -> Self {
        Self::VariantTuple(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Expr {
    ArrayAccess(crate::ast::nodes::ArrayAccess),
    ArrayInitExpr(crate::ast::nodes::ArrayInitExpr),
    Assign(crate::ast::nodes::Assign),
    BinExpr(crate::ast::nodes::BinExpr),
    Block(crate::ast::nodes::Block),
    BreakExpr(crate::ast::nodes::BreakExpr),
    ClosureExpr(crate::ast::nodes::ClosureExpr),
    ContinueExpr(crate::ast::nodes::ContinueExpr),
    FieldAccess(crate::ast::nodes::FieldAccess),
    ForExpr(crate::ast::nodes::ForExpr),
    FunctionCall(crate::ast::nodes::FunctionCall),
    IfExpr(crate::ast::nodes::IfExpr),
    Literal(crate::ast::nodes::Literal),
    LoopExpr(crate::ast::nodes::LoopExpr),
    MatchExpr(crate::ast::nodes::MatchExpr),
    ParenExpr(crate::ast::nodes::ParenExpr),
    QualifiedRef(crate::ast::nodes::QualifiedRef),
    RangeExpr(crate::ast::nodes::RangeExpr),
    RetExpr(crate::ast::nodes::RetExpr),
    StructInitExpr(crate::ast::nodes::StructInitExpr),
    TupleInitExpr(crate::ast::nodes::TupleInitExpr),
    UnaryExpr(crate::ast::nodes::UnaryExpr),
    VarRef(crate::ast::nodes::VarRef),
    WhileExpr(crate::ast::nodes::WhileExpr),
}
impl Expr {
    pub fn is_array_access(&self) -> bool {
        ::core::matches!(self, Self::ArrayAccess(..))
    }
    pub fn is_array_init_expr(&self) -> bool {
        ::core::matches!(self, Self::ArrayInitExpr(..))
    }
    pub fn is_assign(&self) -> bool {
        ::core::matches!(self, Self::Assign(..))
    }
    pub fn is_bin_expr(&self) -> bool {
        ::core::matches!(self, Self::BinExpr(..))
    }
    pub fn is_block(&self) -> bool {
        ::core::matches!(self, Self::Block(..))
    }
    pub fn is_break_expr(&self) -> bool {
        ::core::matches!(self, Self::BreakExpr(..))
    }
    pub fn is_closure_expr(&self) -> bool {
        ::core::matches!(self, Self::ClosureExpr(..))
    }
    pub fn is_continue_expr(&self) -> bool {
        ::core::matches!(self, Self::ContinueExpr(..))
    }
    pub fn is_field_access(&self) -> bool {
        ::core::matches!(self, Self::FieldAccess(..))
    }
    pub fn is_for_expr(&self) -> bool {
        ::core::matches!(self, Self::ForExpr(..))
    }
    pub fn is_function_call(&self) -> bool {
        ::core::matches!(self, Self::FunctionCall(..))
    }
    pub fn is_if_expr(&self) -> bool {
        ::core::matches!(self, Self::IfExpr(..))
    }
    pub fn is_literal(&self) -> bool {
        ::core::matches!(self, Self::Literal(..))
    }
    pub fn is_loop_expr(&self) -> bool {
        ::core::matches!(self, Self::LoopExpr(..))
    }
    pub fn is_match_expr(&self) -> bool {
        ::core::matches!(self, Self::MatchExpr(..))
    }
    pub fn is_paren_expr(&self) -> bool {
        ::core::matches!(self, Self::ParenExpr(..))
    }
    pub fn is_qualified_ref(&self) -> bool {
        ::core::matches!(self, Self::QualifiedRef(..))
    }
    pub fn is_range_expr(&self) -> bool {
        ::core::matches!(self, Self::RangeExpr(..))
    }
    pub fn is_ret_expr(&self) -> bool {
        ::core::matches!(self, Self::RetExpr(..))
    }
    pub fn is_struct_init_expr(&self) -> bool {
        ::core::matches!(self, Self::StructInitExpr(..))
    }
    pub fn is_tuple_init_expr(&self) -> bool {
        ::core::matches!(self, Self::TupleInitExpr(..))
    }
    pub fn is_unary_expr(&self) -> bool {
        ::core::matches!(self, Self::UnaryExpr(..))
    }
    pub fn is_var_ref(&self) -> bool {
        ::core::matches!(self, Self::VarRef(..))
    }
    pub fn is_while_expr(&self) -> bool {
        ::core::matches!(self, Self::WhileExpr(..))
    }
    pub fn as_array_access(&self) -> ::core::option::Option<&crate::ast::nodes::ArrayAccess> {
        if let Self::ArrayAccess(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_array_init_expr(&self) -> ::core::option::Option<&crate::ast::nodes::ArrayInitExpr> {
        if let Self::ArrayInitExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_assign(&self) -> ::core::option::Option<&crate::ast::nodes::Assign> {
        if let Self::Assign(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_bin_expr(&self) -> ::core::option::Option<&crate::ast::nodes::BinExpr> {
        if let Self::BinExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_block(&self) -> ::core::option::Option<&crate::ast::nodes::Block> {
        if let Self::Block(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_break_expr(&self) -> ::core::option::Option<&crate::ast::nodes::BreakExpr> {
        if let Self::BreakExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_closure_expr(&self) -> ::core::option::Option<&crate::ast::nodes::ClosureExpr> {
        if let Self::ClosureExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_continue_expr(&self) -> ::core::option::Option<&crate::ast::nodes::ContinueExpr> {
        if let Self::ContinueExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_field_access(&self) -> ::core::option::Option<&crate::ast::nodes::FieldAccess> {
        if let Self::FieldAccess(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_for_expr(&self) -> ::core::option::Option<&crate::ast::nodes::ForExpr> {
        if let Self::ForExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_function_call(&self) -> ::core::option::Option<&crate::ast::nodes::FunctionCall> {
        if let Self::FunctionCall(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_if_expr(&self) -> ::core::option::Option<&crate::ast::nodes::IfExpr> {
        if let Self::IfExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_literal(&self) -> ::core::option::Option<&crate::ast::nodes::Literal> {
        if let Self::Literal(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_loop_expr(&self) -> ::core::option::Option<&crate::ast::nodes::LoopExpr> {
        if let Self::LoopExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_match_expr(&self) -> ::core::option::Option<&crate::ast::nodes::MatchExpr> {
        if let Self::MatchExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_paren_expr(&self) -> ::core::option::Option<&crate::ast::nodes::ParenExpr> {
        if let Self::ParenExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_qualified_ref(&self) -> ::core::option::Option<&crate::ast::nodes::QualifiedRef> {
        if let Self::QualifiedRef(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_range_expr(&self) -> ::core::option::Option<&crate::ast::nodes::RangeExpr> {
        if let Self::RangeExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_ret_expr(&self) -> ::core::option::Option<&crate::ast::nodes::RetExpr> {
        if let Self::RetExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_struct_init_expr(
        &self,
    ) -> ::core::option::Option<&crate::ast::nodes::StructInitExpr> {
        if let Self::StructInitExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_tuple_init_expr(&self) -> ::core::option::Option<&crate::ast::nodes::TupleInitExpr> {
        if let Self::TupleInitExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_unary_expr(&self) -> ::core::option::Option<&crate::ast::nodes::UnaryExpr> {
        if let Self::UnaryExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_var_ref(&self) -> ::core::option::Option<&crate::ast::nodes::VarRef> {
        if let Self::VarRef(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_while_expr(&self) -> ::core::option::Option<&crate::ast::nodes::WhileExpr> {
        if let Self::WhileExpr(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_array_access(self) -> ::core::result::Result<crate::ast::nodes::ArrayAccess, Self> {
        if let Self::ArrayAccess(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_array_init_expr(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::ArrayInitExpr, Self> {
        if let Self::ArrayInitExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_assign(self) -> ::core::result::Result<crate::ast::nodes::Assign, Self> {
        if let Self::Assign(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_bin_expr(self) -> ::core::result::Result<crate::ast::nodes::BinExpr, Self> {
        if let Self::BinExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_block(self) -> ::core::result::Result<crate::ast::nodes::Block, Self> {
        if let Self::Block(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_break_expr(self) -> ::core::result::Result<crate::ast::nodes::BreakExpr, Self> {
        if let Self::BreakExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_closure_expr(self) -> ::core::result::Result<crate::ast::nodes::ClosureExpr, Self> {
        if let Self::ClosureExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_continue_expr(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::ContinueExpr, Self> {
        if let Self::ContinueExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_field_access(self) -> ::core::result::Result<crate::ast::nodes::FieldAccess, Self> {
        if let Self::FieldAccess(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_for_expr(self) -> ::core::result::Result<crate::ast::nodes::ForExpr, Self> {
        if let Self::ForExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_function_call(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::FunctionCall, Self> {
        if let Self::FunctionCall(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_if_expr(self) -> ::core::result::Result<crate::ast::nodes::IfExpr, Self> {
        if let Self::IfExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_literal(self) -> ::core::result::Result<crate::ast::nodes::Literal, Self> {
        if let Self::Literal(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_loop_expr(self) -> ::core::result::Result<crate::ast::nodes::LoopExpr, Self> {
        if let Self::LoopExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_match_expr(self) -> ::core::result::Result<crate::ast::nodes::MatchExpr, Self> {
        if let Self::MatchExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_paren_expr(self) -> ::core::result::Result<crate::ast::nodes::ParenExpr, Self> {
        if let Self::ParenExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_qualified_ref(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::QualifiedRef, Self> {
        if let Self::QualifiedRef(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_range_expr(self) -> ::core::result::Result<crate::ast::nodes::RangeExpr, Self> {
        if let Self::RangeExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_ret_expr(self) -> ::core::result::Result<crate::ast::nodes::RetExpr, Self> {
        if let Self::RetExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_struct_init_expr(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::StructInitExpr, Self> {
        if let Self::StructInitExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_tuple_init_expr(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::TupleInitExpr, Self> {
        if let Self::TupleInitExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_unary_expr(self) -> ::core::result::Result<crate::ast::nodes::UnaryExpr, Self> {
        if let Self::UnaryExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_var_ref(self) -> ::core::result::Result<crate::ast::nodes::VarRef, Self> {
        if let Self::VarRef(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_while_expr(self) -> ::core::result::Result<crate::ast::nodes::WhileExpr, Self> {
        if let Self::WhileExpr(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
}
impl crate::ast::AstNode for Expr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::nodes::ArrayAccess as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::ArrayInitExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::Assign as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::BinExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::Block as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::BreakExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::ClosureExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::ContinueExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::FieldAccess as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::ForExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::FunctionCall as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::IfExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::Literal as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::LoopExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::MatchExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::ParenExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::QualifiedRef as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::RangeExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::RetExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::StructInitExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::TupleInitExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::UnaryExpr as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::VarRef as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::WhileExpr as crate::ast::AstNode>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            kind if <crate::ast::nodes::ArrayAccess as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::ArrayAccess as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ArrayAccess(node)))
            }
            kind if <crate::ast::nodes::ArrayInitExpr as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::ArrayInitExpr as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ArrayInitExpr(node)))
            }
            kind if <crate::ast::nodes::Assign as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::Assign as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Assign(node)))
            }
            kind if <crate::ast::nodes::BinExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::BinExpr as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::BinExpr(node)))
            }
            kind if <crate::ast::nodes::Block as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::Block as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Block(node)))
            }
            kind if <crate::ast::nodes::BreakExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::BreakExpr as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::BreakExpr(node)))
            }
            kind if <crate::ast::nodes::ClosureExpr as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::ClosureExpr as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ClosureExpr(node)))
            }
            kind if <crate::ast::nodes::ContinueExpr as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::ContinueExpr as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ContinueExpr(node)))
            }
            kind if <crate::ast::nodes::FieldAccess as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::FieldAccess as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FieldAccess(node)))
            }
            kind if <crate::ast::nodes::ForExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::ForExpr as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ForExpr(node)))
            }
            kind if <crate::ast::nodes::FunctionCall as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::FunctionCall as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FunctionCall(node)))
            }
            kind if <crate::ast::nodes::IfExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::IfExpr as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::IfExpr(node)))
            }
            kind if <crate::ast::nodes::Literal as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::Literal as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Literal(node)))
            }
            kind if <crate::ast::nodes::LoopExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::LoopExpr as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::LoopExpr(node)))
            }
            kind if <crate::ast::nodes::MatchExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::MatchExpr as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::MatchExpr(node)))
            }
            kind if <crate::ast::nodes::ParenExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::ParenExpr as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ParenExpr(node)))
            }
            kind if <crate::ast::nodes::QualifiedRef as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::QualifiedRef as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::QualifiedRef(node)))
            }
            kind if <crate::ast::nodes::RangeExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::RangeExpr as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RangeExpr(node)))
            }
            kind if <crate::ast::nodes::RetExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::RetExpr as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RetExpr(node)))
            }
            kind if <crate::ast::nodes::StructInitExpr as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node = match <crate::ast::nodes::StructInitExpr as crate::ast::AstNode>::cast(
                    syntax,
                ) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::StructInitExpr(node)))
            }
            kind if <crate::ast::nodes::TupleInitExpr as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::TupleInitExpr as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TupleInitExpr(node)))
            }
            kind if <crate::ast::nodes::UnaryExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::UnaryExpr as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::UnaryExpr(node)))
            }
            kind if <crate::ast::nodes::VarRef as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::VarRef as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VarRef(node)))
            }
            kind if <crate::ast::nodes::WhileExpr as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::WhileExpr as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::WhileExpr(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::ArrayAccess(syntax) => {
                <crate::ast::nodes::ArrayAccess as crate::ast::AstNode>::syntax(syntax)
            }
            Self::ArrayInitExpr(syntax) => {
                <crate::ast::nodes::ArrayInitExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::Assign(syntax) => {
                <crate::ast::nodes::Assign as crate::ast::AstNode>::syntax(syntax)
            }
            Self::BinExpr(syntax) => {
                <crate::ast::nodes::BinExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::Block(syntax) => {
                <crate::ast::nodes::Block as crate::ast::AstNode>::syntax(syntax)
            }
            Self::BreakExpr(syntax) => {
                <crate::ast::nodes::BreakExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::ClosureExpr(syntax) => {
                <crate::ast::nodes::ClosureExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::ContinueExpr(syntax) => {
                <crate::ast::nodes::ContinueExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::FieldAccess(syntax) => {
                <crate::ast::nodes::FieldAccess as crate::ast::AstNode>::syntax(syntax)
            }
            Self::ForExpr(syntax) => {
                <crate::ast::nodes::ForExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::FunctionCall(syntax) => {
                <crate::ast::nodes::FunctionCall as crate::ast::AstNode>::syntax(syntax)
            }
            Self::IfExpr(syntax) => {
                <crate::ast::nodes::IfExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::Literal(syntax) => {
                <crate::ast::nodes::Literal as crate::ast::AstNode>::syntax(syntax)
            }
            Self::LoopExpr(syntax) => {
                <crate::ast::nodes::LoopExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::MatchExpr(syntax) => {
                <crate::ast::nodes::MatchExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::ParenExpr(syntax) => {
                <crate::ast::nodes::ParenExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::QualifiedRef(syntax) => {
                <crate::ast::nodes::QualifiedRef as crate::ast::AstNode>::syntax(syntax)
            }
            Self::RangeExpr(syntax) => {
                <crate::ast::nodes::RangeExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::RetExpr(syntax) => {
                <crate::ast::nodes::RetExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::StructInitExpr(syntax) => {
                <crate::ast::nodes::StructInitExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::TupleInitExpr(syntax) => {
                <crate::ast::nodes::TupleInitExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::UnaryExpr(syntax) => {
                <crate::ast::nodes::UnaryExpr as crate::ast::AstNode>::syntax(syntax)
            }
            Self::VarRef(syntax) => {
                <crate::ast::nodes::VarRef as crate::ast::AstNode>::syntax(syntax)
            }
            Self::WhileExpr(syntax) => {
                <crate::ast::nodes::WhileExpr as crate::ast::AstNode>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::ArrayAccess {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::ArrayAccess(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::ArrayInitExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::ArrayInitExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::Assign {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::Assign(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::BinExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::BinExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::Block {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::Block(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::BreakExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::BreakExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::ClosureExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::ClosureExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::ContinueExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::ContinueExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::FieldAccess {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::FieldAccess(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::ForExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::ForExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::FunctionCall {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::FunctionCall(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::IfExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::IfExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::Literal {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::Literal(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::LoopExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::LoopExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::MatchExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::MatchExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::ParenExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::ParenExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::QualifiedRef {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::QualifiedRef(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::RangeExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::RangeExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::RetExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::RetExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::StructInitExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::StructInitExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::TupleInitExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::TupleInitExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::UnaryExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::UnaryExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::VarRef {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::VarRef(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Expr> for crate::ast::nodes::WhileExpr {
    type Error = Expr;
    #[inline]
    fn try_from(value: Expr) -> ::core::result::Result<Self, Self::Error> {
        if let Expr::WhileExpr(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::nodes::ArrayAccess> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::ArrayAccess) -> Self {
        Self::ArrayAccess(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::ArrayInitExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::ArrayInitExpr) -> Self {
        Self::ArrayInitExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::Assign> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::Assign) -> Self {
        Self::Assign(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::BinExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::BinExpr) -> Self {
        Self::BinExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::Block> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::Block) -> Self {
        Self::Block(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::BreakExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::BreakExpr) -> Self {
        Self::BreakExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::ClosureExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::ClosureExpr) -> Self {
        Self::ClosureExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::ContinueExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::ContinueExpr) -> Self {
        Self::ContinueExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::FieldAccess> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::FieldAccess) -> Self {
        Self::FieldAccess(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::ForExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::ForExpr) -> Self {
        Self::ForExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::FunctionCall> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::FunctionCall) -> Self {
        Self::FunctionCall(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::IfExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::IfExpr) -> Self {
        Self::IfExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::Literal> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::Literal) -> Self {
        Self::Literal(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::LoopExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::LoopExpr) -> Self {
        Self::LoopExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::MatchExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::MatchExpr) -> Self {
        Self::MatchExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::ParenExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::ParenExpr) -> Self {
        Self::ParenExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::QualifiedRef> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::QualifiedRef) -> Self {
        Self::QualifiedRef(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::RangeExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::RangeExpr) -> Self {
        Self::RangeExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::RetExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::RetExpr) -> Self {
        Self::RetExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::StructInitExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::StructInitExpr) -> Self {
        Self::StructInitExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::TupleInitExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::TupleInitExpr) -> Self {
        Self::TupleInitExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::UnaryExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::UnaryExpr) -> Self {
        Self::UnaryExpr(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::VarRef> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::VarRef) -> Self {
        Self::VarRef(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::WhileExpr> for Expr {
    #[inline]
    fn from(value: crate::ast::nodes::WhileExpr) -> Self {
        Self::WhileExpr(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum FieldAccessor {
    FieldAccessorName(crate::ast::nodes::FieldAccessorName),
    Number(crate::ast::nodes::Number),
}
impl FieldAccessor {
    pub fn is_field_accessor_name(&self) -> bool {
        ::core::matches!(self, Self::FieldAccessorName(..))
    }
    pub fn is_number(&self) -> bool {
        ::core::matches!(self, Self::Number(..))
    }
    pub fn as_field_accessor_name(
        &self,
    ) -> ::core::option::Option<&crate::ast::nodes::FieldAccessorName> {
        if let Self::FieldAccessorName(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_number(&self) -> ::core::option::Option<&crate::ast::nodes::Number> {
        if let Self::Number(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_field_accessor_name(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::FieldAccessorName, Self> {
        if let Self::FieldAccessorName(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_number(self) -> ::core::result::Result<crate::ast::nodes::Number, Self> {
        if let Self::Number(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
}
impl crate::ast::AstNode for FieldAccessor {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::nodes::FieldAccessorName as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::Number as crate::ast::AstNode>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            kind
                if <crate::ast::nodes::FieldAccessorName as crate::ast::AstNode>::can_cast_from(
                    kind,
                ) =>
            {
                let node = match <crate::ast::nodes::FieldAccessorName as crate::ast::AstNode>::cast(
                    syntax,
                ) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FieldAccessorName(
                    node,
                )))
            }
            kind if <crate::ast::nodes::Number as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::Number as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Number(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::FieldAccessorName(syntax) => {
                <crate::ast::nodes::FieldAccessorName as crate::ast::AstNode>::syntax(syntax)
            }
            Self::Number(syntax) => {
                <crate::ast::nodes::Number as crate::ast::AstNode>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<FieldAccessor> for crate::ast::nodes::FieldAccessorName {
    type Error = FieldAccessor;
    #[inline]
    fn try_from(value: FieldAccessor) -> ::core::result::Result<Self, Self::Error> {
        if let FieldAccessor::FieldAccessorName(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<FieldAccessor> for crate::ast::nodes::Number {
    type Error = FieldAccessor;
    #[inline]
    fn try_from(value: FieldAccessor) -> ::core::result::Result<Self, Self::Error> {
        if let FieldAccessor::Number(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::nodes::FieldAccessorName> for FieldAccessor {
    #[inline]
    fn from(value: crate::ast::nodes::FieldAccessorName) -> Self {
        Self::FieldAccessorName(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::Number> for FieldAccessor {
    #[inline]
    fn from(value: crate::ast::nodes::Number) -> Self {
        Self::Number(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Item {
    ConstDef(crate::ast::nodes::ConstDef),
    EnumDef(crate::ast::nodes::EnumDef),
    FunctionDef(crate::ast::nodes::FunctionDef),
    ImplBlock(crate::ast::nodes::ImplBlock),
    StructDef(crate::ast::nodes::StructDef),
    TypeAlias(crate::ast::nodes::TypeAlias),
    UseDef(crate::ast::nodes::UseDef),
}
impl Item {
    pub fn is_const_def(&self) -> bool {
        ::core::matches!(self, Self::ConstDef(..))
    }
    pub fn is_enum_def(&self) -> bool {
        ::core::matches!(self, Self::EnumDef(..))
    }
    pub fn is_function_def(&self) -> bool {
        ::core::matches!(self, Self::FunctionDef(..))
    }
    pub fn is_impl_block(&self) -> bool {
        ::core::matches!(self, Self::ImplBlock(..))
    }
    pub fn is_struct_def(&self) -> bool {
        ::core::matches!(self, Self::StructDef(..))
    }
    pub fn is_type_alias(&self) -> bool {
        ::core::matches!(self, Self::TypeAlias(..))
    }
    pub fn is_use_def(&self) -> bool {
        ::core::matches!(self, Self::UseDef(..))
    }
    pub fn as_const_def(&self) -> ::core::option::Option<&crate::ast::nodes::ConstDef> {
        if let Self::ConstDef(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_enum_def(&self) -> ::core::option::Option<&crate::ast::nodes::EnumDef> {
        if let Self::EnumDef(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_function_def(&self) -> ::core::option::Option<&crate::ast::nodes::FunctionDef> {
        if let Self::FunctionDef(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_impl_block(&self) -> ::core::option::Option<&crate::ast::nodes::ImplBlock> {
        if let Self::ImplBlock(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_struct_def(&self) -> ::core::option::Option<&crate::ast::nodes::StructDef> {
        if let Self::StructDef(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_type_alias(&self) -> ::core::option::Option<&crate::ast::nodes::TypeAlias> {
        if let Self::TypeAlias(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_use_def(&self) -> ::core::option::Option<&crate::ast::nodes::UseDef> {
        if let Self::UseDef(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_const_def(self) -> ::core::result::Result<crate::ast::nodes::ConstDef, Self> {
        if let Self::ConstDef(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_enum_def(self) -> ::core::result::Result<crate::ast::nodes::EnumDef, Self> {
        if let Self::EnumDef(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_function_def(self) -> ::core::result::Result<crate::ast::nodes::FunctionDef, Self> {
        if let Self::FunctionDef(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_impl_block(self) -> ::core::result::Result<crate::ast::nodes::ImplBlock, Self> {
        if let Self::ImplBlock(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_struct_def(self) -> ::core::result::Result<crate::ast::nodes::StructDef, Self> {
        if let Self::StructDef(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_type_alias(self) -> ::core::result::Result<crate::ast::nodes::TypeAlias, Self> {
        if let Self::TypeAlias(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_use_def(self) -> ::core::result::Result<crate::ast::nodes::UseDef, Self> {
        if let Self::UseDef(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
}
impl crate::ast::AstNode for Item {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::nodes::ConstDef as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::EnumDef as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::FunctionDef as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::ImplBlock as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::StructDef as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::TypeAlias as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::UseDef as crate::ast::AstNode>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            kind if <crate::ast::nodes::ConstDef as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::ConstDef as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ConstDef(node)))
            }
            kind if <crate::ast::nodes::EnumDef as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::EnumDef as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::EnumDef(node)))
            }
            kind if <crate::ast::nodes::FunctionDef as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::FunctionDef as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FunctionDef(node)))
            }
            kind if <crate::ast::nodes::ImplBlock as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::ImplBlock as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ImplBlock(node)))
            }
            kind if <crate::ast::nodes::StructDef as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::StructDef as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::StructDef(node)))
            }
            kind if <crate::ast::nodes::TypeAlias as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::TypeAlias as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TypeAlias(node)))
            }
            kind if <crate::ast::nodes::UseDef as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::UseDef as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::UseDef(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::ConstDef(syntax) => {
                <crate::ast::nodes::ConstDef as crate::ast::AstNode>::syntax(syntax)
            }
            Self::EnumDef(syntax) => {
                <crate::ast::nodes::EnumDef as crate::ast::AstNode>::syntax(syntax)
            }
            Self::FunctionDef(syntax) => {
                <crate::ast::nodes::FunctionDef as crate::ast::AstNode>::syntax(syntax)
            }
            Self::ImplBlock(syntax) => {
                <crate::ast::nodes::ImplBlock as crate::ast::AstNode>::syntax(syntax)
            }
            Self::StructDef(syntax) => {
                <crate::ast::nodes::StructDef as crate::ast::AstNode>::syntax(syntax)
            }
            Self::TypeAlias(syntax) => {
                <crate::ast::nodes::TypeAlias as crate::ast::AstNode>::syntax(syntax)
            }
            Self::UseDef(syntax) => {
                <crate::ast::nodes::UseDef as crate::ast::AstNode>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<Item> for crate::ast::nodes::ConstDef {
    type Error = Item;
    #[inline]
    fn try_from(value: Item) -> ::core::result::Result<Self, Self::Error> {
        if let Item::ConstDef(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Item> for crate::ast::nodes::EnumDef {
    type Error = Item;
    #[inline]
    fn try_from(value: Item) -> ::core::result::Result<Self, Self::Error> {
        if let Item::EnumDef(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Item> for crate::ast::nodes::FunctionDef {
    type Error = Item;
    #[inline]
    fn try_from(value: Item) -> ::core::result::Result<Self, Self::Error> {
        if let Item::FunctionDef(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Item> for crate::ast::nodes::ImplBlock {
    type Error = Item;
    #[inline]
    fn try_from(value: Item) -> ::core::result::Result<Self, Self::Error> {
        if let Item::ImplBlock(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Item> for crate::ast::nodes::StructDef {
    type Error = Item;
    #[inline]
    fn try_from(value: Item) -> ::core::result::Result<Self, Self::Error> {
        if let Item::StructDef(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Item> for crate::ast::nodes::TypeAlias {
    type Error = Item;
    #[inline]
    fn try_from(value: Item) -> ::core::result::Result<Self, Self::Error> {
        if let Item::TypeAlias(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Item> for crate::ast::nodes::UseDef {
    type Error = Item;
    #[inline]
    fn try_from(value: Item) -> ::core::result::Result<Self, Self::Error> {
        if let Item::UseDef(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::nodes::ConstDef> for Item {
    #[inline]
    fn from(value: crate::ast::nodes::ConstDef) -> Self {
        Self::ConstDef(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::EnumDef> for Item {
    #[inline]
    fn from(value: crate::ast::nodes::EnumDef) -> Self {
        Self::EnumDef(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::FunctionDef> for Item {
    #[inline]
    fn from(value: crate::ast::nodes::FunctionDef) -> Self {
        Self::FunctionDef(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::ImplBlock> for Item {
    #[inline]
    fn from(value: crate::ast::nodes::ImplBlock) -> Self {
        Self::ImplBlock(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::StructDef> for Item {
    #[inline]
    fn from(value: crate::ast::nodes::StructDef) -> Self {
        Self::StructDef(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::TypeAlias> for Item {
    #[inline]
    fn from(value: crate::ast::nodes::TypeAlias) -> Self {
        Self::TypeAlias(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::UseDef> for Item {
    #[inline]
    fn from(value: crate::ast::nodes::UseDef) -> Self {
        Self::UseDef(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Literal {
    Bool(crate::ast::nodes::Bool),
    Number(crate::ast::nodes::Number),
    String(crate::ast::nodes::String),
}
impl Literal {
    pub fn is_bool(&self) -> bool {
        ::core::matches!(self, Self::Bool(..))
    }
    pub fn is_number(&self) -> bool {
        ::core::matches!(self, Self::Number(..))
    }
    pub fn is_string(&self) -> bool {
        ::core::matches!(self, Self::String(..))
    }
    pub fn as_bool(&self) -> ::core::option::Option<&crate::ast::nodes::Bool> {
        if let Self::Bool(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_number(&self) -> ::core::option::Option<&crate::ast::nodes::Number> {
        if let Self::Number(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_string(&self) -> ::core::option::Option<&crate::ast::nodes::String> {
        if let Self::String(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_bool(self) -> ::core::result::Result<crate::ast::nodes::Bool, Self> {
        if let Self::Bool(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_number(self) -> ::core::result::Result<crate::ast::nodes::Number, Self> {
        if let Self::Number(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_string(self) -> ::core::result::Result<crate::ast::nodes::String, Self> {
        if let Self::String(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
}
impl crate::ast::AstNode for Literal {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::nodes::Bool as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::Number as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::String as crate::ast::AstNode>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            kind if <crate::ast::nodes::Bool as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::Bool as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Bool(node)))
            }
            kind if <crate::ast::nodes::Number as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::Number as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Number(node)))
            }
            kind if <crate::ast::nodes::String as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::String as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::String(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::Bool(syntax) => <crate::ast::nodes::Bool as crate::ast::AstNode>::syntax(syntax),
            Self::Number(syntax) => {
                <crate::ast::nodes::Number as crate::ast::AstNode>::syntax(syntax)
            }
            Self::String(syntax) => {
                <crate::ast::nodes::String as crate::ast::AstNode>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<Literal> for crate::ast::nodes::Bool {
    type Error = Literal;
    #[inline]
    fn try_from(value: Literal) -> ::core::result::Result<Self, Self::Error> {
        if let Literal::Bool(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Literal> for crate::ast::nodes::Number {
    type Error = Literal;
    #[inline]
    fn try_from(value: Literal) -> ::core::result::Result<Self, Self::Error> {
        if let Literal::Number(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Literal> for crate::ast::nodes::String {
    type Error = Literal;
    #[inline]
    fn try_from(value: Literal) -> ::core::result::Result<Self, Self::Error> {
        if let Literal::String(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::nodes::Bool> for Literal {
    #[inline]
    fn from(value: crate::ast::nodes::Bool) -> Self {
        Self::Bool(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::Number> for Literal {
    #[inline]
    fn from(value: crate::ast::nodes::Number) -> Self {
        Self::Number(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::String> for Literal {
    #[inline]
    fn from(value: crate::ast::nodes::String) -> Self {
        Self::String(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Pattern {
    StructPattern(crate::ast::nodes::StructPattern),
    TuplePattern(crate::ast::nodes::TuplePattern),
    VarRef(crate::ast::nodes::VarRef),
}
impl Pattern {
    pub fn is_struct_pattern(&self) -> bool {
        ::core::matches!(self, Self::StructPattern(..))
    }
    pub fn is_tuple_pattern(&self) -> bool {
        ::core::matches!(self, Self::TuplePattern(..))
    }
    pub fn is_var_ref(&self) -> bool {
        ::core::matches!(self, Self::VarRef(..))
    }
    pub fn as_struct_pattern(&self) -> ::core::option::Option<&crate::ast::nodes::StructPattern> {
        if let Self::StructPattern(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_tuple_pattern(&self) -> ::core::option::Option<&crate::ast::nodes::TuplePattern> {
        if let Self::TuplePattern(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_var_ref(&self) -> ::core::option::Option<&crate::ast::nodes::VarRef> {
        if let Self::VarRef(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_struct_pattern(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::StructPattern, Self> {
        if let Self::StructPattern(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_tuple_pattern(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::TuplePattern, Self> {
        if let Self::TuplePattern(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_var_ref(self) -> ::core::result::Result<crate::ast::nodes::VarRef, Self> {
        if let Self::VarRef(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
}
impl crate::ast::AstNode for Pattern {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::nodes::StructPattern as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::TuplePattern as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::VarRef as crate::ast::AstNode>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            kind if <crate::ast::nodes::StructPattern as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::StructPattern as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::StructPattern(node)))
            }
            kind if <crate::ast::nodes::TuplePattern as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::TuplePattern as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TuplePattern(node)))
            }
            kind if <crate::ast::nodes::VarRef as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::VarRef as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VarRef(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::StructPattern(syntax) => {
                <crate::ast::nodes::StructPattern as crate::ast::AstNode>::syntax(syntax)
            }
            Self::TuplePattern(syntax) => {
                <crate::ast::nodes::TuplePattern as crate::ast::AstNode>::syntax(syntax)
            }
            Self::VarRef(syntax) => {
                <crate::ast::nodes::VarRef as crate::ast::AstNode>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<Pattern> for crate::ast::nodes::StructPattern {
    type Error = Pattern;
    #[inline]
    fn try_from(value: Pattern) -> ::core::result::Result<Self, Self::Error> {
        if let Pattern::StructPattern(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Pattern> for crate::ast::nodes::TuplePattern {
    type Error = Pattern;
    #[inline]
    fn try_from(value: Pattern) -> ::core::result::Result<Self, Self::Error> {
        if let Pattern::TuplePattern(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Pattern> for crate::ast::nodes::VarRef {
    type Error = Pattern;
    #[inline]
    fn try_from(value: Pattern) -> ::core::result::Result<Self, Self::Error> {
        if let Pattern::VarRef(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::nodes::StructPattern> for Pattern {
    #[inline]
    fn from(value: crate::ast::nodes::StructPattern) -> Self {
        Self::StructPattern(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::TuplePattern> for Pattern {
    #[inline]
    fn from(value: crate::ast::nodes::TuplePattern) -> Self {
        Self::TuplePattern(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::VarRef> for Pattern {
    #[inline]
    fn from(value: crate::ast::nodes::VarRef) -> Self {
        Self::VarRef(value)
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
pub struct RangeOp {
    syntax: crate::SyntaxNode,
}
impl crate::ast::AstNode for RangeOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RANGE_OP
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub enum Stmt {
    ExprStmt(crate::ast::nodes::ExprStmt),
    VarDecl(crate::ast::nodes::VarDecl),
}
impl Stmt {
    pub fn is_expr_stmt(&self) -> bool {
        ::core::matches!(self, Self::ExprStmt(..))
    }
    pub fn is_var_decl(&self) -> bool {
        ::core::matches!(self, Self::VarDecl(..))
    }
    pub fn as_expr_stmt(&self) -> ::core::option::Option<&crate::ast::nodes::ExprStmt> {
        if let Self::ExprStmt(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_var_decl(&self) -> ::core::option::Option<&crate::ast::nodes::VarDecl> {
        if let Self::VarDecl(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_expr_stmt(self) -> ::core::result::Result<crate::ast::nodes::ExprStmt, Self> {
        if let Self::ExprStmt(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_var_decl(self) -> ::core::result::Result<crate::ast::nodes::VarDecl, Self> {
        if let Self::VarDecl(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
}
impl crate::ast::AstNode for Stmt {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::nodes::ExprStmt as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::VarDecl as crate::ast::AstNode>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            kind if <crate::ast::nodes::ExprStmt as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::ExprStmt as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ExprStmt(node)))
            }
            kind if <crate::ast::nodes::VarDecl as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::VarDecl as crate::ast::AstNode>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::unreachable!()
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VarDecl(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::ExprStmt(syntax) => {
                <crate::ast::nodes::ExprStmt as crate::ast::AstNode>::syntax(syntax)
            }
            Self::VarDecl(syntax) => {
                <crate::ast::nodes::VarDecl as crate::ast::AstNode>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<Stmt> for crate::ast::nodes::ExprStmt {
    type Error = Stmt;
    #[inline]
    fn try_from(value: Stmt) -> ::core::result::Result<Self, Self::Error> {
        if let Stmt::ExprStmt(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Stmt> for crate::ast::nodes::VarDecl {
    type Error = Stmt;
    #[inline]
    fn try_from(value: Stmt) -> ::core::result::Result<Self, Self::Error> {
        if let Stmt::VarDecl(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::nodes::ExprStmt> for Stmt {
    #[inline]
    fn from(value: crate::ast::nodes::ExprStmt) -> Self {
        Self::ExprStmt(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::VarDecl> for Stmt {
    #[inline]
    fn from(value: crate::ast::nodes::VarDecl) -> Self {
        Self::VarDecl(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum StructFields {
    BracketedStructFields(crate::ast::nodes::BracketedStructFields),
    TupleStructFields(crate::ast::nodes::TupleStructFields),
}
impl StructFields {
    pub fn is_bracketed_struct_fields(&self) -> bool {
        ::core::matches!(self, Self::BracketedStructFields(..))
    }
    pub fn is_tuple_struct_fields(&self) -> bool {
        ::core::matches!(self, Self::TupleStructFields(..))
    }
    pub fn as_bracketed_struct_fields(
        &self,
    ) -> ::core::option::Option<&crate::ast::nodes::BracketedStructFields> {
        if let Self::BracketedStructFields(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_tuple_struct_fields(
        &self,
    ) -> ::core::option::Option<&crate::ast::nodes::TupleStructFields> {
        if let Self::TupleStructFields(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_bracketed_struct_fields(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::BracketedStructFields, Self> {
        if let Self::BracketedStructFields(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_tuple_struct_fields(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::TupleStructFields, Self> {
        if let Self::TupleStructFields(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
}
impl crate::ast::AstNode for StructFields {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::nodes::BracketedStructFields as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::TupleStructFields as crate::ast::AstNode>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate :: SyntaxNode :: kind (syntax) { kind if < crate :: ast :: nodes :: BracketedStructFields as crate :: ast :: AstNode > :: can_cast_from (kind) => { let node = match < crate :: ast :: nodes :: BracketedStructFields as crate :: ast :: AstNode > :: cast (syntax) { :: core :: option :: Option :: Some (node) => :: std :: borrow :: Cow :: into_owned (node) , :: core :: option :: Option :: None => if :: core :: cfg ! (debug_assertions) { :: core :: unreachable ! () } else { unsafe { :: core :: hint :: unreachable_unchecked () } } , } ; :: core :: option :: Option :: Some (:: std :: borrow :: Cow :: Owned (Self :: BracketedStructFields (node) ,) ,) } kind if < crate :: ast :: nodes :: TupleStructFields as crate :: ast :: AstNode > :: can_cast_from (kind) => { let node = match < crate :: ast :: nodes :: TupleStructFields as crate :: ast :: AstNode > :: cast (syntax) { :: core :: option :: Option :: Some (node) => :: std :: borrow :: Cow :: into_owned (node) , :: core :: option :: Option :: None => if :: core :: cfg ! (debug_assertions) { :: core :: unreachable ! () } else { unsafe { :: core :: hint :: unreachable_unchecked () } } , } ; :: core :: option :: Option :: Some (:: std :: borrow :: Cow :: Owned (Self :: TupleStructFields (node) ,) ,) } _ => :: core :: option :: Option :: None , }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::BracketedStructFields(syntax) => {
                <crate::ast::nodes::BracketedStructFields as crate::ast::AstNode>::syntax(syntax)
            }
            Self::TupleStructFields(syntax) => {
                <crate::ast::nodes::TupleStructFields as crate::ast::AstNode>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<StructFields> for crate::ast::nodes::BracketedStructFields {
    type Error = StructFields;
    #[inline]
    fn try_from(value: StructFields) -> ::core::result::Result<Self, Self::Error> {
        if let StructFields::BracketedStructFields(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<StructFields> for crate::ast::nodes::TupleStructFields {
    type Error = StructFields;
    #[inline]
    fn try_from(value: StructFields) -> ::core::result::Result<Self, Self::Error> {
        if let StructFields::TupleStructFields(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::nodes::BracketedStructFields> for StructFields {
    #[inline]
    fn from(value: crate::ast::nodes::BracketedStructFields) -> Self {
        Self::BracketedStructFields(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::TupleStructFields> for StructFields {
    #[inline]
    fn from(value: crate::ast::nodes::TupleStructFields) -> Self {
        Self::TupleStructFields(value)
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Type {
    FunctionType(crate::ast::nodes::FunctionType),
    GenericType(crate::ast::nodes::GenericType),
    TupleType(crate::ast::nodes::TupleType),
}
impl Type {
    pub fn is_function_type(&self) -> bool {
        ::core::matches!(self, Self::FunctionType(..))
    }
    pub fn is_generic_type(&self) -> bool {
        ::core::matches!(self, Self::GenericType(..))
    }
    pub fn is_tuple_type(&self) -> bool {
        ::core::matches!(self, Self::TupleType(..))
    }
    pub fn as_function_type(&self) -> ::core::option::Option<&crate::ast::nodes::FunctionType> {
        if let Self::FunctionType(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_generic_type(&self) -> ::core::option::Option<&crate::ast::nodes::GenericType> {
        if let Self::GenericType(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_tuple_type(&self) -> ::core::option::Option<&crate::ast::nodes::TupleType> {
        if let Self::TupleType(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_function_type(
        self,
    ) -> ::core::result::Result<crate::ast::nodes::FunctionType, Self> {
        if let Self::FunctionType(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_generic_type(self) -> ::core::result::Result<crate::ast::nodes::GenericType, Self> {
        if let Self::GenericType(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_tuple_type(self) -> ::core::result::Result<crate::ast::nodes::TupleType, Self> {
        if let Self::TupleType(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
}
impl crate::ast::AstNode for Type {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::nodes::FunctionType as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::GenericType as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::TupleType as crate::ast::AstNode>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            kind if <crate::ast::nodes::FunctionType as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::FunctionType as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FunctionType(node)))
            }
            kind if <crate::ast::nodes::GenericType as crate::ast::AstNode>::can_cast_from(
                kind,
            ) =>
            {
                let node =
                    match <crate::ast::nodes::GenericType as crate::ast::AstNode>::cast(syntax) {
                        ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                        ::core::option::Option::None => {
                            if ::core::cfg!(debug_assertions) {
                                ::core::unreachable!()
                            } else {
                                unsafe { ::core::hint::unreachable_unchecked() }
                            }
                        }
                    };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::GenericType(node)))
            }
            kind if <crate::ast::nodes::TupleType as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::TupleType as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TupleType(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::FunctionType(syntax) => {
                <crate::ast::nodes::FunctionType as crate::ast::AstNode>::syntax(syntax)
            }
            Self::GenericType(syntax) => {
                <crate::ast::nodes::GenericType as crate::ast::AstNode>::syntax(syntax)
            }
            Self::TupleType(syntax) => {
                <crate::ast::nodes::TupleType as crate::ast::AstNode>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<Type> for crate::ast::nodes::FunctionType {
    type Error = Type;
    #[inline]
    fn try_from(value: Type) -> ::core::result::Result<Self, Self::Error> {
        if let Type::FunctionType(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Type> for crate::ast::nodes::GenericType {
    type Error = Type;
    #[inline]
    fn try_from(value: Type) -> ::core::result::Result<Self, Self::Error> {
        if let Type::GenericType(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<Type> for crate::ast::nodes::TupleType {
    type Error = Type;
    #[inline]
    fn try_from(value: Type) -> ::core::result::Result<Self, Self::Error> {
        if let Type::TupleType(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::nodes::FunctionType> for Type {
    #[inline]
    fn from(value: crate::ast::nodes::FunctionType) -> Self {
        Self::FunctionType(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::GenericType> for Type {
    #[inline]
    fn from(value: crate::ast::nodes::GenericType) -> Self {
        Self::GenericType(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::TupleType> for Type {
    #[inline]
    fn from(value: crate::ast::nodes::TupleType) -> Self {
        Self::TupleType(value)
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
pub struct UnaryOp {
    syntax: crate::SyntaxNode,
}
impl crate::ast::AstNode for UnaryOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::UNARY_OP
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
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
pub enum UseBranchOrAlias {
    UseAlias(crate::ast::nodes::UseAlias),
    UseBranch(crate::ast::nodes::UseBranch),
}
impl UseBranchOrAlias {
    pub fn is_use_alias(&self) -> bool {
        ::core::matches!(self, Self::UseAlias(..))
    }
    pub fn is_use_branch(&self) -> bool {
        ::core::matches!(self, Self::UseBranch(..))
    }
    pub fn as_use_alias(&self) -> ::core::option::Option<&crate::ast::nodes::UseAlias> {
        if let Self::UseAlias(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn as_use_branch(&self) -> ::core::option::Option<&crate::ast::nodes::UseBranch> {
        if let Self::UseBranch(syntax) = self {
            ::core::option::Option::Some(syntax)
        } else {
            ::core::option::Option::None
        }
    }
    pub fn into_use_alias(self) -> ::core::result::Result<crate::ast::nodes::UseAlias, Self> {
        if let Self::UseAlias(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
    pub fn into_use_branch(self) -> ::core::result::Result<crate::ast::nodes::UseBranch, Self> {
        if let Self::UseBranch(syntax) = self {
            ::core::result::Result::Ok(syntax)
        } else {
            ::core::result::Result::Err(self)
        }
    }
}
impl crate::ast::AstNode for UseBranchOrAlias {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        <crate::ast::nodes::UseAlias as crate::ast::AstNode>::can_cast_from(kind)
            || <crate::ast::nodes::UseBranch as crate::ast::AstNode>::can_cast_from(kind)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            kind if <crate::ast::nodes::UseAlias as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::UseAlias as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::UseAlias(node)))
            }
            kind if <crate::ast::nodes::UseBranch as crate::ast::AstNode>::can_cast_from(kind) => {
                let node = match <crate::ast::nodes::UseBranch as crate::ast::AstNode>::cast(syntax)
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
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::UseBranch(node)))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::UseAlias(syntax) => {
                <crate::ast::nodes::UseAlias as crate::ast::AstNode>::syntax(syntax)
            }
            Self::UseBranch(syntax) => {
                <crate::ast::nodes::UseBranch as crate::ast::AstNode>::syntax(syntax)
            }
        }
    }
}
impl ::core::convert::TryFrom<UseBranchOrAlias> for crate::ast::nodes::UseAlias {
    type Error = UseBranchOrAlias;
    #[inline]
    fn try_from(value: UseBranchOrAlias) -> ::core::result::Result<Self, Self::Error> {
        if let UseBranchOrAlias::UseAlias(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::TryFrom<UseBranchOrAlias> for crate::ast::nodes::UseBranch {
    type Error = UseBranchOrAlias;
    #[inline]
    fn try_from(value: UseBranchOrAlias) -> ::core::result::Result<Self, Self::Error> {
        if let UseBranchOrAlias::UseBranch(this) = value {
            ::core::result::Result::Ok(this)
        } else {
            ::core::result::Result::Err(value)
        }
    }
}
impl ::core::convert::From<crate::ast::nodes::UseAlias> for UseBranchOrAlias {
    #[inline]
    fn from(value: crate::ast::nodes::UseAlias) -> Self {
        Self::UseAlias(value)
    }
}
impl ::core::convert::From<crate::ast::nodes::UseBranch> for UseBranchOrAlias {
    #[inline]
    fn from(value: crate::ast::nodes::UseBranch) -> Self {
        Self::UseBranch(value)
    }
}
