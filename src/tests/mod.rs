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

    fn e(children: Vec<Node>) -> Node {
        Node { children }
    }

    fn f() -> Node {
        Node { children: Vec::new() }
    }

    #[test]
    fn full_traverse() {
        let mut t = e(vec![
            e(vec![
                f(),
                f(),
            ]),
            e(vec![
                f(),
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

            assert!(!c.down());
            assert!(c.up());

            assert!(c.down());

            assert!(c.down());
            assert!(!c.down());
            assert!(c.up());

            assert!(!c.down());
            assert!(c.up());

            assert!(!c.down());
            assert!(!c.up());
        }
    }

    #[test]
    fn partial_traverse() {
        let mut t = e(vec![
            e(vec![
                e(vec![
                    f(),
                ]),
                f(),
            ]),
            e(vec![
                f(),
                f(),
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

mod link_tree_tests {
    use std::ptr;
    use tree::{Down, Link, LinkError, LinkTreeCursor};

    #[derive(Debug)]
    enum Node {
        Seq(Vec<Node>),
        Name(String, Box<Node>),
        Link(String),
    }

    impl Down for Node {
        fn down(&mut self, idx: usize) -> Option<*mut Self> {
            match self {
                &mut Node::Seq(ref mut children) =>
                    children.get_mut(idx).map(|c: &mut Self| c as *mut Self),
                &mut Node::Name(_, ref mut child) => {
                    if idx == 0 {
                        Some(&mut **child as *mut Self)
                    } else {
                        None
                    }
                },
                &mut Node::Link(_) => None,
            }
        }
    }

    impl Link for Node {
        fn name(&self) -> Option<&str> {
            match self {
                &Node::Name(ref name, _) => Some(name),
                _ => None,
            }
        }

        fn target(&self) -> Option<&str> {
            match self {
                &Node::Link(ref target) => Some(target),
                _ => None,
            }
        }
    }

    fn e(children: Vec<Node>) -> Node {
        Node::Seq(children)
    }

    fn n(name: &str, child: Node) -> Node {
        Node::Name(name.to_string(), Box::new(child))
    }

    fn k(target: &str) -> Node {
        Node::Link(target.to_string())
    }

    fn f() -> Node {
        Node::Seq(Vec::new())
    }

    #[test]
    fn full_traverse() {
        let mut t = e(vec![
            f(),
            f(),
            f(),
            n("go", e(vec![
                n("foo", e(vec![
                    f(),
                    f(),
                ])),
                e(vec![
                    k("foo"),
                ]),
            ])),
        ]);

        let mut c = LinkTreeCursor::new(&mut t, "go").unwrap();

        for _ in 0..100 {
            let start = c.get() as *const Node;

            assert!(c.down());
            assert!(c.down());
            assert!(c.down());

            {
                let here = c.get() as *const Node;
                let here_mut = c.get_mut() as *mut Node;
                assert!(ptr::eq(here, here_mut));
                assert!(!ptr::eq(here, start));

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

            assert!(!c.down());
            assert!(c.up());

            assert!(!c.down());
            assert!(c.up());

            assert!(c.down());
            assert!(c.down());
            assert!(c.down());
            assert!(c.down());
            assert!(c.down());
            assert!(!c.down());
            assert!(c.up());

            assert!(c.down());
            assert!(!c.down());
            assert!(c.up());

            assert!(!c.down());
            assert!(c.up());

            assert!(!c.down());
            assert!(c.up());

            assert!(!c.down());
            assert!(c.up());

            assert!(!c.down());
            assert!(c.up());

            assert!(!c.down());
            assert!(c.up());

            assert!(!c.down());
            assert!(!c.up());

            assert!(ptr::eq(c.get(), start));
        }
    }

    #[test]
    fn link_errors() {
        {
            let mut t = e(vec![
                k("foo"),
                n("bar", f()),
            ]);
            assert_eq!(
                LinkTreeCursor::new(&mut t, "bar").unwrap_err(),
                LinkError::BrokenLink,
            );
        }

        {
            let mut t = e(vec![
                n("bar", e(vec![
                    e(vec![
                        f(),
                        f(),
                        k("foo"),
                    ]),
                ])),
            ]);
            assert_eq!(
                LinkTreeCursor::new(&mut t, "bar").unwrap_err(),
                LinkError::BrokenLink,
            );
        }

        {
            let mut t = e(vec![
                f(),
                n("foo", n("foo", f())),
                n("bar", f()),
            ]);
            assert_eq!(
                LinkTreeCursor::new(&mut t, "foo").unwrap_err(),
                LinkError::DuplicateName,
            );
            assert_eq!(
                LinkTreeCursor::new(&mut t, "bar").unwrap_err(),
                LinkError::DuplicateName,
            );
        }

        {
            let mut t = n("foo", f());
            assert_eq!(
                LinkTreeCursor::new(&mut t, "bar").unwrap_err(),
                LinkError::BrokenLink,
            );
        }
    }
}
