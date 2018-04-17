use func_iter;

#[test]
fn count() {
    let mut i = 0;
    let mut c = func_iter(|| {
        let ii = i;
        i = i + 1;
        Some(ii)
    });
    assert_eq!(c.next(), Some(0));
    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
}

#[test]
fn repeat() {
    let mut c = func_iter(|| Some("spoon".to_string()));
    let s = Some("spoon".to_string());
    assert_eq!(c.next(), s);
    assert_eq!(c.next(), s);
    assert_eq!(c.next(), s);
    assert_eq!(c.next(), s);
}

#[test]
fn stop() {
    let mut stop_next = false;
    let mut c = func_iter(|| {
        if stop_next {
            None
        } else {
            stop_next = true;
            Some(-5)
        }
    });
    assert_eq!(c.next(), Some(-5));
    assert_eq!(c.next(), None);
}
