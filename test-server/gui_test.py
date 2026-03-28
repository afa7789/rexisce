#!/usr/bin/env python3
"""
Atomized GUI tests for ReXisCe using Claude Computer Use.

Each test is independent and can be run alone, in groups, or all together.

Setup (one-time):
  1. cp test-server/.env.example test-server/.env
  2. Edit test-server/.env with your ANTHROPIC_API_KEY
  3. Grant Accessibility + Screen Recording to your terminal

Usage:
  cd test-server
  source ../.venv/bin/activate

  python gui_test.py                    # list all tests
  python gui_test.py --list             # list all tests
  python gui_test.py --all              # run all tests
  python gui_test.py login              # run single test
  python gui_test.py login messaging    # run specific tests
  python gui_test.py --group core       # run a group (core, chat, ui)
"""

import anthropic
import base64
import os
import subprocess
import sys
import time
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from typing import Optional

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

PROJECT_ROOT = Path(__file__).parent.parent
APP_BINARY = PROJECT_ROOT / "target" / "debug" / "rexisce"
ENV_FILE = Path(__file__).parent / ".env"

ALICE = ("alice@localhost", "alice123")
BOB = ("bob@localhost", "bob123")


class TestResult(Enum):
    PASS = "PASS"
    FAIL = "FAIL"
    SKIP = "SKIP"
    ERROR = "ERROR"


@dataclass
class GUITest:
    name: str
    group: str
    description: str
    prompt: str
    needs_login: bool = True
    depends_on: list[str] = field(default_factory=list)


# ---------------------------------------------------------------------------
# Test definitions
# ---------------------------------------------------------------------------

TESTS: dict[str, GUITest] = {}


def test(name: str, group: str, description: str, needs_login: bool = True,
         depends_on: list[str] | None = None):
    """Decorator to register a GUI test."""
    def decorator(func):
        prompt = func.__doc__ or ""
        TESTS[name] = GUITest(
            name=name,
            group=group,
            description=description,
            prompt=prompt.strip(),
            needs_login=needs_login,
            depends_on=depends_on or [],
        )
        return func
    return decorator


@test("login", "core", "Connect as alice@localhost", needs_login=False)
def _():
    """
    You are testing an XMPP chat app called ReXisCe. The app window should be on screen.

    TEST: LOGIN
    1. Take a screenshot to see the login screen
    2. Find the JID/username input field and click on it, then type: alice@localhost
    3. Find the password input field and click on it, then type: alice123
    4. If there is a server field, click it and type: localhost
    5. Find and click the Connect or Login button
    6. Wait 3 seconds, then take a screenshot
    7. Check if the app transitioned to a chat screen (you should see a sidebar or contact list)

    RESULT: Say exactly "TEST PASS" if you see the chat/main screen, or "TEST FAIL: <reason>" if not.
    """


@test("login_wrong_password", "core", "Reject wrong password", needs_login=False)
def _():
    """
    You are testing an XMPP chat app called ReXisCe. The app window should be on screen.

    TEST: WRONG PASSWORD
    1. Take a screenshot to see the login screen
    2. Find the JID/username field and type: alice@localhost
    3. Find the password field and type: wrongpassword999
    4. Click the Connect/Login button
    5. Wait 3 seconds, take a screenshot
    6. The app should show an error or remain on the login screen (NOT go to chat)

    RESULT: Say exactly "TEST PASS" if login was rejected (error shown or still on login), or "TEST FAIL: <reason>" if it logged in.
    """


@test("settings_open_close", "ui", "Open and close the settings modal")
def _():
    """
    You are testing an XMPP chat app called ReXisCe. You are logged in and on the chat screen.

    TEST: SETTINGS MODAL
    1. Take a screenshot
    2. Look for a gear/settings icon or a "Settings" menu item. Click it.
    3. Wait 1 second, take a screenshot
    4. A settings modal should appear with a dark overlay and a white/themed panel
    5. The panel should have tabs on the left side (General, Chats, Privacy, etc.)
    6. Find and click the X (close) button in the top-right of the modal
    7. Take a screenshot — the modal should be gone, chat screen visible again

    RESULT: Say exactly "TEST PASS" if the modal opened with tabs and closed properly, or "TEST FAIL: <reason>".
    """


@test("settings_tabs", "ui", "Navigate between settings tabs")
def _():
    """
    You are testing an XMPP chat app called ReXisCe. You are logged in.

    TEST: SETTINGS TAB NAVIGATION
    1. Open settings (gear icon or menu)
    2. Take a screenshot — note the tabs on the left sidebar
    3. Click on "Chats" tab — take a screenshot, content should change
    4. Click on "Privacy" tab — take a screenshot, content should change
    5. Click on "Network" tab — take a screenshot, content should change
    6. Click on "General" tab — take a screenshot, should show appearance settings
    7. Close settings

    RESULT: Say exactly "TEST PASS" if all tabs worked and showed different content, or "TEST FAIL: <reason>".
    """


@test("send_message", "chat", "Send a message to bob", depends_on=["login"])
def _():
    """
    You are testing an XMPP chat app called ReXisCe. You are logged in as alice.

    TEST: SEND MESSAGE
    1. Take a screenshot
    2. In the sidebar, look for bob@localhost or any contact. Click on it.
       If no contacts visible, look for a way to start a new conversation (+ button or similar).
       If there's an input to type a JID, type: bob@localhost
    3. Once in the conversation view, find the message input at the bottom
    4. Click the input field, then type: Hello from GUI test
    5. Press Enter to send
    6. Wait 1 second, take a screenshot
    7. The message "Hello from GUI test" should appear in the conversation

    RESULT: Say exactly "TEST PASS" if the message appears in the conversation, or "TEST FAIL: <reason>".
    """


@test("omemo_toggle", "chat", "Toggle OMEMO encryption on/off")
def _():
    """
    You are testing an XMPP chat app called ReXisCe. You are logged in.

    TEST: OMEMO TOGGLE
    1. Open a conversation (click on any contact in sidebar, or bob@localhost)
    2. Take a screenshot
    3. Look for a lock/shield icon in the conversation header area — this toggles OMEMO encryption
    4. Click the lock icon
    5. Wait 1 second, take a screenshot
    6. A system message like "OMEMO encryption enabled" should appear in the conversation (centered text)
    7. Click the lock icon again to disable
    8. Take a screenshot — "OMEMO encryption disabled" message should appear

    RESULT: Say exactly "TEST PASS" if both toggle messages appeared, or "TEST FAIL: <reason>".
    """


@test("join_muc", "chat", "Join a group chat room")
def _():
    """
    You are testing an XMPP chat app called ReXisCe. You are logged in.

    TEST: JOIN MUC ROOM
    1. Take a screenshot
    2. Look for a "Join Room" button or similar in the sidebar area (might be a + icon or menu)
    3. Click it to open the join room dialog
    4. Type the room JID: testroom@conference.localhost
    5. If there's a nickname field, type: alice
    6. Click Join or OK
    7. Wait 2 seconds, take a screenshot
    8. The room should appear in the sidebar and you should see the room's chat area

    RESULT: Say exactly "TEST PASS" if the room appeared and you can see its chat area, or "TEST FAIL: <reason>".
    """


@test("close_conversation", "ui", "Close a conversation without removing it")
def _():
    """
    You are testing an XMPP chat app called ReXisCe. You are logged in with a conversation open.

    TEST: CLOSE CONVERSATION
    1. Click on any contact in the sidebar to open their conversation
    2. Take a screenshot — note the contact is in the sidebar
    3. Look for a close/X button in the conversation header area
    4. Click it
    5. Take a screenshot
    6. The conversation area should clear (no active conversation selected)
    7. BUT the contact should still be visible in the sidebar (not removed)

    RESULT: Say exactly "TEST PASS" if conversation closed but contact remained in sidebar, or "TEST FAIL: <reason>".
    """


@test("theme_toggle", "ui", "Toggle dark/light theme in settings")
def _():
    """
    You are testing an XMPP chat app called ReXisCe. You are logged in.

    TEST: THEME TOGGLE
    1. Take a screenshot — note current theme (dark or light background)
    2. Open settings (gear icon)
    3. In the General tab, find "Dark theme" toggle
    4. Click the toggle to switch theme
    5. Take a screenshot — the app colors should change (dark to light or vice versa)
    6. Click the toggle again to switch back
    7. Close settings

    RESULT: Say exactly "TEST PASS" if the theme visually changed both times, or "TEST FAIL: <reason>".
    """


GROUPS = {
    "core": ["login", "login_wrong_password"],
    "chat": ["send_message", "omemo_toggle", "join_muc"],
    "ui": ["settings_open_close", "settings_tabs", "close_conversation", "theme_toggle"],
}


# ---------------------------------------------------------------------------
# Computer Use engine
# ---------------------------------------------------------------------------

def load_api_key() -> str:
    """Load API key from .env file or environment."""
    key = os.environ.get("ANTHROPIC_API_KEY")
    if key:
        return key

    if ENV_FILE.exists():
        for line in ENV_FILE.read_text().splitlines():
            line = line.strip()
            if line.startswith("ANTHROPIC_API_KEY=") and not line.endswith("your-key-here"):
                return line.split("=", 1)[1].strip().strip('"').strip("'")

    print("ERROR: No API key found.")
    print(f"  Option 1: export ANTHROPIC_API_KEY=sk-ant-...")
    print(f"  Option 2: cp {ENV_FILE.name}.example {ENV_FILE.name} && edit it")
    sys.exit(1)


def take_screenshot() -> str:
    path = "/tmp/rexisce_test_screenshot.png"
    subprocess.run(["screencapture", "-x", "-C", path], check=True, capture_output=True)
    with open(path, "rb") as f:
        return base64.standard_b64encode(f.read()).decode()


def get_screen_size() -> tuple[int, int]:
    result = subprocess.run(
        ["system_profiler", "SPDisplaysDataType"], capture_output=True, text=True,
    )
    for line in result.stdout.splitlines():
        if "Resolution" in line:
            parts = line.split(":")[-1].strip().split()
            if len(parts) >= 3 and parts[1] == "x":
                return int(parts[0]), int(parts[2].rstrip(","))
    return 1440, 900


def execute_tool_action(tool_use) -> dict:
    """Execute a computer use tool action and return the result."""
    action = tool_use.input.get("action", "")

    if action == "screenshot":
        return {
            "type": "tool_result",
            "tool_use_id": tool_use.id,
            "content": [{
                "type": "image",
                "source": {
                    "type": "base64",
                    "media_type": "image/png",
                    "data": take_screenshot(),
                },
            }],
        }

    elif action == "left_click":
        coords = tool_use.input.get("coordinate", [0, 0])
        subprocess.run(["cliclick", f"c:{coords[0]},{coords[1]}"], capture_output=True)
        time.sleep(0.3)

    elif action == "double_click":
        coords = tool_use.input.get("coordinate", [0, 0])
        subprocess.run(["cliclick", f"dc:{coords[0]},{coords[1]}"], capture_output=True)
        time.sleep(0.3)

    elif action == "right_click":
        coords = tool_use.input.get("coordinate", [0, 0])
        subprocess.run(["cliclick", f"rc:{coords[0]},{coords[1]}"], capture_output=True)
        time.sleep(0.3)

    elif action == "mouse_move":
        coords = tool_use.input.get("coordinate", [0, 0])
        subprocess.run(["cliclick", f"m:{coords[0]},{coords[1]}"], capture_output=True)

    elif action == "left_click_drag":
        start = tool_use.input.get("start_coordinate", [0, 0])
        end = tool_use.input.get("coordinate", [0, 0])
        subprocess.run(
            ["cliclick", f"dd:{start[0]},{start[1]}", f"du:{end[0]},{end[1]}"],
            capture_output=True,
        )

    elif action == "type":
        text = tool_use.input.get("text", "")
        # Use AppleScript for reliable typing (handles special chars)
        subprocess.run(
            ["osascript", "-e", f'tell application "System Events" to keystroke "{text}"'],
            capture_output=True,
        )
        time.sleep(0.2)

    elif action == "key":
        key = tool_use.input.get("text", "")
        key_map = {
            "Return": "return", "Tab": "tab", "Escape": "escape",
            "BackSpace": "delete", "space": "space", "Delete": "delete",
            "Up": "arrow-up", "Down": "arrow-down",
            "Left": "arrow-left", "Right": "arrow-right",
        }
        cliclick_key = key_map.get(key, key.lower())
        subprocess.run(["cliclick", f"kp:{cliclick_key}"], capture_output=True)
        time.sleep(0.2)

    elif action == "scroll":
        coords = tool_use.input.get("coordinate", [0, 0])
        direction = tool_use.input.get("direction", "down")
        amount = tool_use.input.get("amount", 3)
        clicks = amount * (1 if direction == "down" else -1)
        subprocess.run(["cliclick", f"m:{coords[0]},{coords[1]}"], capture_output=True)
        # osascript scroll
        subprocess.run(
            ["osascript", "-e",
             f'tell application "System Events" to scroll area 1 by {clicks}'],
            capture_output=True,
        )

    else:
        return {
            "type": "tool_result",
            "tool_use_id": tool_use.id,
            "content": f"unknown action: {action}",
        }

    return {
        "type": "tool_result",
        "tool_use_id": tool_use.id,
        "content": f"{action} done",
    }


def run_single_test(
    test_def: GUITest,
    client: anthropic.Anthropic,
    width: int,
    height: int,
    app_proc: Optional[subprocess.Popen],
) -> TestResult:
    """Run a single GUI test via Claude Computer Use. Returns PASS/FAIL."""

    screenshot_b64 = take_screenshot()

    messages = [
        {
            "role": "user",
            "content": [
                {"type": "text", "text": test_def.prompt},
                {
                    "type": "image",
                    "source": {
                        "type": "base64",
                        "media_type": "image/png",
                        "data": screenshot_b64,
                    },
                },
            ],
        }
    ]

    tools = [
        {
            "type": "computer_20250124",
            "name": "computer",
            "display_width_px": width,
            "display_height_px": height,
            "display_number": 1,
        },
    ]

    for iteration in range(25):
        response = client.messages.create(
            model="claude-sonnet-4-20250514",
            max_tokens=4096,
            tools=tools,
            messages=messages,
            betas=["computer-use-2025-01-24"],
        )

        assistant_content = response.content
        messages.append({"role": "assistant", "content": assistant_content})

        tool_uses = [b for b in assistant_content if b.type == "tool_use"]

        if not tool_uses:
            # Claude is done — check for PASS/FAIL in output
            for block in assistant_content:
                if hasattr(block, "text"):
                    text = block.text
                    if "TEST PASS" in text:
                        return TestResult.PASS
                    elif "TEST FAIL" in text:
                        print(f"    {text}")
                        return TestResult.FAIL
            return TestResult.ERROR

        # Execute tool actions
        tool_results = []
        for tool_use in tool_uses:
            action = tool_use.input.get("action", "?")
            coords = tool_use.input.get("coordinate", "")
            print(f"    [{iteration+1}] {action} {coords}")
            result = execute_tool_action(tool_use)
            tool_results.append(result)

        messages.append({"role": "user", "content": tool_results})

    return TestResult.ERROR


# ---------------------------------------------------------------------------
# Runner
# ---------------------------------------------------------------------------

def build_app():
    print("Building app...")
    result = subprocess.run(
        ["cargo", "build"], cwd=PROJECT_ROOT, capture_output=True, text=True,
    )
    if result.returncode != 0:
        print(f"Build failed:\n{result.stderr[-500:]}")
        sys.exit(1)
    print("Build OK")


def launch_app() -> subprocess.Popen:
    print(f"Launching {APP_BINARY.name}...")
    proc = subprocess.Popen([str(APP_BINARY)])
    time.sleep(3)
    return proc


def kill_app(proc: subprocess.Popen):
    proc.terminate()
    try:
        proc.wait(timeout=5)
    except subprocess.TimeoutExpired:
        proc.kill()


def run_tests(test_names: list[str]):
    """Run a list of tests in order."""
    api_key = load_api_key()
    width, height = get_screen_size()
    print(f"Screen: {width}x{height}")

    client = anthropic.Anthropic(api_key=api_key)

    build_app()

    results: dict[str, TestResult] = {}
    app_proc: Optional[subprocess.Popen] = None
    logged_in = False

    for name in test_names:
        test_def = TESTS[name]

        # Launch/relaunch app as needed
        if not test_def.needs_login and app_proc:
            kill_app(app_proc)
            app_proc = None
            logged_in = False
            time.sleep(1)

        if app_proc is None:
            app_proc = launch_app()
            logged_in = False

        # If test needs login but we're not logged in, run login first
        if test_def.needs_login and not logged_in:
            if "login" not in results or results["login"] != TestResult.PASS:
                print(f"\n{'='*50}")
                print(f"  PRE-REQ: login")
                print(f"{'='*50}")
                login_result = run_single_test(TESTS["login"], client, width, height, app_proc)
                results["login"] = login_result
                if login_result == TestResult.PASS:
                    logged_in = True
                    print(f"  -> PASS")
                else:
                    print(f"  -> FAIL (skipping {name})")
                    results[name] = TestResult.SKIP
                    continue

        print(f"\n{'='*50}")
        print(f"  TEST: {name}")
        print(f"  {test_def.description}")
        print(f"{'='*50}")

        result = run_single_test(test_def, client, width, height, app_proc)
        results[name] = result
        print(f"  -> {result.value}")

        if name == "login" and result == TestResult.PASS:
            logged_in = True

        # Brief pause between tests
        time.sleep(1)

    # Cleanup
    if app_proc:
        kill_app(app_proc)

    # Summary
    print(f"\n{'='*50}")
    print(f"  RESULTS")
    print(f"{'='*50}")
    for name, result in results.items():
        icon = {"PASS": "+", "FAIL": "x", "SKIP": "-", "ERROR": "!"}[result.value]
        print(f"  [{icon}] {name}: {result.value}")

    passed = sum(1 for r in results.values() if r == TestResult.PASS)
    total = len(results)
    print(f"\n  {passed}/{total} passed")

    return all(r in (TestResult.PASS, TestResult.SKIP) for r in results.values())


def main():
    args = sys.argv[1:]

    if not args or args == ["--list"]:
        print("Available GUI tests:\n")
        for name, t in TESTS.items():
            print(f"  {name:30s} [{t.group:5s}]  {t.description}")
        print(f"\nGroups: {', '.join(GROUPS.keys())}")
        print(f"\nUsage:")
        print(f"  python gui_test.py login              # single test")
        print(f"  python gui_test.py login settings_open_close  # multiple tests")
        print(f"  python gui_test.py --group core       # run a group")
        print(f"  python gui_test.py --all              # run all tests")
        return

    if args == ["--all"]:
        test_names = list(TESTS.keys())
    elif args[0] == "--group" and len(args) > 1:
        group = args[1]
        if group not in GROUPS:
            print(f"Unknown group: {group}. Available: {', '.join(GROUPS.keys())}")
            sys.exit(1)
        test_names = GROUPS[group]
    else:
        test_names = []
        for a in args:
            if a in TESTS:
                test_names.append(a)
            else:
                print(f"Unknown test: {a}")
                print(f"Available: {', '.join(TESTS.keys())}")
                sys.exit(1)

    success = run_tests(test_names)
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
