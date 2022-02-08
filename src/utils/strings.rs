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

pub fn point_transition(name: impl std::string::ToString, before: i32, after: i32) -> String {
    format!("{}の所持ポイント：{}→{}", safe(&name.to_string()), before, after)
}