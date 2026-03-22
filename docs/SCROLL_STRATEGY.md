# Scroll Strategy

## Decision: iced native scrollable + AbsoluteOffset tracking

After benchmarking 10,000 items in iced's scrollable widget:

### Approach chosen
- Use `iced::widget::scrollable` with `Direction::Vertical`
- Track `AbsoluteOffset` in app state for persistence per conversation
- "Scroll to bottom" via `scrollable::scroll_to()` Task
- New message detection: compare stored offset vs content height estimate

### Why not virtual/windowed list
iced 0.13's scrollable renders all items but clips painting — acceptable for
~1,000–5,000 messages. For very long histories (10k+), we will load messages
in pages from SQLite (MAM pagination, task P4.1) so the rendered list stays
bounded. No custom virtual list needed for MVP.

### Anchor tracking (replaces DOM-based approach from useMessageListScroll.ts)
- Store `anchor_message_id: Option<String>` — the first visible message ID
- On new message: if user is at bottom (offset near max), auto-scroll
- If user scrolled up: preserve position, show "N new messages" badge
- No ResizeObserver, no rAF, no element queries — pure state

### Files
- `src/ui/benchmark.rs` — prototype with scroll offset tracking
- `src/ui/chat.rs` (future, Task P2.3) — production implementation
