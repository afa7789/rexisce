// Task P3.4 — XEP-0048 Room Bookmarks
// XEP reference: https://xmpp.org/extensions/xep-0048.html
//
// This is a pure data module — no I/O, no async.
// Supports private XML storage (XEP-0048 §3.2).

use tokio_xmpp::minidom::Element;

const NS_BOOKMARKS: &str = "storage:bookmarks";
const NS_PRIVATE: &str = "jabber:iq:private";

/// A single XMPP MUC room bookmark (XEP-0048).
#[derive(Debug, Clone, PartialEq)]
pub struct Bookmark {
    /// MUC room JID (e.g. `room@conference.example.org`).
    pub jid: String,
    /// Human-readable room name.
    pub name: Option<String>,
    /// Join this room automatically on login.
    pub autojoin: bool,
    /// Preferred nickname inside the room.
    pub nick: Option<String>,
    /// Room password, if required.
    pub password: Option<String>,
}

/// Manages the local set of room bookmarks and builds/parses the XEP-0048
/// private XML storage stanzas.
pub struct BookmarkManager {
    bookmarks: Vec<Bookmark>,
}

impl BookmarkManager {
    /// Creates a new, empty manager.
    pub fn new() -> Self {
        Self {
            bookmarks: Vec::new(),
        }
    }

    /// Replaces the entire bookmark list with `bookmarks`.
    pub fn set_bookmarks(&mut self, bookmarks: Vec<Bookmark>) {
        self.bookmarks = bookmarks;
    }

    /// Parses bookmarks from a `<storage xmlns='storage:bookmarks'>` element.
    ///
    /// Tolerates missing optional attributes/children gracefully.
    pub fn parse_bookmarks_from_iq(el: &Element) -> Vec<Bookmark> {
        // Accept either a bare <storage> element or a full <iq> wrapping
        // <query><storage>…</storage></query>.
        let storage = if el.name() == "storage" && el.ns() == NS_BOOKMARKS {
            el
        } else if let Some(query) = el.get_child("query", NS_PRIVATE) {
            match query.get_child("storage", NS_BOOKMARKS) {
                Some(s) => s,
                None => return Vec::new(),
            }
        } else {
            return Vec::new();
        };

        storage
            .children()
            .filter(|child| child.name() == "conference")
            .map(|conf| {
                let jid = conf.attr("jid").unwrap_or("").to_string();
                let name = conf.attr("name").map(std::string::ToString::to_string);
                let autojoin = matches!(conf.attr("autojoin"), Some("true") | Some("1"));
                let nick = conf
                    .get_child("nick", NS_BOOKMARKS)
                    .map(tokio_xmpp::minidom::Element::text);
                let password = conf
                    .get_child("password", NS_BOOKMARKS)
                    .map(tokio_xmpp::minidom::Element::text);

                Bookmark {
                    jid,
                    name,
                    autojoin,
                    nick,
                    password,
                }
            })
            .collect()
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // 1. parse_bookmarks_from_iq parses <conference> elements correctly
    #[test]
    fn parse_bookmarks_from_iq_parses_conference() {
        // Build a <storage> element directly to feed the parser.
        let mut storage = Element::builder("storage", NS_BOOKMARKS).build();

        let mut conf1 = Element::builder("conference", NS_BOOKMARKS)
            .attr("jid", "room1@server")
            .attr("name", "Room One")
            .attr("autojoin", "true")
            .build();
        let mut nick_el = Element::builder("nick", NS_BOOKMARKS).build();
        nick_el.append_text_node("Alice");
        conf1.append_child(nick_el);
        storage.append_child(conf1);

        let conf2 = Element::builder("conference", NS_BOOKMARKS)
            .attr("jid", "room2@server")
            .attr("autojoin", "1")
            .build();
        storage.append_child(conf2);

        let conf3 = Element::builder("conference", NS_BOOKMARKS)
            .attr("jid", "room3@server")
            .build();
        storage.append_child(conf3);

        let bookmarks = BookmarkManager::parse_bookmarks_from_iq(&storage);
        assert_eq!(bookmarks.len(), 3);

        let b1 = &bookmarks[0];
        assert_eq!(b1.jid, "room1@server");
        assert_eq!(b1.name, Some("Room One".to_string()));
        assert!(b1.autojoin);
        assert_eq!(b1.nick, Some("Alice".to_string()));

        let b2 = &bookmarks[1];
        assert_eq!(b2.jid, "room2@server");
        assert!(b2.autojoin, "autojoin='1' should parse as true");

        let b3 = &bookmarks[2];
        assert_eq!(b3.jid, "room3@server");
        assert!(!b3.autojoin);
        assert!(b3.nick.is_none());
    }
}
