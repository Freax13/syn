use nom::IResult;
use unicode_xid::UnicodeXID;

pub fn whitespace(input: &str) -> IResult<&str, ()> {
    if input.is_empty() {
        return IResult::Error;
    }

    let mut start = 0;
    let mut chars = input.char_indices();
    while let Some((i, ch)) = chars.next() {
        let s = &input[start + i..];
        if ch == '/' {
            if s.starts_with("//") && (!s.starts_with("///") || s.starts_with("////")) &&
               !s.starts_with("//!") {
                if let Some(len) = s.find('\n') {
                    start += i + len + 1;
                    chars = input[start..].char_indices();
                    continue;
                }
                break;
            } else if s.starts_with("/*") && !s.starts_with("/**") && !s.starts_with("/*!") {
                match block_comment(s) {
                    IResult::Done(_, com) => {
                        start += i + com.len();
                        chars = input[start..].char_indices();
                        continue;
                    }
                    IResult::Error => {
                        return IResult::Error;
                    }
                }
            }
        }
        if !ch.is_whitespace() {
            return if start + i > 0 {
                IResult::Done(s, ())
            } else {
                IResult::Error
            };
        }
    }
    IResult::Done("", ())
}

pub fn block_comment(input: &str) -> IResult<&str, &str> {
    if !input.starts_with("/*") {
        return IResult::Error;
    }

    let mut depth = 0;
    let bytes = input.as_bytes();
    let mut i = 0;
    let upper = bytes.len() - 1;
    while i < upper {
        if bytes[i] == b'/' && bytes[i + 1] == b'*' {
            depth += 1;
            i += 1; // eat '*'
        } else if bytes[i] == b'*' && bytes[i + 1] == b'/' {
            depth -= 1;
            if depth == 0 {
                return IResult::Done(&input[i + 2..], &input[..i + 2]);
            }
            i += 1; // eat '/'
        }
        i += 1;
    }
    IResult::Error
}

pub fn word_break(input: &str) -> IResult<&str, ()> {
    match input.chars().next() {
        Some(ch) if UnicodeXID::is_xid_continue(ch) => IResult::Error,
        Some(_) | None => IResult::Done(input, ()),
    }
}
