# ReXisCe — Strategic Roadmap & Project Status

> **Snapshot:** 2026-03-29 | **Build:** 1026 tests ✅ | **Debt:** 95 `#[allow(dead_code)]`
> **Vision:** A premium, privacy-first XMPP client with vertical sidebar navigation and verified OMEMO-E2EE.

---

## 🗺️ Development Milestones

### 🟢 Milestone 1: UI/UX Pivot (The "Photo 1" Style)
**Goal:** Restore the vertical sidebar navigation requested by the user, while preserving the functional richness of the recent horizontal tab sprint.

- [ ] **`ui-sidebar-pivot`**: Revert from horizontal tabs to vertical sidebar (~160px width) in Settings.
- [ ] **`ui-settings-modular`**: Refactor `settings.rs` into modular `view_*` helper methods.
- [ ] **`ui-close-btn`**: Ensure every modal/overlay has a standard [X] close interaction.
- [ ] **`ui-sidebar-about`**: Add "About" and "Logout" buttons to the sidebar bottom.

### 🟡 Milestone 2: Privacy & Trust (OMEMO Resilience)
**Goal:** Fix the OMEMO delivery issues by implementing a complete "Trust" lifecycle and recipient status feedback.

- [ ] **`omemo-trust-ui`**: Wire the `OmemoTrustScreen` as an overlay to manage peer fingerprints.
- [ ] **`omemo-status-header`**: Add persistent encryption/trust icons (Lock/Shield) to the chat header.
- [ ] **`omemo-feedback-loop`**: Show clear UI warnings for "No devices found" or "Untrusted devices".
- [ ] **`omemo-send-plaintext`**: Implement a fallback toggle to send unencrypted messages if OMEMO is not viable for a specific peer.
- [ ] **`omemo-pep-sync`**: Verify automatic OMEMO list/bundle publication on account login and PEP push updates.

### 🔵 Milestone 3: Feature Awakening (Wire-Up & Debt)
**Goal:** Activate dormant features currently marked as `dead_code` to make the client feature-complete.

- [ ] **`wire-audio-player`**: Render inline audio widgets for voice messages (Opus/MP3).
- [ ] **`wire-unread-badges`**: Connect `message_repo::count_unread()` to sidebar contact badges.
- [ ] **`wire-push-toggle`**: Complete the settings toggle for XEP-0357 (Push Notifications).
- [ ] **`wire-contact-time`**: Enable XEP-0202 (Entity Time) in contact profiles.
- [ ] **`wire-muc-sidebar`**: Add a member list/room info panel for group chats.

---

## 🏗️ Technical Architecture & Standards

### 🛡️ Definition of Done (DOD)
1. **Code Hygiene**: Zero `#[allow(dead_code)]` in the newly wired path.
2. **Persistence**: All state (trust, drafts, unread) must persist correctly in SQLite.
3. **Verification**: `cargo test` + manual verification against a secondary client (profanity/conversations).
4. **Consistency**: UI must follow the established palette and typography (Photo 1 style).

### 🧪 Testing Roadmap
- **TDD (src/ui/)**: 24 UI state machine test cases (login, sidebar, conversation).
- **E2E (ts/Docker)**: Infrastructure for real-server testing (Prosody).

---

## 🔌 Feature Inventory (Wire Up vs. Remove)

| Status | Component | Feature | Action |
|---|---|---|---|
| 🔌 **Wire Up** | `ui/audio_player.rs` | Voice Message Playback | Integrate to chat view. |
| 🔌 **Wire Up** | `ui/omemo_trust.rs` | Fingerprint Verification | Link to chat header icon. |
| 🔌 **Wire Up** | `store/message_repo.rs`| Unread Badge Logic | Connect to Sidebar. |
| 🔌 **Wire Up** | `ui/muc_panel.rs` | Room Info Drawer | Integrate to MUC view. |
| 🗑️ **Remove** | `ui/styles.rs` | `cancel_btn_style()` | Delete if unused post-sidebar. |
| 🗑️ **Remove** | `xmpp/modules/console.rs`| Debug Console | Archive as utility only. |

---

## 📁 Source Control (Source of Truth)

| File | Status | Purpose |
|---|---|---|
| [PLAN.md](file:///Users/afa/Developer/arthur/xmpp-start/.claude/PLAN.md) | **ACTIVE** | **Primary Roadmap** (this file) |
| `implementation_plan.md` | **INTERNAL** | Technical audit and detailed specs (brain dir) |
| `tasks.yaml` | *Lost* | Replaced by Milestone tracking above |
| [SETTINGS_REDESIGN.md] | *Lost* | Integrated into Milestone 1 |
