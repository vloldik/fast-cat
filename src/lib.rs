#![doc = include_str!("../Readme.md")]

#[macro_export]
macro_rules! _concat_str_internal {
    (
        @pre {$($pre:stmt,)*},
        @strname $strname:ident,
        @len $($len:expr)?, 
        @appends {$($appends:expr,)*},
        $head:literal $(, $($tail:tt)*)?
    ) => {
        $crate::_concat_str_internal!(
            @pre {$($pre, )*
                let tmp_res = $head
            ,},            
            @strname $strname,
            @len $($len +)? tmp_res.len(), 
            @appends {
                $($appends,)* 
                $strname.push_str(tmp_res),
            }, 
            $($($tail)*)?
        )
    };

    (
        @pre {$($pre:stmt,)*},
        @strname $strname:ident,
        @len $($len:expr)?, 
        @appends {$($appends:expr,)*},
        $head:ident $(, $($tail:tt)*)?
    ) => {
        $crate::_concat_str_internal!(
            @pre {$($pre, )*},
            @strname $strname,
            @len $($len +)? $head.len(), 
            @appends {
                $($appends,)* 
                $strname.push_str($head),
            }, 
            $($($tail)*)?
        )
    };

    (
        @pre {$($pre:stmt,)*},
        @strname $strname:ident,
        @len $($len:expr)?, 
        @appends {$($appends:expr,)*},
        $head:expr $(, $($tail:tt)*)?
    ) => {
        $crate::_concat_str_internal!(
            @pre {$($pre, )*
                let tmp_res = $head
            ,},
            @strname $strname,
            @len $($len +)? tmp_res.len(), 
            @appends {
                $($appends,)* 
                $strname.push_str(tmp_res),
            }, 
            $($($tail)*)?
        )
    };

    (
        @pre {$($pre:stmt,)*},
        @strname $strname:ident,
        @len $len:expr,
        @appends {$($appends:expr,)*},
    ) => {
        {
            $($pre)*
            let mut $strname = String::with_capacity($len);
            $($appends;)*
            $strname
        }
    };
}

/// Concatenates a sequence of string-like arguments into a new `String`.
///
/// This macro offers high-performance string concatenation by calculating the total
/// length of all arguments upfront and performing only a **single memory allocation**.
/// It serves as a more efficient alternative to repeated use of the `+` operator or
/// `format!` when all arguments are string-based.
///
/// # Arguments
///
/// The macro accepts a comma-separated list of arguments that can be converted to `&str`:
/// - String literals (e.g., `"hello"`).
/// - `&String` variables (e.g., `&my_string`).
/// - `&str` slices (e.g., `my_slice`).
/// - Expressions that evaluate to a string type (e.g., `&user.name` or `&num.to_string()`).
///
/// # Panics
///
/// The macro itself does not introduce panics. However, expressions passed to it
/// might panic, for example, due to indexing out of bounds.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use fast_cat::concat_str; // Замените `fast_cat` на имя вашего крейта
/// let s = concat_str!("the quick ", "brown ", "fox");
/// assert_eq!(s, "the quick brown fox");
/// ```
///
/// Mixing literals and variables:
/// ```
/// # use fast_cat::concat_str;
/// let color = "brown";
/// let animal = String::from("fox");
/// let sentence = concat_str!("the quick ", color, " ", &animal, " jumps over...");
/// assert_eq!(sentence, "the quick brown fox jumps over...");
/// ```
///
/// Using expressions:
/// ```
/// let num = 42;
/// let text = concat_str!("The answer is ", &num.to_string(), ".");
/// assert_eq!(text, "The answer is 42.");
/// ```
#[macro_export]
macro_rules! concat_str {
    ($($tt:tt)+) => {
        $crate::_concat_str_internal!(@pre {}, @strname res_string, @len, @appends {}, $($tt)+)
    };
}


#[cfg(test)]
mod tests {
    #[test]
    fn rtest() {
        let a = "test";
        let b = "local".to_string();
        let c = "focal".to_string();
        let i = 3;

        assert_eq!(
            concat_str!("", ""),
            ""
        );

        assert_eq!(
            concat_str!("so, ", a, "test"),
            "so, testtest"
        );

        assert_eq!(
            concat_str!("so, ", &b, " ", &c),
            "so, local focal"
        );

        assert_eq!(
            concat_str!("so, ", &i.to_string(), " ", &c),
            "so, 3 focal"
        );

        // recursion limit by default is 128, so it should compile
        let repeat = "repeat?";
        let repeat_o = "repeat?".to_string();
        assert_eq!(
            concat_str!(
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
            ),
            "repeat?".repeat(100)
        );
    }

    #[test]
    fn explicit_failure_test() {
        let short_str = "1".to_string();
        let long_str = "long_string".to_string();

        assert_eq!(
            concat_str!(&short_str, " ", &long_str),
            "1 long_string"
        );
    }

    #[cfg(feature="count-allocations")]
    #[test]
    fn test_single_alloc() {
        let repeat = "repeat?";
        let repeat_o = "repeat?".to_string();
        let alloc_count = allocation_counter::measure(|| {
            let _ = concat_str!(
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
                "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, "repeat?", repeat, &repeat_o, repeat,
            );
        });
        assert_eq!(alloc_count.count_total, 1);
    }
}
