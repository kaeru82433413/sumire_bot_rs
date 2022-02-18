use serenity::model::id::{ChannelId, GuildId};


pub const IS_DEBUG: bool = cfg!(debug_assertions);
pub const SUMIRE_GUILD: GuildId = GuildId(504299765379366912);
pub const REGULAR_NOTIFICATION_CHANNEL: ChannelId = ChannelId(820939592999108648);
pub const LOGIN_NOTIFICATION_CHANNEL: ChannelId = ChannelId(769174714538786847);
pub const ERROR_LOG_CHANNEL: ChannelId = ChannelId(782423473569660969);
pub const WHITESPACES: &[u32] = &[
    // https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt
    0x0009, 0x000A, 0x000B, 0x000C, 0x000D,
    0x0020, 0x0085, 0x00A0, 0x1680,
    0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005,
    0x2006, 0x2007, 0x2008, 0x2009, 0x200A,
    0x2028, 0x2029, 0x202F, 0x205F, 0x3000
];
