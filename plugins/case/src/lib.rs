use plugin_core::PluginOption;

#[derive(Debug, Clone)]
pub enum Case {
    Lower,
    Upper,
    Toggle,
}

#[derive(Debug)]
pub struct CasePluginOption {
    pub case: Case,
}

impl PluginOption for CasePluginOption {}

impl Default for CasePluginOption {
    fn default() -> Self {
        CasePluginOption { case: Case::Lower }
    }
}

#[no_mangle]
pub fn transform<'a>(s: String, option: &'a CasePluginOption) -> String {
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
            transform(
                String::from("HelLo, WorLd!"),
                &CasePluginOption { case: Case::Lower }
            ),
            String::from("hello, world!")
        );

        assert_eq!(
            transform(
                String::from("안녕하세요 こんにちは 你好"),
                &CasePluginOption { case: Case::Lower }
            ),
            String::from("안녕하세요 こんにちは 你好")
        );

        assert_eq!(
            transform(String::from("Grüße, Jürgen ❤ 😇"), {
                &CasePluginOption { case: Case::Lower }
            }),
            String::from("grüße, jürgen ❤ 😇")
        );
    }

    #[test]
    fn test_upper() {
        assert_eq!(
            transform(
                String::from("HelLo, WorLd!"),
                &CasePluginOption { case: Case::Upper }
            ),
            String::from("HELLO, WORLD!")
        );

        assert_eq!(
            transform(
                String::from("안녕하세요 こんにちは 你好"),
                &CasePluginOption { case: Case::Upper }
            ),
            String::from("안녕하세요 こんにちは 你好")
        );

        assert_eq!(
            transform(String::from("Grüße, Jürgen ❤ 😇"), {
                &CasePluginOption { case: Case::Upper }
            }),
            String::from("GRÜSSE, JÜRGEN ❤ 😇")
        );
    }

    #[test]
    fn test_toggle() {
        assert_eq!(
            transform(
                String::from("HelLo, WorLd!"),
                &CasePluginOption { case: Case::Toggle }
            ),
            String::from("hELlO, wORlD!")
        );

        assert_eq!(
            transform(
                String::from("안녕하세요 こんにちは 你好"),
                &CasePluginOption { case: Case::Toggle }
            ),
            String::from("안녕하세요 こんにちは 你好")
        );

        assert_eq!(
            transform(String::from("Grüße, Jürgen ❤ 😇"), {
                &CasePluginOption { case: Case::Toggle }
            }),
            String::from("gRÜSE, jÜRGEN ❤ 😇")
        );
    }
}
