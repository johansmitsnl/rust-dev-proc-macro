use my_proc_macros_lib::my_fn_like_proc_macro;

macro_rules! view {
    (if ($cond:expr) $block:block else $else_block:block ) => {
        if $cond {
            $block
        } else {
            $else_block
        }
    };
    (if ($cond:expr) $block:block ) => {
        if $cond {
            $block
        } else {
            ""
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::my_fn_like_proc_macro;

    #[test]
    fn it_works() {
        let result = view!(
            if (1 + 1 == 2) {
                "Hi it is 2"
            }
        );
        assert_eq!(result, "Hi it is 2");
    }

    #[test]
    fn it_works_without_else() {
        let result = view!(
            if (1 + 1 == 3) {
                "Hi it is 2"
            }
        );
        assert_eq!(result, "");
    }

    #[test]
    fn it_works_with_else() {
        let result = view!(
            if (1 + 1 == 3) {
                "Hi it is 2"
            } else {
                "Its 3"
            }
        );
        assert_eq!(result, "Its 3");
    }


    #[test]
    fn it_works_proc_with_else() {
        let result = my_fn_like_proc_macro!(
            if 1 + 1 == 2 {
                "Hi it is 2"
            } else {
                "With a else case"
            }
        );
        assert_eq!(result, "Hi it is 2");
    }


    #[test]
    fn it_works_proc_without_else() {
        let result = my_fn_like_proc_macro!(
            if 1 + 1 == 2 {
                "Hi it is 2"
            }
        );
        assert_eq!(result, "Hi it is 2");
    }

    #[test]
    fn it_works_proc_without_else() {
        let result = my_fn_like_proc_macro!(
            if 1 + 1 == 3 {
                "Hi it is 2"
            }
        );
        assert_eq!(result, "");
    }
}