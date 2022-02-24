use plugin_core::Plugin;

#[derive(Debug, Clone)]
pub enum Case {
    Lower,
    Upper,
    Toggle,
}

#[derive(Debug)]
pub struct CasePlugin {
    pub case: Case,
}

impl Plugin for CasePlugin {
    fn name(&self) -> &str {
        option_env!("CARGO_CRATE_NAME").expect("Failed to get crate name")
    }
}

impl Default for CasePlugin {
    fn default() -> Self {
        CasePlugin { case: Case::Lower }
    }
}

#[no_mangle]
pub fn transform(s: String, option: &'_ CasePlugin) -> String {
    match option.case {
        Case::Lower => s.to_lowercase(),
        Case::Upper => s.to_uppercase(),
        Case::Toggle => s
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().next()
                } else if c.is_lowercase() {
                    c.to_uppercase().next()
                } else {
                    Some(c)
                }
            })
            .collect::<Option<String>>()
            .expect("Failed to transform"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lower() {
        assert_eq!(
            transform(String::from("HelLo, WorLd!"), &CasePlugin::default()),
            String::from("hello, world!")
        );

        assert_eq!(
            transform(
                String::from("안녕하세요 こんにちは 你好"),
                &CasePlugin { case: Case::Lower }
            ),
            String::from("안녕하세요 こんにちは 你好")
        );

        assert_eq!(
            transform(String::from("Grüße, Jürgen ❤ 😇"), &CasePlugin::default()),
            String::from("grüße, jürgen ❤ 😇")
        );
    }

    #[test]
    fn test_upper() {
        assert_eq!(
            transform(
                String::from("HelLo, WorLd!"),
                &CasePlugin { case: Case::Upper }
            ),
            String::from("HELLO, WORLD!")
        );

        assert_eq!(
            transform(
                String::from("안녕하세요 こんにちは 你好"),
                &CasePlugin { case: Case::Upper }
            ),
            String::from("안녕하세요 こんにちは 你好")
        );

        assert_eq!(
            transform(String::from("Grüße, Jürgen ❤ 😇"), {
                &CasePlugin { case: Case::Upper }
            }),
            String::from("GRÜSSE, JÜRGEN ❤ 😇")
        );
    }

    #[test]
    fn test_toggle() {
        assert_eq!(
            transform(
                String::from("HelLo, WorLd!"),
                &CasePlugin { case: Case::Toggle }
            ),
            String::from("hELlO, wORlD!")
        );

        assert_eq!(
            transform(
                String::from("안녕하세요 こんにちは 你好"),
                &CasePlugin { case: Case::Toggle }
            ),
            String::from("안녕하세요 こんにちは 你好")
        );

        assert_eq!(
            transform(String::from("Grüße, Jürgen ❤ 😇"), {
                &CasePlugin { case: Case::Toggle }
            }),
            String::from("gRÜSE, jÜRGEN ❤ 😇")
        );
    }
}
