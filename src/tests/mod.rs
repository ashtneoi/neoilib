mod func_iter_tests {
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
}

mod peek_while_tests {
    use peek_while;

    #[test]
    fn count_limit() {
        let mut c = (1..).peekable();
        let to4: Vec<_> = peek_while(&mut c, |&x| x <= 4).collect();
        assert_eq!(&to4, &[1, 2, 3, 4]);
        let to4: Vec<_> = peek_while(&mut c, |&x| x <= 8).collect();
        assert_eq!(&to4, &[5, 6, 7, 8]);
    }
}
