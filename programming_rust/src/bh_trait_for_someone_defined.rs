// 既存の型に独自トレイトを実装

// Stringに対して便利なメソッドを追加
trait StringExt {
    fn is_email(&self) -> bool;
    fn truncate_with_ellipsis(&self, max_len: usize) -> String;
}

impl StringExt for String {
    fn is_email(&self) -> bool {
        self.contains('@') && self.contains('.') && self.len() > 3
    }

    fn truncate_with_ellipsis(&self, max_len: usize) -> String {
        if self.len() <= max_len {
            self.clone()
        } else {
            format!("{}...", &self[..max_len.saturating_sub(3)])
        }
    }
}

// &strにも実装
impl StringExt for &str {
    fn is_email(&self) -> bool {
        self.contains('@') && self.contains('.') && self.len() > 3
    }

    fn truncate_with_ellipsis(&self, max_len: usize) -> String {
        if self.len() <= max_len {
            self.to_string()
        } else {
            format!("{}...", &self[..max_len.saturating_sub(3)])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_email() {
        let email = "user@example.com".to_string();
        assert!(email.is_email());

        let not_email = "invalid".to_string();
        assert!(!not_email.is_email());

        // &strでも使える
        assert!("test@mail.co.jp".is_email());
        assert!(!"no-at-sign".is_email());
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        let long_text = "This is a very long text".to_string();
        assert_eq!(long_text.truncate_with_ellipsis(10), "This is...");

        let short_text = "Short".to_string();
        assert_eq!(short_text.truncate_with_ellipsis(10), "Short");

        // &strでも使える
        assert_eq!("Hello, World!".truncate_with_ellipsis(8), "Hello...");
    }
}
