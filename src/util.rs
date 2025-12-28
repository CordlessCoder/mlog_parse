#[macro_export]
macro_rules! const_regex {
    ($name:ident, $re:literal $(,)?) => {
        static $name: ::std::sync::OnceLock<regex::Regex> = ::std::sync::OnceLock::new();
        $name = $name.get_or_init(|| regex::Regex::new($re).unwrap())
    }
}