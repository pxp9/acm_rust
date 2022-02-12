use std::clone::Clone;
use std::fmt;
use std::ops::{Add, Mul};
use std::ptr;
#[cfg(test)]
#[path = "test/tests.rs"]
mod tests;
#[derive(Debug, PartialEq, Eq)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

type Link<T> = *mut Node<T>;

#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> fmt::Display for Node<T>
where
    T: fmt::Display + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.elem)
    }
}

impl<T> fmt::Display for List<T>
where
    T: fmt::Display + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        unsafe {
            if self.head.is_null() {
                write!(f, "]")
            } else {
                let mut aux = self.head;
                write!(f, "{}", (*aux).elem)?;
                aux = (*aux).next;
                if aux.is_null() {
                    write!(f, "]")
                } else {
                    while !aux.is_null() {
                        write!(f, " -> {}", (*aux).elem)?;
                        aux = (*aux).next;
                    }
                    write!(f, "]")
                }
            }
        }
    }
}

impl<T> Add for &List<T>
where
    T: fmt::Display + Clone,
{
    type Output = List<T>;
    fn add(self, other: &List<T>) -> List<T> {
        let mut list: List<T> = List::new();
        let iter = self.iter().chain(other.iter());
        for node in iter {
            list.append(node.clone());
        }
        list
    }
}

impl<T> Mul<u8> for &List<T>
where
    T: fmt::Display + Clone,
{
    type Output = List<T>;
    fn mul(self, number: u8) -> List<T> {
        let mut list: List<T> = List::new();
        for _n in 0..number {
            list = &list + self;
        }
        list
    }
}

impl<T> Mul<&List<T>> for u8
where
    T: fmt::Display + Clone,
{
    type Output = List<T>;
    fn mul(self, original: &List<T>) -> List<T> {
        let mut list: List<T> = List::new();
        for _n in 0..self {
            list = &list + original;
        }
        list
    }
}
pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }
    pub fn append(&mut self, elem: T) {
        let new_tail = Box::into_raw(Box::new(Node {
            elem: elem,
            next: ptr::null_mut(),
        }));

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = new_tail;
            }
        } else {
            self.head = new_tail;
        }

        self.tail = new_tail;
        self.len = self.len + 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }
                self.len = self.len - 1;
                Some(head.elem)
            }
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_head = Box::into_raw(Box::new(Node {
            elem: elem,
            next: self.head,
        }));

        (*self).head = new_head;
        self.len = self.len + 1;
    }
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}
