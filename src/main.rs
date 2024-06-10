mod conf;
mod errors;
mod running;

use anyhow::Context;
use clap::Parser;
use clio::*;
use clap_logger::LevelFilter;
use rhai::{Engine, EvalAltResult};
use crate::errors::AutomatonError;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, name = "automaton")]
struct Args {
    /// Turn on the generic automation mode. Mainly affects terminology so that "test" is
    /// replaced with "task" in logs and reports. By default, the mode is got
    /// from test/task header in data files.
    #[arg(long)]
    rpa: Option<bool>,
    /// Activate localization. `lang` can be a name or a code of a built-in language,
    /// or a path or a module name of a custom language file.
    #[arg(long, value_name = "lang *")]
    language: Option<Vec<String>>,
    /// Parse only files with this extension when executing
    /// a directory. Has no effect when running individual
    /// files or when using resource files. If more than one
    /// extension is needed, separate them with a colon.
    /// Examples: `--extension txt`, `--extension robot:txt`
    #[arg(long, short = 'F', value_name = "value")]
    extension: Option<String>,
    /// Parse only files matching `pattern`. It can be:
    ///  - a file name or pattern like `example.robot` or
    ///  `*.robot` to parse all files matching that name,
    ///  - a file path like `path/to/example.robot`, or
    ///  - a directory path like `path/to/example` to parse
    ///    all files in that directory, recursively.
    #[arg(long, short = 'I', value_name = "pattern")]
    parse_include: Option<Vec<String>>,
    /// Set the name of the top level suite. By default, the name is created based on the executed
    /// file or directory.
    #[arg(long, short = 'N', value_name = "name")]
    name: Option<String>,
    /// Set the documentation of the top level suite. Simple formatting is supported (e.g. *bold*).
    /// If the documentation contains spaces, it must be quoted. If the value is path to an
    /// existing file, actual documentation is read from that file.
    /// Examples: --doc "Very *good* example"
    ///           --doc doc_from_file.txt
    #[arg(long, short = 'D', value_name = "documentation")]
    doc: Option<String>,
    /// Set metadata of the top level suite. Value can contain formatting and be read from a file
    /// similarly as --doc. Example: --metadata Version:1.2
    #[arg(long, short = 'M', value_name = "name:value *")]
    metadata: Option<Vec<String>>,
    /// Sets given tag(s) to all executed tests.
    #[arg(long, short = 'G', value_name = "tag *")]
    set_tag: Option<Vec<String>>,
    /// Select tests by name or by long name containing also parent suite name like `Parent.Test`.
    /// Name is case and space insensitive, and it can also be a simple pattern where `*` matches
    /// anything, `?` matches any single character, and `[chars]` matches one character in brackets.
    #[arg(long, short = 't', value_name = "name *")]
    test: Option<Vec<String>>,
    /// Alias to --test. Especially applicable with --rpa.
    #[arg(long, value_name = "name *")]
    task: Option<Vec<String>>,
    /// Select suites by name. When this option is used with
    /// --test, --include or --exclude, only tests in
    /// matching suites and also matching other filtering
    /// criteria are selected. Name can be a simple pattern
    /// similarly as with --test, and it can contain parent
    /// name separated with a dot. For example, `-s X.Y`
    /// selects suite `Y` only if its parent is `X`.
    #[arg(long, short = 's', value_name = "name *")]
    suite: Option<Vec<String>>,
    /// Select tests by tag. Similarly, as name with --test,
    /// tag is case and space insensitive, and it is possible
    /// to use patterns with `*`, `?` and `[]` as wildcards.
    /// Tags and patterns can also be combined with
    /// `AND`, `OR`, and `NOT` operators.
    /// Examples: --include foo --include bar* --include fooANDbar*
    #[arg(long, short = 'i', value_name = "tag *")]
    include: Option<Vec<String>>,
    /// Select test cases not to run by tag. These tests are
    /// not run even if included with --include. Tags are
    /// matched using same rules as with --include.
    #[arg(long, short = 'e', value_name = "tag *")]
    exclude: Option<Vec<String>>,
    /// Select failed tests from an earlier output file to be
    /// re-executed. Equivalent to selecting same tests
    /// individually using --test.
    #[arg(long, short = 'R', value_name = "output")]
    rerun_failed: Option<String>,
    /// Select failed suites from an earlier output file to be re-executed.
    #[arg(long, short = 'S', value_name = "output")]
    rerun_failed_suites: Option<String>,
    /// Executes suite even if it contains no tests. Useful
    /// e.g. with --include/--exclude when it is not an error
    /// that no test matches the condition.
    #[arg(long)]
    rerun_empty_suite: Option<bool>,
    /// Tests having given tag will be skipped. Tag can be a pattern.
    #[arg(long, value_name = "tag *")]
    skip: Option<Vec<String>>,
    /// Set variables in the test data. Only scalar
    /// variables with string value are supported and name is
    /// given without `${}`. See --variablefile for a more
    /// powerful variable setting mechanism.
    /// Examples:
    /// --variable name:Robot  =>  ${name} = `Robot`
    /// -v "hello:Hello world" =>  ${hello} = `Hello world`
    /// -v x: -v y:42          =>  ${x} = ``, ${y} = `42`
    #[arg(long, short, value_name = "name:value")]
    variable: Option<Vec<String>>,
    /// Python or YAML file file to read variables from.
    /// Possible arguments to the variable file can be given
    /// after the path using colon or semicolon as separator.    ///
    /// Examples: --variablefile path/vars.yaml
    /// --variablefile environment.py:testing
    #[arg(long, value_name = "path *")]
    variable_file: Option<Vec<Input>>,
    /// Where to create output files. The default is the
    /// directory where tests are run from and the given path
    /// is considered relative to that unless it is absolute.
    #[arg(long, short = 'd', value_name = "dir")]
    output_fir: Option<ClioPath>,
    /// XML output file. Given path, similarly as paths given
    /// to --log, --report, --xunit, and --debugfile, is
    /// relative to --outputdir unless given as an absolute
    /// path. Other output files are created based on XML
    /// output files after the test execution and XML outputs
    /// can also be further processed with Rebot tool. Can be
    /// isabled by giving a special value `NONE`.
    /// Default: output.xml
    #[arg(long, short = 'o', value_name = "file")]
    output: Option<Output>,
    /// HTML log file. Default: log.html
    #[arg(long, short = 'l', value_name = "file")]
    log: Option<Output>,
    /// HTML report file. similarly as --log. Default: report.html
    #[arg(long, short = 'r', value_name = "file")]
    report: Option<Output>,
    /// Threshold level for logging. Available levels: TRACE,
    /// DEBUG, INFO (default), WARN, NONE (no logging). Use
    /// syntax `LOGLEVEL:DEFAULT` to define the default
    /// visible log level in log files.
    /// Examples: --loglevel DEBUG, --loglevel DEBUG:INFO
    #[arg(long, short = 'L', value_name = "level")]
    log_level: Option<LevelFilter>,
    /// Text file to read more arguments from. Use special path `STDIN` to read contents from the standard input
    /// stream. File can have both options and input files or directories, one per line. Contents do not need to
    /// be escaped but spaces in the beginning and end of lines are removed. Empty lines and lines starting
    /// with a hash character (#) are ignored.
    #[arg(long, short = 'A', value_name = "path *")]
    argument_file: Option<Vec<Input>>,
    #[clap(value_parser)]
    paths: Vec<Input>
}

fn main() -> anyhow::Result<()>{
    let args = Args::parse();
    log::trace!("{:?}", args);
    let engine = Engine::new();
    let result = engine.eval::<i64>("40 + 2").expect("Could not convert Rhai script");
    println!("Answer: {result}");
    Ok(())
}
