#[macro_export]
macro_rules! vecmac {
    ($($element:expr),*) => {{
        const COUNT: usize = $crate::count![@COUNT; $($element),*];
        let mut v_items = Vec::with_capacity(COUNT);
        $(v_items.push($element);)*
        v_items
    }};
    ($($element:expr,)*) => {{
        $crate::vecmac![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let mut v_items = Vec::new();
        v_items.resize($count, $element);
        v_items
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr), *) => {
        <[()]>::len(&[$($crate::count![@SUBSTITUDE; $element]),*])
    };
    (@SUBSTITUDE; $_element:expr) => {
        ()
    }
}

/// ```compile_fail
/// let x: Vec<u32> = vecmac![36; "roy"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;

/// ```compile_fail
/// fn non_clone() {
///     struct NonCloneStruct;
///     let y = NonCloneStruct;
///     let x: Vec<NonCloneStruct> = vecmac::vecmac![Foo; 2];
///     assert!(!x.is_empty());
///     assert_eq!(x.len(), 2);
///     assert_eq!(x[0], Foo);
///     assert_eq!(x[1], Foo);
/// }
/// ```
#[allow(dead_code)]
struct NonCloneStruct;

// Read the `Little Book of Macros` to learn deeply about macros.
// `($element:expr)` accepts as many inputs as many is provided by caller.
// `($(pattern),+)` pettern will be repeated based on the number of input it gets and separated by the token `,`.
// Here `+` denotes the repeatations of one or more time based on the given inputs.
// only one expression can be returned. If we need multiple lines, we surround it with `{{ steps to define macro }}`
// `$(v_items.push($element);)+` means that repeat it as many times as the pattern `($element:expr)`
// `+` means the repeatition. Something like one or more of previous pattern.
// `,` is the separator of pattern.
// `$(,)?` allows 0 or 1 number of trailing comma after the pattern
