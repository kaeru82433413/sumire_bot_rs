macro_rules! chmin {
    ($a:expr, $b:expr) => {
        $a = $a.min($b)
    };
}

pub fn optimal_damerau_levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<_> = a.chars().collect();
    let b: Vec<_> = b.chars().collect();
    let (n, m) = (a.len(), b.len());

    let mut dp = vec![vec![n.max(m); m+1]; n+1];

    for i in 0..=n {
        for j in 0..=m {
            if a[i] == b[j] {
                chmin!(dp[i][j], dp[i-1][j-1]);
            }
            chmin!(dp[i][j], dp[i-1][j]+1);
            chmin!(dp[i][j], dp[i-1][j]+1);
            if i>=2 && j>=2 && (a[i-1], a[i-2]) == (b[i-2], b[i-1]) {
                chmin!(dp[i][j], dp[i-2][j-2]+1);
            }
        }
    }
    dp[n][m]
}


pub fn safe(raw: &str) -> String {
    let mut escaped = vec![];
    let mut is_escaped = true;
    for c in raw.chars() {
        if c == '.' && !is_escaped {
            escaped.push('\\');
        }
        escaped.push(c);
        if c == '\\' {
            is_escaped ^= true;
        } else {
            is_escaped = false;
        }
    }
    let escaped: String = escaped.iter().collect();
    escaped.replace("@everyone", "@\u{200b}everyone").replace("@here", "@\u{200b}here")
}

use serenity::model::channel::Message;
pub fn display_name(msg: &Message) -> String {
    if let Some(member) = &msg.member {
        if let Some(nick) = &member.nick {
            return nick.clone()
        }
    }
    msg.author.name.clone()
}


use serenity::model::guild::Member;

pub struct PointTransition {
    name: String,
    before: Option<i32>,
    after: Option<i32>,
    increase: Option<i32>,
}

impl PointTransition {
    pub fn new(member: &Member) -> Self {
        Self::name(&member.display_name())
    }
    pub fn name(raw_name: &str) -> Self {
        Self {
            name: safe(raw_name),
            before: None, after: None, increase: None,
        }
    }

    pub fn before(mut self, value: i32) -> Self {
        self.before = Some(value);
        self
    }
    pub fn after(mut self, value: i32) -> Self {
        self.after = Some(value);
        self
    }
    pub fn increase(mut self, value: i32) -> Self {
        self.increase = Some(value);
        self
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for PointTransition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let before = match self.before {
            Some(value) => value,
            None => self.after.unwrap() - self.increase.unwrap(),
        };
        let after = match self.after {
            Some(value) => value,
            None => self.before.unwrap() + self.increase.unwrap(),
        };
        write!(f, "{}の所持コイン：{}→{}", self.name, before, after)?;
        Ok(())
    }
}