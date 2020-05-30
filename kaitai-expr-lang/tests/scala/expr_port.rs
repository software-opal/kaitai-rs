use crate::ast::func::*;
use crate::Expression;



#[test]
fn it_parses_1_plus_2() {
    assert_eq! {Expression.parse("1 + 2"), (BinOp(IntNum(1), Add, IntNum(2)))
    }
}
#[test]
fn it_parses_1_plus_2_plus_5() {
    assert_eq! {Expression.parse("1 + 2 + 5"), (
        BinOp(BinOp(IntNum(1), Add, IntNum(2)), Add, IntNum(5))
      )
    }
}
#[test]
fn it_parses_1_plus_2_div_7_plus_8() {
    assert_eq! {Expression.parse("(1 + 2) / (7 * 8)"), (
        BinOp(
          BinOp(IntNum(1), Add, IntNum(2)),
          Div,
          BinOp(IntNum(7), Mult, IntNum(8))
        )
      )
    }
}
#[test]
fn it_parses_1_less_than_2() {
    assert_eq! {Expression.parse("1 < 2"), (Compare(IntNum(1), Lt, IntNum(2)))
    }
}
#[test]
fn it_parses_a_index_42() {
    assert_eq! {Expression.parse("a[42]"), (Subscript(Name(identifier("a")), IntNum(42)))
    }
}
#[test]
fn it_parses_a_index_42_minus_2() {
    assert_eq! {Expression.parse("a[42 - 2]"), (
        Subscript(
          Name(identifier("a")),
          BinOp(IntNum(42), Sub, IntNum(2))
        )
      )
    }
}
#[test]
fn it_parses_2_less_than_3_inline_if_foo_string_or_bar_string() {
    assert_eq! {Expression.parse("2 < 3 ? \"foo\" : \"bar\""), (
        IfExp(
          Compare(IntNum(2), Lt, IntNum(3)),
          Str("foo"),
          Str("bar")
        )
      )
    }
}
#[test]
fn it_parses_bitwise_invert_operation() {
    assert_eq! {Expression.parse("~777"), (UnaryOp(Invert, IntNum(777)))
    }
}
#[test]
fn it_parses_invert_7_plus_3() {
    assert_eq! {Expression.parse("~(7+3)"), (UnaryOp(Invert, BinOp(IntNum(7), Add, IntNum(3))))
    }

    // Enums
}
#[test]
fn it_parses_enum_by_label_port_http() {
    assert_eq! {Expression.parse("port::http"), (EnumByLabel(identifier("port"), identifier("http")))
    }
}
#[test]
fn it_parses_enum_by_path_some_type_port_http() {
    assert_eq! {Expression.parse("some_type::port::http"), (
        EnumByLabel(
          identifier("port"),
          identifier("http"),
          typeId(absolute = false, Seq("some_type"))
        )
      )
    }
}
#[test]
fn it_parses_parent_type_child_type_port_http() {
    assert_eq! {Expression.parse("parent_type::child_type::port::http"), (
        EnumByLabel(
          identifier("port"),
          identifier("http"),
          typeId(absolute = false, Seq("parent_type", "child_type"))
        )
      )
    }
}
#[test]
fn it_parses_parent_type_child_type_port_http() {
    assert_eq! {Expression.parse("::parent_type::child_type::port::http"), (
        EnumByLabel(
          identifier("port"),
          identifier("http"),
          typeId(absolute = true, Seq("parent_type", "child_type"))
        )
      )
    }
}
#[test]
fn it_parses_port_http_to_i_plus_8000_equals_8080() {
    assert_eq! {Expression.parse("port::http.to_i + 8000 == 8080"), (
        Compare(
          BinOp(
            Attribute(
              EnumByLabel(identifier("port"),identifier("http")),
              identifier("to_i")
            ),
            Add,
            IntNum(8000)
          ),
          Eq,
          IntNum(8080)
        )
      )
    }
}
#[test]
fn it_parses_array_of_1_and_2_and_0x1234() {
    assert_eq! {Expression.parse("[1, 2, 0x1234]"), (
        List(ArrayBuffer(IntNum(1), IntNum(2), IntNum(4660)))
      )
    }

    // Boolean literals
}
#[test]
fn it_parses_true() {
    assert_eq! {Expression.parse("true"), (Bool(true))
    }
}
#[test]
fn it_parses_false() {
    assert_eq! {Expression.parse("false"), (Bool(false))
    }
}
#[test]
fn it_parses_truer() {
    assert_eq! {Expression.parse("truer"), (Name(identifier("truer")))
    }

    // Boolean operations
}
#[test]
fn it_parses_not_foo() {
    assert_eq! {Expression.parse("not foo"), (
        UnaryOp(
          Ast.unaryop.Not,
          Name(identifier("foo"))
        )
      )
    }
}
#[test]
fn it_parses_note_len() {
    assert_eq! {Expression.parse("note_len"), (Name(identifier("note_len")))
    }
}
#[test]
fn it_parses_notnot() {
    assert_eq! {Expression.parse("notnot"), (Name(identifier("notnot")))
    }
}
#[test]
fn it_parses_not_not_true() {
    assert_eq! {Expression.parse("not not true"), (
        UnaryOp(
          Ast.unaryop.Not,
          UnaryOp(
            Ast.unaryop.Not,
            Bool(true)
          )
        )
      )
    }

    // String literals
}
#[test]
fn it_parses_simple_string() {
    assert_eq! {Expression.parse("\"abc\""), (Str("abc"))
    }
}
#[test]
fn it_parses_interpolated_string_with_newline() {
    assert_eq! {Expression.parse("\"abc\\ndef\""), (Str("abc\ndef"))
    }
}
#[test]
fn it_parses_non_interpolated_string_with_newline() {
    assert_eq! {Expression.parse("'abc\\ndef'"), (Str("abc\\ndef"))
    }
}
#[test]
fn it_parses_interpolated_string_with_zero_char() {
    assert_eq! {Expression.parse("\"abc\\0def\""), (Str("abc\0def"))
    }
}
#[test]
fn it_parses_non_interpolated_string_with_zero_char() {
    assert_eq! {Expression.parse("'abc\\0def'"), (Str("abc\\0def"))
    }
}
#[test]
fn it_parses_interpolated_string_with_octal_char() {
    assert_eq! {Expression.parse("\"abc\\75def\""), (Str("abc=def"))
    }
}
#[test]
fn it_parses_interpolated_string_with_hex_unicode_char() {
    assert_eq! {Expression.parse("\"abc\\u21bbdef\""), (Str("abc\u{21bb}def"))
    }

    // Casts
}
#[test]
fn it_parses_123_as_u4() {
    assert_eq! {Expression.parse("123.as<u4>"), (
        CastToType(IntNum(123), typeId(false, Seq("u4")))
      )
    }
}
#[test]
fn it_parses_123_as_u4() {
    assert_eq! {Expression.parse("(123).as<u4>"), (
        CastToType(IntNum(123), typeId(false, Seq("u4")))
      )
    }
}
#[test]
fn it_parses_str_as_x() {
    assert_eq! {Expression.parse("\"str\".as<x>"), (
        CastToType(Str("str"), typeId(false, Seq("x")))
      )
    }
}
#[test]
fn it_parses_foo_as_x() {
    assert_eq! {Expression.parse("foo.as<x>"), (
        CastToType(Name(identifier("foo")), typeId(false, Seq("x")))
      )
    }
}
#[test]
fn it_parses_foo_as_x_with_spacing() {
    assert_eq! {Expression.parse("foo.as < x  >  "), (
        CastToType(Name(identifier("foo")), typeId(false, Seq("x")))
      )
    }
}
#[test]
fn it_parses_foo_as_bar_baz() {
    assert_eq! {Expression.parse("foo.as<bar::baz>"), (
        CastToType(Name(identifier("foo")), typeId(false, Seq("bar", "baz")))
      )
    }
}
#[test]
fn it_parses_foo_as_root_bar_baz() {
    assert_eq! {Expression.parse("foo.as<::bar::baz>"), (
        CastToType(Name(identifier("foo")), typeId(true, Seq("bar", "baz")))
      )
    }
}
#[test]
fn it_parses_foo_as_bar_array() {
    assert_eq! {Expression.parse("foo.as<bar[]>"), (
        CastToType(Name(identifier("foo")), typeId(false, Seq("bar"), true))
      )
    }
}
#[test]
fn it_parses_foo_as_bar_baz() {
    assert_eq! {Expression.parse("foo.as<::bar::baz[]>"), (
        CastToType(Name(identifier("foo")), typeId(true, Seq("bar", "baz"), true))
      )
    }
}
#[test]
fn it_parses_foo_as() {
    assert_eq! {Expression.parse("foo.as"), (Attribute(Name(identifier("foo")),identifier("as")))
    }
}
#[test]
fn it_parses_foo_as_less_than_x() {
    assert_eq! {Expression.parse("foo.as<x"), (
        Compare(
          Attribute(Name(identifier("foo")),identifier("as")),
          Lt,
          Name(identifier("x"))
        )
      )
    }

    // sizeof keyword
}
#[test]
fn it_parses_sizeof_foo() {
    assert_eq! {Expression.parse("sizeof<foo>"), (
        ByteSizeOfType(typeId(false, Seq("foo")))
      )
    }
}
#[test]
fn it_parses_sizeof_foo_bar() {
    assert_eq! {Expression.parse("sizeof<foo::bar>"), (
        ByteSizeOfType(typeId(false, Seq("foo", "bar")))
      )
    }
}
#[test]
fn it_parses_sizeof_foo_bar() {
    assert_eq! {Expression.parse("sizeof<::foo::bar>"), (
        ByteSizeOfType(typeId(true, Seq("foo", "bar")))
      )
    }
}
#[test]
fn it_parses_sizeof_less_than_foo() {
    assert_eq! {Expression.parse("sizeof<foo"), (
        Compare(
          Name(identifier("sizeof")),
          Lt,
          Name(identifier("foo"))
        )
      )
    }
}
#[test]
fn it_parses_bitsizeof_foo() {
    assert_eq! {Expression.parse("bitsizeof<foo>"), (
        BitSizeOfType(typeId(false, Seq("foo")))
      )
    }
}
#[test]
fn it_parses_bitsizeof_less_than_foo() {
    assert_eq! {Expression.parse("bitsizeof<foo"), (
        Compare(
          Name(identifier("bitsizeof")),
          Lt,
          Name(identifier("foo"))
        )
      )
    }

    // Attribute / method call
}
#[test]
fn it_parses_123_to_s() {
    assert_eq! {Expression.parse("123.to_s"), (Attribute(IntNum(123),identifier("to_s")))
    }
}
#[test]
fn it_parses_123_4_to_s() {
    assert_eq! {Expression.parse("123.4.to_s"), (Attribute(FloatNum(123.4),identifier("to_s")))
    }
}
#[test]
fn it_parses_foo_bar() {
    assert_eq! {Expression.parse("foo.bar"), (Attribute(Name(identifier("foo")),identifier("bar")))
    }
}
