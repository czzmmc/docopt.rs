use std::prelude::v1::*;
use std::collections::HashMap;

use serde::Deserialize;

use docopt::{Docopt, ArgvMap, Error};
use docopt::Value::{self, Switch, Plain};

pub fn get_args(doc: &str, argv: &[&'static str]) -> ArgvMap {
    let dopt = match Docopt::new(doc) {
        Err(err) => panic!("Invalid usage: {}", err),
        Ok(dopt) => dopt,
    };
    match dopt.argv(vec!["cmd"].iter().chain(argv.iter())).parse() {
        Err(err) => panic!("{}", err),
        Ok(vals) => vals,
    }
}

pub fn map_from_alist(alist: Vec<(&'static str, Value)>)
                 -> HashMap<String, Value> {
    alist.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}

pub fn same_args(expected: &HashMap<String, Value>, got: &ArgvMap) {
    for (k, ve) in expected.iter() {
        match got.map.find(k) {
            None => panic!("EXPECTED has '{}' but GOT does not.", k),
            Some(vg) => {
                assert!(ve == vg,
                        "{}: EXPECTED = '{:?}' != '{:?}' = GOT", k, ve, vg)
            }
        }
    }
    for (k, vg) in got.map.iter() {
        match got.map.find(k) {
            None => panic!("GOT has '{}' but EXPECTED does not.", k),
            Some(ve) => {
                assert!(vg == ve,
                        "{}: GOT = '{:?}' != '{:?}' = EXPECTED", k, vg, ve)
            }
        }
    }
}

macro_rules! test_expect(
    ($name:ident, $doc:expr, $args:expr, $expected:expr) => (
        #[test]
        fn $name() {
            let vals = get_args($doc, $args);
            let expected = map_from_alist($expected);
            same_args(&expected, &vals);
        }
    );
);

macro_rules! test_user_error(
    ($name:ident, $doc:expr, $args:expr) => (
        #[test]
        #[should_panic]
        fn $name() { get_args($doc, $args); }
    );
);

test_expect!(test_issue_13, "Usage: prog file <file>", &["file", "file"],
             vec![("file", Switch(true)),
                  ("<file>", Plain(Some("file".to_string())))]);

test_expect!(test_issue_129, "Usage: prog [options]

Options:
    --foo ARG   Foo foo.",
             &["--foo=a b"],
             vec![("--foo", Plain(Some("a b".into())))]);


pub fn regression_issue_12() {
    const USAGE: &'static str = "
    Usage:
        whisper rows <file>
        whisper update <file> <timestamp> <value>
        whisper mark <file> <value>
    ";

    #[derive(Deserialize, Debug)]
    struct Args {
        arg_file: String,
        cmd_rows: bool,
        cmd_update: bool,
        arg_timestamp: u64,
        arg_value: f64,
    }

    let dopt: Args = Docopt::new(USAGE)
        .unwrap()
        .argv(&["whisper", "mark", "./p/blah", "100"])
        .deserialize()
        .unwrap();
    assert_eq!(dopt.arg_file, "./p/blah");
}

pub fn regression_issue_195() {
    const USAGE: &'static str = "
    Usage:
        slow [-abcdefghijklmnopqrs...]
    ";

    let argv = &["slow", "-abcdefghijklmnopqrs"];
    let dopt : Docopt = Docopt::new(USAGE).unwrap().argv(argv);

  let tt = dopt.parse().unwrap();
}

pub fn regression_issue_xsv() {
    const USAGE: &'static str = "
    Concatenates CSV data by column or by row.

When concatenating by column, the columns will be written in the same order as
the inputs given. The number of rows in the result is always equivalent to to
the minimum number of rows across all given CSV data. (This behavior can be
reversed with the '--pad' flag.)

When concatenating by row, all CSV data must have the same number of columns.
If you need to rearrange the columns or fix the lengths of records, use the
'select' or 'fixlengths' commands. Also, only the headers of the *first* CSV
data given are used. Headers in subsequent inputs are ignored. (This behavior
can be disabled with --no-headers.)

Usage:
    cat rows    [options] [<input>...]
    cat columns [options] [<input>...]
    cat --help

cat options:
    -p, --pad              When concatenating columns, this flag will cause
                           all records to appear. It will pad each row if
                           other CSV data isn't long enough.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. Note that this has no effect when
                           concatenating columns.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    ";

    let argv = &["cat","rows","/home/carrie/projects/rust/xsv/test.csv","/home/carrie/projects/rust/xsv/test1.csv","-o","/home/carrie/projects/rust/xsv/123"];
    let dopt : Docopt = Docopt::new(USAGE).unwrap();
}

pub fn regression_issue_219() {
    #[derive(Deserialize)]
    struct Args {
        arg_type: Vec<String>,
        arg_param: Vec<String>,
    }

    const USAGE: &'static str = "
    Usage:
        encode [-v <type> <param>]...
    ";

    let argv = &["encode", "-v", "bool", "true","string", "foo"];
    let args: Args = Docopt::new(USAGE).unwrap().argv(argv).deserialize().unwrap();
    assert_eq!(args.arg_type, vec!["bool".to_owned(), "string".to_owned()]);
    assert_eq!(args.arg_param, vec!["true".to_owned(), "foo".to_owned()]);
}

pub fn test_unit_struct() {
    const USAGE: &'static str = "
    Usage:
        cargo version [options]

    Options:
        -h, --help               Print this message
    ";

    #[derive(Deserialize)]
    struct Options;

    let argv = &["cargo", "version"];
    let dopt: Result<Options, Error>= Docopt::new(USAGE)
        .unwrap()
        .argv(argv)
        .deserialize();
    assert!(dopt.is_ok());
}
