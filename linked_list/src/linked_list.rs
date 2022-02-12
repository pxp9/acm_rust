#[cfg(test)]
#[path = "test/tests.rs"]
mod tests;
use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct List<T>
where
    T: fmt::Display,
{
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Eq)]
struct Node<T>
where
    T: fmt::Display,
{
    elem: T,
    next: Link<T>,
}

impl<T> fmt::Display for Node<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.elem)
    }
}
impl<T> fmt::Display for List<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let opt = self.head.as_ref();
        match opt {
            None => {
                write!(f, "]")
            }
            Some(boxed) => {
                // Option<&Box<Node<T>>>
                let mut next_opt = (**boxed).next.as_ref();
                if let None = next_opt {
                    write!(f, "{}]", **boxed)
                } else {
                    write!(f, "{}", **boxed)?;
                    while let Some(boxx) = next_opt {
                        next_opt = (**boxx).next.as_ref();
                        if let None = next_opt {
                            write!(f, "{}]", **boxx)?;
                        } else {
                            write!(f, " -> {}", **boxx)?;
                        }
                    }
                    Ok(())
                    //self.fmt_aux(f, next_opt)
                }
            }
        }
    }
}
impl<T> List<T>
where
    T: fmt::Display,
{
    pub fn new() -> Self {
        List { head: None, len: 0 }
    }
    /*fn fmt_aux(&self, f: &mut fmt::Formatter, opt: Option<&Box<Node<T>>>) -> fmt::Result {
        match opt {
            None => {
                write!(f, "]")
            }
            Some(boxed) => {
                let next_opt = (**boxed).next.as_ref();
                if let None = next_opt {
                    write!(f, "{}", **boxed)?;
                } else {
                    write!(f, " -> {}", **boxed)?;
                }
                self.fmt_aux(f, next_opt)
            }
        }
    }*/
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.len = self.len + 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.len = self.len - 1;
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn len(&self) -> usize {
        self.len
    }
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    #[allow(dead_code)]
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    #[allow(dead_code)]
    pub fn into_iter(self) -> IntoIter<T>
    where
        T: fmt::Display,
    {
        IntoIter(self)
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> Iter<'_, T>
    where
        T: fmt::Display,
    {
        Iter {
            next: self.head.as_deref(),
        }
    }
    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> IterMut<'_, T>
    where
        T: fmt::Display,
    {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T>
where
    T: fmt::Display,
{
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>)
where
    T: fmt::Display;

impl<T> Iterator for IntoIter<T>
where
    T: fmt::Display,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T: std::fmt::Display>
where
    T: fmt::Display,
{
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: fmt::Display,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T>
where
    T: fmt::Display,
{
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: fmt::Display,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
