use std::{
    borrow::Cow,
    iter::{FromIterator, FusedIterator},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literal<'a> {
    String(Cow<'a, str>),
    Character(char),
    Integer(i128),
}

impl<'a> Literal<'a> {
    pub fn clone_to_static(&self) -> Literal<'static> {
        match self {
            Self::String(c) => Literal::String(Cow::Owned(c.to_string())),
            Self::Character(c) => Literal::Character(*c),
            Self::Integer(i) => Literal::Integer(*i),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Delimeter {
    Braces,
    Brackets,
    Parethesis,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenTree<'a> {
    Ident(Cow<'a, str>),
    Lit(Literal<'a>),
    Delimeted(Delimeter, TokenStream<'a>),
    Punct(char),
}

impl<'a> TokenTree<'a> {
    pub fn clone_to_static(&self) -> TokenTree<'static> {
        match self {
            Self::Ident(c) => TokenTree::Ident(Cow::Owned(c.to_string())),
            Self::Lit(l) => TokenTree::Lit(l.clone_to_static()),
            Self::Delimeted(d, strm) => TokenTree::Delimeted(*d, strm.clone_to_static()),
            Self::Punct(c) => TokenTree::Punct(*c),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenStream<'a> {
    stream: Vec<TokenTree<'a>>,
}

impl<'a> TokenStream<'a> {
    pub fn clone_to_static(&self) -> TokenStream<'static> {
        TokenStream {
            stream: self.stream.iter().map(TokenTree::clone_to_static).collect(),
        }
    }

    pub fn iter(&self) -> Iter<'_, 'a> {
        Iter(self.stream.iter())
    }
}

impl<'a> IntoIterator for TokenStream<'a> {
    type IntoIter = IntoIter<'a>;
    type Item = TokenTree<'a>;
    fn into_iter(self) -> IntoIter<'a> {
        IntoIter(self.stream.into_iter())
    }
}

impl<'a, 'b: 'a> IntoIterator for &'a TokenStream<'b> {
    type IntoIter = Iter<'a, 'b>;
    type Item = &'a TokenTree<'b>;
    fn into_iter(self) -> Iter<'a, 'b> {
        self.iter()
    }
}

impl<'a, 'b: 'a> FromIterator<TokenTree<'b>> for TokenStream<'a> {
    fn from_iter<I: IntoIterator<Item = TokenTree<'b>>>(it: I) -> Self {
        Self {
            stream: it.into_iter().collect(),
        }
    }
}

impl<'a, 'b: 'a> FromIterator<TokenStream<'b>> for TokenStream<'a> {
    fn from_iter<I: IntoIterator<Item = TokenStream<'b>>>(it: I) -> Self {
        Self {
            stream: it.into_iter().flatten().collect(),
        }
    }
}

impl<'a> Extend<TokenTree<'a>> for TokenStream<'a> {
    fn extend<I: IntoIterator<Item = TokenTree<'a>>>(&mut self, iter: I) {
        self.stream.extend(iter)
    }
}

impl<'a> Extend<TokenStream<'a>> for TokenStream<'a> {
    fn extend<I: IntoIterator<Item = TokenStream<'a>>>(&mut self, iter: I) {
        self.stream.extend(iter.into_iter().flatten())
    }
}

pub struct Iter<'a, 'b: 'a>(std::slice::Iter<'a, TokenTree<'b>>);

impl<'a, 'b: 'a> Iterator for Iter<'a, 'b> {
    type Item = &'a TokenTree<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, 'b: 'a> FusedIterator for Iter<'a, 'b> {}
impl<'a, 'b: 'a> ExactSizeIterator for Iter<'a, 'b> {}

pub struct IntoIter<'a>(std::vec::IntoIter<TokenTree<'a>>);

impl<'a> Iterator for IntoIter<'a> {
    type Item = TokenTree<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> FusedIterator for IntoIter<'a> {}
impl<'a> ExactSizeIterator for IntoIter<'a> {}
