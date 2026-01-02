//! 权限相关实体

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 用户权限摘要（用于鉴权）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissions {
    pub user_id: Uuid,
    pub groups: Vec<super::Group>,
    pub permissions: Vec<String>, // 权限字符串列表，支持通配符如 "model.*.use"
}

impl UserPermissions {
    /// 检查是否拥有指定权限（支持通配符匹配）
    pub fn has_permission(&self, required: &str) -> bool {
        self.permissions.iter().any(|p| {
            // "*" 匹配所有
            if p == "*" {
                return true;
            }
            // 精确匹配
            if p == required {
                return true;
            }
            // 通配符匹配 (简单 glob)
            glob_match(p, required)
        })
    }
}

/// 简单 glob 匹配
/// 支持 * 匹配单段，** 匹配多段
fn glob_match(pattern: &str, text: &str) -> bool {
    let pattern_parts: Vec<&str> = pattern.split('.').collect();
    let text_parts: Vec<&str> = text.split('.').collect();

    glob_match_parts(&pattern_parts, &text_parts)
}

fn glob_match_parts(pattern: &[&str], text: &[&str]) -> bool {
    match (pattern.first(), text.first()) {
        (None, None) => true,
        (Some(&"**"), _) => {
            // ** 匹配零个或多个段
            glob_match_parts(&pattern[1..], text)
                || (!text.is_empty() && glob_match_parts(pattern, &text[1..]))
        }
        (Some(&"*"), Some(_)) => {
            // * 匹配单个段
            glob_match_parts(&pattern[1..], &text[1..])
        }
        (Some(p), Some(t)) if p == t => glob_match_parts(&pattern[1..], &text[1..]),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_match() {
        assert!(glob_match("*", "chat"));
        assert!(glob_match("chat.*", "chat.send"));
        assert!(glob_match("model.*.use", "model.gpt-4.use"));
        assert!(glob_match("model.**", "model.gpt-4.use"));
        assert!(glob_match("**", "a.b.c"));
        assert!(!glob_match("chat.*", "model.send"));
        assert!(!glob_match("model.gpt-4.use", "model.claude.use"));
    }
}
