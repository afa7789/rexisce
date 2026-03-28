#!/usr/bin/env python3
"""
Atomized GUI tests for ReXisCe using cliclick + osascript + screencapture.
No API key needed — runs entirely within Claude Code session.

Usage:
  python test-server/gui_test.py login
  python test-server/gui_test.py --all
  python test-server/gui_test.py --group core
  python test-server/gui_test.py --list
"""

import os
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent
APP_BINARY = PROJECT_ROOT / "target" / "debug" / "rexisce"
SETTINGS_DIR = Path.home() / "Library" / "Application Support" / "rexisce"

# Window state
WIN_X, WIN_Y, WIN_W, WIN_H = 244, 75, 1024, 796


def screenshot_window(name="test"):
    path = f"/tmp/rexisce_{name}.png"
    subprocess.run(
        ["screencapture", "-x", "-R", f"{WIN_X},{WIN_Y},{WIN_W},{WIN_H}", path],
        capture_output=True,
    )
    return path


def click(x, y):
    subprocess.run(["cliclick", f"c:{x},{y}"], capture_output=True)
    time.sleep(0.3)


def type_text(text):
    escaped = text.replace("\\", "\\\\").replace('"', '\\"')
    subprocess.run(
        ["osascript", "-e", f'tell application "System Events" to keystroke "{escaped}"'],
        capture_output=True,
    )
    time.sleep(0.2)


def press_tab():
    subprocess.run(
        ["osascript", "-e", 'tell application "System Events" to key code 48'],
        capture_output=True,
    )
    time.sleep(0.3)


def select_all():
    subprocess.run(
        ["osascript", "-e",
         'tell application "System Events" to keystroke "a" using command down'],
        capture_output=True,
    )
    time.sleep(0.1)


def press_key(key):
    subprocess.run(["cliclick", f"kp:{key}"], capture_output=True)
    time.sleep(0.3)


def focus_app():
    subprocess.run(
        ["osascript", "-e",
         'tell application "System Events" to set frontmost of process "rexisce" to true'],
        capture_output=True,
    )
    time.sleep(0.5)


def get_window_pos():
    global WIN_X, WIN_Y, WIN_W, WIN_H
    r = subprocess.run(
        ["osascript", "-e", """
tell application "System Events"
    tell process "rexisce"
        set p to position of window 1
        set s to size of window 1
        return (item 1 of p as text) & " " & (item 2 of p as text) & " " & (item 1 of s as text) & " " & (item 2 of s as text)
    end tell
end tell"""],
        capture_output=True, text=True,
    )
    parts = r.stdout.strip().split()
    if len(parts) == 4:
        WIN_X, WIN_Y, WIN_W, WIN_H = [int(p) for p in parts]


def clear_settings():
    """Remove all saved state so the app starts fresh (no auto-connect)."""
    for f in ["settings.json", "credentials.json"]:
        p = SETTINGS_DIR / f
        if p.exists():
            p.unlink()
    # Also remove the DB to prevent auto-connect from cached conversations
    db = SETTINGS_DIR / "messages.db"
    if db.exists():
        db.unlink()


def kill_app():
    subprocess.run(["pkill", "-f", "target/debug/rexisce"], capture_output=True)
    time.sleep(1)


# --- Button coordinate helpers (relative to window) ---

def win_pos(x_pct, y_pct):
    """Convert percentage of window to screen coordinates."""
    return WIN_X + int(WIN_W * x_pct / 100), WIN_Y + int(WIN_H * y_pct / 100)


def sidebar_btn(name):
    """Get screen coords for sidebar header buttons."""
    # From screenshot analysis: buttons at y=77 in window, spaced at x=105,131,166
    y = WIN_Y + 77
    if name == "+":
        return WIN_X + 105, y
    elif name == "#":
        return WIN_X + 131, y
    elif name == "new":
        return WIN_X + 166, y
    elif name == "account":
        return WIN_X + 90, WIN_Y + 47  # alice@localhost bar
    return 0, 0


def login_as(jid="alice@localhost", password="alice123", server="localhost"):
    focus_app()
    get_window_pos()
    time.sleep(0.5)

    cx, jid_y = win_pos(50, 36)
    click(cx, jid_y)
    time.sleep(0.3)
    select_all()
    type_text(jid)

    press_tab()
    select_all()
    type_text(password)

    press_tab()
    select_all()
    type_text(server)
    time.sleep(0.3)

    # Click Connect (sweep to find button)
    bx = WIN_X + int(WIN_W * 0.43)
    for y in range(WIN_Y + int(WIN_H * 0.57), WIN_Y + int(WIN_H * 0.62), 3):
        click(bx, y)


def wait_connect(seconds=10):
    time.sleep(seconds)


# ---------------------------------------------------------------------------
# Tests
# ---------------------------------------------------------------------------

@dataclass
class TestResult:
    name: str
    passed: bool
    screenshot: str = ""
    message: str = ""


def test_login() -> TestResult:
    clear_settings()
    proc = subprocess.Popen([str(APP_BINARY)])
    time.sleep(3)
    login_as()
    wait_connect(10)
    get_window_pos()
    path = screenshot_window("login")
    proc.terminate(); proc.wait(timeout=5)
    return TestResult("login", True, path, "Check: chat screen with alice@localhost in title bar")


def test_login_wrong_password() -> TestResult:
    clear_settings()
    proc = subprocess.Popen([str(APP_BINARY)])
    time.sleep(3)
    login_as(password="wrongpassword999")
    wait_connect(8)
    path = screenshot_window("wrong_password")
    proc.terminate(); proc.wait(timeout=5)
    return TestResult("login_wrong_password", True, path,
                      "Check: should show error or stay on login screen (NOT chat screen)")


def test_send_message() -> TestResult:
    clear_settings()
    proc = subprocess.Popen([str(APP_BINARY)])
    time.sleep(3)
    login_as()
    wait_connect(10)
    focus_app(); get_window_pos()

    # Click "+" (add contact) button
    x, y = sidebar_btn("+")
    click(x, y)
    time.sleep(0.5)

    # Type bob@localhost in the add-contact input
    type_text("bob@localhost")
    press_key("return")
    time.sleep(2)

    # Click on bob in sidebar (should appear below the buttons)
    click(WIN_X + 90, WIN_Y + 115)
    time.sleep(1)

    # Type message in composer (bottom of window)
    cx, cy = win_pos(50, 95)
    click(cx, cy)
    time.sleep(0.3)
    type_text("Hello from GUI test!")
    press_key("return")
    time.sleep(2)

    path = screenshot_window("send_message")
    proc.terminate(); proc.wait(timeout=5)
    return TestResult("send_message", True, path,
                      "Check: message 'Hello from GUI test!' visible in conversation")


def test_settings_open_close() -> TestResult:
    clear_settings()
    proc = subprocess.Popen([str(APP_BINARY)])
    time.sleep(3)
    login_as()
    wait_connect(10)
    focus_app(); get_window_pos()

    # Click alice@localhost bar to open account menu
    x, y = sidebar_btn("account")
    click(x, y)
    time.sleep(1)

    # Screenshot the dropdown menu
    path_menu = screenshot_window("settings_menu")

    # Click "Settings" in the dropdown (it's below the account bar)
    # The menu items are: Available, Away, DND, Settings, Switch Account
    # Settings is ~4th item, each ~25px apart, starting ~20px below the bar
    settings_y = WIN_Y + 47 + 100  # rough estimate for 4th menu item
    click(WIN_X + 90, settings_y)
    time.sleep(1)

    path_open = screenshot_window("settings_open")

    # Close with Escape
    press_key("esc")
    time.sleep(0.5)
    path_close = screenshot_window("settings_close")

    proc.terminate(); proc.wait(timeout=5)
    return TestResult("settings_open_close", True, path_open,
                      f"Check: settings modal with tabs. Screenshots: {path_menu}, {path_open}, {path_close}")


def test_join_muc() -> TestResult:
    clear_settings()
    proc = subprocess.Popen([str(APP_BINARY)])
    time.sleep(3)
    login_as()
    wait_connect(10)
    focus_app(); get_window_pos()

    # Click "#" button to open Join Room dialog
    x, y = sidebar_btn("#")
    click(x, y)
    time.sleep(1)

    # The Join Room dialog has: room JID field, nick field, Join button
    # Type room JID
    type_text("testroom@conference.localhost")
    press_tab()
    type_text("alice")
    press_key("return")
    time.sleep(3)

    path = screenshot_window("join_muc")
    proc.terminate(); proc.wait(timeout=5)
    return TestResult("join_muc", True, path,
                      "Check: testroom visible in sidebar or chat area")


TESTS = {
    "login": ("core", test_login),
    "login_wrong_password": ("core", test_login_wrong_password),
    "send_message": ("chat", test_send_message),
    "settings_open_close": ("ui", test_settings_open_close),
    "join_muc": ("chat", test_join_muc),
}

GROUPS = {
    "core": ["login", "login_wrong_password"],
    "chat": ["send_message", "join_muc"],
    "ui": ["settings_open_close"],
}


def run_tests(names):
    print("Building app...")
    r = subprocess.run(["cargo", "build"], cwd=PROJECT_ROOT, capture_output=True, text=True)
    if r.returncode != 0:
        print(f"Build failed:\n{r.stderr[-300:]}"); sys.exit(1)

    results = []
    for name in names:
        group, func = TESTS[name]
        print(f"\n{'='*50}")
        print(f"  TEST: {name} [{group}]")
        print(f"{'='*50}")
        try:
            result = func()
            results.append(result)
            print(f"  Screenshot: {result.screenshot}")
            print(f"  {result.message}")
        except Exception as e:
            results.append(TestResult(name, False, "", str(e)))
            print(f"  ERROR: {e}")
        kill_app()

    print(f"\n{'='*50}")
    print("  SUMMARY")
    print(f"{'='*50}")
    for r in results:
        print(f"  [{r.name}] {r.message[:70]}")
        if r.screenshot:
            print(f"    -> {r.screenshot}")
    print(f"\n  {len(results)} test(s). Verify screenshots with: Read /tmp/rexisce_<name>.png")


def main():
    args = sys.argv[1:]
    if not args or args == ["--list"]:
        print("GUI tests:\n")
        for n, (g, _) in TESTS.items():
            print(f"  {n:30s} [{g}]")
        print(f"\nGroups: {', '.join(GROUPS)}")
        print(f"  python gui_test.py login / --all / --group core")
        return

    if args == ["--all"]:
        names = list(TESTS.keys())
    elif args[0] == "--group" and len(args) > 1:
        names = GROUPS.get(args[1], [])
    else:
        names = [a for a in args if a in TESTS]

    if not names:
        print("No valid tests specified."); sys.exit(1)
    run_tests(names)


if __name__ == "__main__":
    main()
