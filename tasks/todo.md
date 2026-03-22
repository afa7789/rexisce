# xmpp-start Orchestration TODO

## Completed
- ✅ **B1**: rustls CryptoProvider — wired in main.rs
- ✅ **B2**: i18n module — stub exists as src/i18n/mod.rs
- ✅ **B3+A4**: Presence indicator — sidebar.rs shows ●/○, mod.rs calls on_presence
- ✅ **A1**: SQLite DB at startup — wired in main.rs + App struct
- ✅ **A2**: Persist incoming messages to SQLite (2026-03-22)
- ✅ **A3**: Persist roster to SQLite on RosterReceived (2026-03-22)
- ✅ **A5**: Notifications — wired in ui/mod.rs
- ✅ **C6**: XEP-0280 Carbons — incoming stanza wiring (2026-03-22)
- ✅ **E3**: Emoji reactions (XEP-0444)
- ✅ **E5**: Link previews (2026-03-22)
- ✅ **F1**: XMPP debug console
- ✅ **F2**: Command palette (Cmd+K)
- ✅ **G6**: Draft auto-save per conversation
- ✅ **J2**: Custom status message
- ✅ **J4**: Sound notifications

## Phase B — Storage Layer
- [x] ✅ **B4**: Load message history on conversation open (50 most recent) — (2026-03-22)
- [x] ✅ **B5**: Unread badge count in sidebar — (2026-03-22)
- [ ] **B6**: Mark conversation read, persist last_read_id — depends on A2, B5

## Phase C — XMPP Engine Wiring
- [ ] **C1**: Wire StreamMgmt into engine loop
- [ ] **C2**: Wire PresenceMachine into engine
- [ ] **C3**: MAM post-connect history sync (depends on A1, A2)
- [ ] **C4**: Wire BlockingManager into engine
- [ ] **C5**: Wire DiscoManager / caps into presence

## Phase D — UI Panels
- [ ] **D1**: Render OccupantPanel in MUC conversations
- [ ] **D2**: XEP-0393 message styling (bold/italic/code in ConversationView)
- [ ] **D3**: MUC join/leave UI flow
- [ ] **D4**: Bookmarks autojoin on connect

## Phase E — Rich Features
- [ ] **E1**: Message corrections (XEP-0308)
- [ ] **E2**: Message retractions (XEP-0424)
- [ ] **E4**: File upload (XEP-0363)

## Phase F — Polish
- [ ] **F3**: Settings panel (font size, timestamps, theme toggle)
- [ ] **F4**: Reconnect logic with backoff
- [ ] **F5**: Avatar fetching (XEP-0084 + vCard fallback)

## Phase G — Conversation UX
- [ ] **G1**: Close/remove conversation
- [ ] **G2**: Typing indicators (XEP-0085)
- [ ] **G3**: Message replies (XEP-0461)
- [ ] **G4**: /me action messages (XEP-0245)
- [ ] **G5**: Message grouping + date separators
- [ ] **G7**: Copy message to clipboard
- [ ] **G8**: MAM lazy-load (scroll up for older history)
- [ ] **G9**: Message search within conversation

## Phase H — Avatars & Contact Management
- [ ] **H1**: Show user avatars (XEP-0084 + XEP-0153)
- [ ] **H2**: Own avatar upload (XEP-0084)
- [ ] **H3**: Add/remove/rename contacts
- [ ] **H4**: Contact profile popover (vCard)
- [ ] **H5**: Consistent avatar colors (XEP-0392)

## Phase I — File & Media
- [ ] **I1**: Paste image from clipboard
- [ ] **I2**: Drag & drop files onto composer
- [ ] **I3**: File picker + multiple attachments + upload progress
- [ ] **I4**: Attachment preview in received messages

---
## Orchestration Notes
- NO worktree isolation — agents work directly in main repo on non-overlapping files
- Agent A (Storage/UI): B4 → B5 → B6 → D2 → G-phase (touches ui/, store/)
- Agent B (Engine/XMPP): C1 → C2 → C3 → C4 → C5 → D3 (touches xmpp/engine.rs)
- Always run `cargo test && cargo clippy` before marking complete
- Commit after each completed task, never push
