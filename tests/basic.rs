mod util;
use self::util::test_script;

#[test]
fn test_return() {
    test_script(
        r#"
            return 3
        "#,
        3,
    );
}

#[test]
fn test_local() {
    test_script(
        r#"
            local i = 7
            local j = 30
            local i = 42
            return i
        "#,
        42,
    );
}

#[test]
fn test_assignment() {
    test_script(
        r#"
            local i = 7
            local i = 30
            i = 35
            return i
        "#,
        35,
    );
}

#[test]
fn test_function() {
    test_script(
        r#"
            local function test(a, b)
                return a + b
            end
            local i = test(1, 2)
            return i
        "#,
        3,
    );
}

#[test]
fn test_equality() {
    test_script(
        r#"
            return 1 == 1
        "#,
        true,
    );

    test_script(
        r#"
            return 1 == 2
        "#,
        false,
    );

    test_script(
        r#"
            local i = 1
            return i == 1
        "#,
        true,
    );

    test_script(
        r#"
            local i = 2
            local j = 3
            return i == j
        "#,
        false,
    );

    test_script(
        r#"
            local i = 2
            local j = 2
            return (i == j) == true
        "#,
        true,
    );
}

#[test]
fn test_short_circuit() {
    test_script(
        r#"
            return 1 == 1 and 2 == 2
        "#,
        true,
    );

    test_script(
        r#"
            return 1 == 1 or 1 == 2
        "#,
        true,
    );

    test_script(
        r#"
            return 1 == 1 and 2 == 3
        "#,
        false,
    );

    test_script(
        r#"
            return false or false
        "#,
        false,
    );

    test_script(
        r#"
            local i = 0
            local function t(p)
                i = i + p
            end
            local _ = (false and t(1))
            local _ = (true and t(2))
            local _ = (false or t(3))
            local _ = (true or t(4))
            return i
        "#,
        5,
    );
}
