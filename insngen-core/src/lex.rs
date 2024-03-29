use std::io::{ErrorKind, Read};
use std::iter::{FromIterator, FusedIterator};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literal {
    String(String),
    Character(char),
    Integer(String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Delimeter {
    Braces,
    Brackets,
    Parethesis,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenTree {
    Ident(String),
    Lit(Literal),
    Delimeted(Delimeter, TokenStream),
    Punct(char),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenStream {
    stream: Vec<TokenTree>,
}

impl TokenStream {
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.stream.iter())
    }
}

impl IntoIterator for TokenStream {
    type IntoIter = IntoIter;
    type Item = TokenTree;
    fn into_iter(self) -> IntoIter {
        IntoIter(self.stream.into_iter())
    }
}

impl<'a> IntoIterator for &'a TokenStream {
    type IntoIter = Iter<'a>;
    type Item = &'a TokenTree;
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenTree>>(it: I) -> Self {
        Self {
            stream: it.into_iter().collect(),
        }
    }
}

impl FromIterator<TokenStream> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenStream>>(it: I) -> Self {
        Self {
            stream: it.into_iter().flatten().collect(),
        }
    }
}

impl Extend<TokenTree> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, iter: I) {
        self.stream.extend(iter)
    }
}

impl Extend<TokenStream> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenStream>>(&mut self, iter: I) {
        self.stream.extend(iter.into_iter().flatten())
    }
}

pub struct Iter<'a>(std::slice::Iter<'a, TokenTree>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> FusedIterator for Iter<'a> {}
impl<'a> ExactSizeIterator for Iter<'a> {}

pub struct IntoIter(std::vec::IntoIter<TokenTree>);

impl<'a> Iterator for IntoIter {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl FusedIterator for IntoIter {}
impl ExactSizeIterator for IntoIter {}

pub fn lex<R: Read>(input: R) -> std::io::Result<TokenStream> {
    let mut stream = Vec::new();
    let mut iter = input.bytes();
    while let Some(c) = iter.next() {
        stream.push(match c {
            Ok(punct @ (b'.' | b',')) => TokenTree::Punct(punct as char),
            Ok(start @ (b'A'..=b'Z' | b'a'..=b'z' | b'$' | b'_')) => {
                let mut result = String::new();
                result.push(start as char);
                while let Some(Ok(c @ (b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'$' | b'_'))) =
                    iter.next()
                {
                    result.push(c as char);
                }
                TokenTree::Ident(result)
            }
            c => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    format!("Unexpected Character on Stream {:?}", c),
                ))
            }
        });
    }
    Ok(TokenStream { stream })
}
