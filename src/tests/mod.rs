mod func_iter_tests {
    use iter::func_iter;

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
    use iter::peek_while;

    #[test]
    fn count_limit() {
        let mut c = (1..).peekable();
        let to4: Vec<_> = peek_while(&mut c, |&x| x <= 4).collect();
        assert_eq!(&to4, &[1, 2, 3, 4]);
        let to4: Vec<_> = peek_while(&mut c, |&x| x <= 8).collect();
        assert_eq!(&to4, &[5, 6, 7, 8]);
    }
}

mod tree_tests {
    use std::ptr;
    use tree::{Down, TreeCursor};

    struct Node {
        children: Vec<Node>,
    }

    impl Down for Node {
        fn down(&mut self, idx: usize) -> Option<*mut Self> {
            self.children.get_mut(idx).map(|c: &mut Self| c as *mut Self)
        }
    }

    fn n(children: Vec<Node>) -> Node {
        Node { children }
    }

    fn e() -> Node {
        Node { children: Vec::new() }
    }

    #[test]
    fn full_traverse() {
        let mut t = n(vec![
            n(vec![
                e(),
                e(),
            ]),
            n(vec![
                e(),
            ]),
        ]);
        let root = &t as *const Node;

        let mut c = TreeCursor::new(&mut t);

        for _ in 0..100 {
            assert!(ptr::eq(c.get(), root));

            assert!(c.down());

            {
                let here = c.get() as *const Node;
                let here_mut = c.get_mut() as *mut Node;
                assert!(ptr::eq(here, here_mut));
                assert!(!ptr::eq(here, root));

                assert!(c.down());
                assert!(!ptr::eq(c.get(), here));
                assert!(!c.down());
                assert!(c.up());

                assert!(ptr::eq(c.get(), here));
                assert!(ptr::eq(c.get_mut(), here_mut));
            }

            assert!(c.down());
            assert!(!c.down());
            assert!(c.up());
            assert!(c.up());

            assert!(c.down());

            assert!(c.down());
            assert!(!c.down());
            assert!(c.up());
            assert!(c.up());

            assert!(!c.down());
            assert!(!c.up());
        }
    }

    #[test]
    fn partial_traverse() {
        let mut t = n(vec![
            n(vec![
                n(vec![
                    e(),
                ]),
                e(),
            ]),
            n(vec![
                e(),
                e(),
            ]),
        ]);
        let root = &t as *const Node;

        let mut c = TreeCursor::new(&mut t);

        for _ in 0..100 {
            assert!(ptr::eq(c.get(), root));

            assert!(c.down());

            {
                let here = c.get() as *const Node;
                let here_mut = c.get_mut() as *mut Node;
                assert!(ptr::eq(here, here_mut));
                assert!(!ptr::eq(here, root));

                assert!(c.down());
                assert!(!ptr::eq(c.get(), here));
                assert!(c.up());

                assert!(ptr::eq(c.get(), here));
                assert!(ptr::eq(c.get_mut(), here_mut));
            }

            assert!(c.down());
            assert!(c.up());
            assert!(c.up());

            assert!(c.down());

            assert!(c.down());
            assert!(c.up());
            assert!(c.up());
            assert!(!c.up());
        }
    }
}
