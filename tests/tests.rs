use unsound_helpers::*;

#[test]
fn test_make_static() {
    let my_ref;
    {
        let my_char = 'X';
        my_ref = make_static(&my_char);
    }
    // use-after-free
    assert_eq!(*my_ref, 'X');
}

#[test]
fn test_make_static_mut() {
    let my_ref;
    {
        let mut my_char = 'X';
        my_ref = make_static_mut(&mut my_char);
    }
    // use-after-free
    *my_ref = 'Y';
    assert_eq!(*my_ref, 'Y');
}

#[test]
fn test_make_mut() {
    let x = 42;
    let x_ref = make_mut(&x);
    // mutate an immutable value
    *x_ref = 43;
    assert_eq!(x, 43);
}

#[test]
fn test_copy() {
    let mut x = 42;
    let x_mut = &mut x;
    // alias a &mut
    let x_mut2 = copy(&x_mut);
    *x_mut += 1;
    *x_mut2 += 1;
    assert_eq!(x, 44);
}

#[test]
fn test_as_bytes() {
    #[repr(C)]
    struct Foo {
        x: u8,
        // padding
        y: u16,
    }
    let my_foo = Foo { x: 1, y: 2 };
    let bytes = as_bytes(&my_foo);
    // read an uninitialized byte
    if bytes[1] == 0 {
        assert_eq!(bytes[0], 1);
    } else {
        assert_eq!(bytes[2..4], 2u16.to_ne_bytes());
    }
}

#[test]
fn test_as_bytes_mut() {
    let mut my_bool = false;
    // create an invalid bool
    as_bytes_mut(&mut my_bool)[0] = 0xff;
    assert!(my_bool);
}

#[test]
fn test_destroy() {
    let mut x = 42;
    // double free
    destroy(&mut x);
    destroy(&mut x);
}

#[test]
fn test_sender() {
    use std::rc::Rc;
    let rc = Rc::new(42);
    // data race on the Rc refcount
    let sender = Sender::new(rc.clone());
    std::thread::spawn(|| {
        let sender = sender;
        _ = sender.inner.clone();
    });
    _ = rc.clone();
}
