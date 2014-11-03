//! Messages to and from the server
#![experimental]
use std::from_str::FromStr;

/// IRC Message data
#[experimental]
#[deriving(Clone, PartialEq, Show)]
pub struct Message {
    /// The message prefix (or source) as defined by [RFC 2812](http://tools.ietf.org/html/rfc2812)
    pub prefix: Option<String>,
    /// The IRC command as defined by [RFC 2812](http://tools.ietf.org/html/rfc2812)
    pub command: String,
    /// The command arguments
    pub args: Vec<String>,
    /// The message suffix as defined by [RFC 2812](http://tools.ietf.org/html/rfc2812)
    /// This is the only part of the message that is allowed to contain spaces.
    pub suffix: Option<String>,
}

impl Message {
    /// Creates a new Message
    #[experimental]
    pub fn new(prefix: Option<&str>, command: &str, args: Option<Vec<&str>>, suffix: Option<&str>)
        -> Message {
        Message {
            prefix: prefix.map(|s| s.into_string()),
            command: command.into_string(),
            args: args.map_or(Vec::new(), |v| v.iter().map(|s| s.into_string()).collect()),
            suffix: suffix.map(|s| s.into_string()),
        }
    }

    /// Converts a Message into a String according to the IRC protocol
    #[experimental]
    pub fn into_string(&self) -> String {
        let mut ret = String::new();
        if let Some(ref prefix) = self.prefix {
            ret.push(':');
            ret.push_str(prefix[]);
            ret.push(' ');
        }
        ret.push_str(self.command[]);
        for arg in self.args.iter() {
            ret.push(' ');
            ret.push_str(arg[]);
        }
        if let Some(ref suffix) = self.suffix {
            ret.push_str(" :");
            ret.push_str(suffix[]);
        }
        ret
    }
}

impl FromStr for Message {
    fn from_str(s: &str) -> Option<Message> {
        let mut state = s.clone();
        if s.len() == 0 { return None }
        let prefix = if state.starts_with(":") {
            let prefix = state.find(' ').map(|i| s[1..i]);
            state = state.find(' ').map_or("", |i| s[i..]);
            prefix
        } else {
            None
        };
        let suffix = if state.contains(":") {
            let suffix = state.find(':').map(|i| s[i..]);
            state = state.find(':').map_or("", |i| s[..i]);
            suffix
        } else {
            None
        };
        let command = if let Some(cmd) = state.find(' ').map(|i| s[..i]) {
            state = state.find(' ').map_or("", |i| s[i..]);
            cmd
        } else {
            return None
        };
        let args: Vec<_> = state.splitn(14, ' ').collect();
        Some(Message::new(prefix, command, if args.len() > 0 { Some(args) } else { None }, suffix))
    }
}
