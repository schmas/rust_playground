// MACRO CAPTURES

/* expr
    matches to a valid rust expression
    "hello".to_string(), vec![1, 2, 3], 1 + 2, 1
*/

/* stmt
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable name, function name, module name
*/

/* ty
    matches to a rust type
    i32, Vec<String>, Option<T>
*/

/* path
    matches to a rust path
    std::collections::HashMap
*/

// REPITITION SPECIFIER

// * - match zero or more repititions
// + - match one or more repititions
// ? - Match zero or one repetition

#[allow(unused_macros)]
macro_rules! mad_skills_expr {
    ($x: expr) => {
        format!("You sent an expression: {}", $x)
    };
}

#[allow(unused_macros)]
macro_rules! mad_skills_ty {
    ($x: ty) => {
        match stringify!($x) {
            "i32" => "You sent an i32 type".to_string(),
            _ => "You sent something else".to_string(),
        }
    };
}

#[macro_export]
macro_rules! my_vec {
    ($($x: expr), +) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )+
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn tests_declarative_macro_expr() {
        let some_var = mad_skills_expr!(1 + 3);
        dbg!(some_var);
    }

    #[test]
    fn tests_declarative_macro_ty() {
        let some_var = mad_skills_ty!(i32);
        dbg!(&some_var);
        assert_eq!(some_var, "You sent an i32 type");

        let some_var = mad_skills_ty!(u8);
        dbg!(&some_var);
        assert_eq!(some_var, "You sent something else");
    }

    #[test]
    fn tests_declarative_macro_vec() {
        let mut x: Vec<i32> = vec![1, 2, 3];
        let mut y: Vec<i32> = my_vec!(1);
        dbg!(y);
    }
}
