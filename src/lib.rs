pub use pretty;
use pretty::RcDoc;

pub trait PrettySimple<'a>: Sized {
    fn pprint(self) -> RcDoc<'a>;

    fn append<P: PrettySimple<'a>>(self, other: P) -> RcDoc<'a> {
        self.pprint().append(other.pprint())
    }
}

impl<'a> PrettySimple<'a> for &'a str {
    fn pprint(self) -> RcDoc<'a> {
        RcDoc::text(self)
    }
}

impl<'a> PrettySimple<'a> for RcDoc<'a> {
    fn pprint(self) -> RcDoc<'a> {
        self
    }
}

pub fn surrounded_by<'a, P: PrettySimple<'a>>(l: &'a str, x: P, r: &'a str) -> RcDoc<'a> {
    RcDoc::text(l)
        .append(RcDoc::softline())
        .append(x.pprint().group())
        .append(RcDoc::softline())
        .append(r)
}

pub fn brackets<'a, P: PrettySimple<'a>>(x: P) -> RcDoc<'a> {
    surrounded_by("[", x, "]")
}

pub fn parens<'a, P: PrettySimple<'a>>(x: P) -> RcDoc<'a> {
    surrounded_by("(", x, ")")
}

pub fn braces<'a, P: PrettySimple<'a>>(x: P) -> RcDoc<'a> {
    surrounded_by("{", x, "}")
}

pub fn binop<'a, P: PrettySimple<'a>, Q: PrettySimple<'a>>(l: P, op: &'a str, r: Q) -> RcDoc<'a> {
    l.pprint()
        .append(RcDoc::line())
        .append(op)
        .append(RcDoc::line())
        .append(r.pprint())
        .group()
}

pub fn intersperse<'a, P: PrettySimple<'a> + 'a, I: Iterator<Item = P>, Q: PrettySimple<'a>>(
    items: I,
    separator: Q,
) -> RcDoc<'a> {
    RcDoc::intersperse(items.map(|i| i.pprint()), separator.pprint())
}

pub fn concat<'a, P: PrettySimple<'a> + 'a, I: Iterator<Item = P>>(items: I) -> RcDoc<'a> {
    RcDoc::concat(items.map(|i| i.pprint()))
}

pub fn tuple<'a, P: PrettySimple<'a> + 'a, I: Iterator<Item = P>>(items: I) -> RcDoc<'a> {
    braces(intersperse(items, ",".append(RcDoc::line())).group())
}

pub fn t<'a, S: Into<String>>(s: S) -> RcDoc<'a> {
    RcDoc::text(s.into())
}

#[macro_export]
macro_rules! doc {
    (@fold $acc:tt) => {
        $acc
    };
    (@fold $acc:tt $head:tt $($tail:tt)*) => {
        doc!(@fold {$acc.append($head)} $($tail)*)
    };
    ($($arg:expr),+) => {{
        doc!(@fold $({$arg.pprint()})*)
    }}
}

#[macro_export]
macro_rules! docf {
    ($s:literal) => {
        $crate::pretty::RcDoc::text(format!($s))
    };
    ($s:literal, $($arg:expr),+) => {
        $crate::pretty::RcDoc::text(format!($s, $($arg),+))
    };
}

pub struct Ln;

impl<'a> PrettySimple<'a> for Ln {
    fn pprint(self) -> RcDoc<'a> {
        RcDoc::line()
    }
}
