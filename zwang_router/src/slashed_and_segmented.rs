/// Newtype where the contained string is guaranteed to start with a slash.
#[derive(Clone)]
pub struct Slashed<'a>(&'a str);

impl<'a> Slashed<'a> {
    pub fn new(p: &'a str) -> Result<Slashed<'a>, String> {
        if p.chars().next() != Some('/') {
            Err("first char not /".into())
        } else {
            Ok(Self(p))
        }
    }
}

impl<'a> std::fmt::Display for Slashed<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl<'a> Slashed<'a> {
    pub fn non_slash_len(&self) -> usize {
        self.0.len() - 1
    }
}

/// New type where the contained str is guaranteed to start with a slash and no other slashes are
/// contained there after.
pub struct PathSegment<'a>(&'a str);

impl<'a> PathSegment<'a> {
    pub fn non_slash(&self) -> &str {
        &self.0[1..]
    }
}

#[derive(Debug, PartialEq)]
pub struct DoesNotStartWithSlashError;

/// Will error out if first char is not a slash.
/// split_to_two_at_non_initial_slash("/hi/hello/asdf") == Ok((PathSegment("/hi"), Slashed("/hello/asdf"))).
/// split_to_two_at_non_initial_slash("hi/hello/asdf") == Err(DoesNotStartWithSlashError).
/// split_to_two_at_non_initial_slash("/hi") == (PathSegment("/hi"), Slashed("/")).
/// split_to_two_at_non_initial_slash("hi") == Err(DoesNotStartWithSlashError)).
/// split_to_two_at_non_initial_slash("/") == (PathSegment("/"), Slashed("/"))
pub fn split_path(path: &str) -> Result<(PathSegment, Slashed), DoesNotStartWithSlashError> {
    let mut chars = path.chars().enumerate();
    if chars.next().map(|p| p.1) != Some('/') {
        return Err(DoesNotStartWithSlashError);
    }
    let slash_2_idx = chars.find(|(_, i)| *i == '/').map(|p| p.0);

    Ok((
        PathSegment(match slash_2_idx {
            Some(idx) => &path[..idx],
            None => path,
        }),
        Slashed(match slash_2_idx {
            Some(idx) => &path[idx..],
            None => "/",
        }),
    ))
}

pub fn split_slashed(slashed: Slashed) -> (PathSegment, Slashed) {
    split_path(slashed.0).expect("should not give us DoesNotStartWithSlashError because Slashed is guaranteed to start with a slash")
}

