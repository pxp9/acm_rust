use super::*;
#[test]
fn basics() {
    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.append(1);
    list.append(2);
    list.push(2);
    list.append(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));

    // append some more just to make sure nothing's corrupted
    list.append(4);
    list.append(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(3));

    // Check exhaustion
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), None);

    // Check the exhaustion case fixed the pointer right
    list.append(6);
    list.append(7);

    // Check normal removal
    assert_eq!(list.pop(), Some(6));
    assert_eq!(list.pop(), Some(7));
    assert_eq!(list.pop(), None);
}

#[test]
fn into_iter() {
    let mut list = List::new();
    list.append(1);
    list.append(2);
    list.append(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    let mut list = List::new();
    list.append(1);
    list.append(2);
    list.append(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.append(1);
    list.append(2);
    list.append(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), None);
}

#[test]
fn miri_food() {
    let mut list = List::new();

    list.append(1);
    list.append(2);
    list.append(3);

    assert!(list.pop() == Some(1));
    list.append(4);
    assert!(list.pop() == Some(2));
    list.append(5);

    assert!(list.peek() == Some(&3));
    list.append(6);
    list.peek_mut().map(|x| *x *= 10);
    assert!(list.peek() == Some(&30));
    assert!(list.pop() == Some(30));

    for elem in list.iter_mut() {
        *elem *= 100;
    }

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&400));
    assert_eq!(iter.next(), Some(&500));
    assert_eq!(iter.next(), Some(&600));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    assert!(list.pop() == Some(400));
    list.peek_mut().map(|x| *x *= 10);
    assert!(list.peek() == Some(&5000));
    list.append(7);

    // Drop it on the ground and let the dtor exercise itself
}
