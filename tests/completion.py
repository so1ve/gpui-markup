import json
import os
from pathlib import Path
import queue
import subprocess
import tempfile
import threading
import time


CALLER = """use gpui_markup::ui;
use gpui::{div, svg, anchored, ParentElement};
mod gpui {
    pub struct Div;
    pub fn div() -> Div { Div }
    pub fn svg() -> Div { Div }
    pub fn anchored() -> Div { Div }
    pub trait ParentElement: Sized {
        fn child(self, _: impl Sized) -> Self { self }
        fn children(self, _: impl Sized) -> Self { self }
    }
    impl ParentElement for Div {}
    impl Div {
        pub fn flex(self) -> Self { self }
        pub fn flex_col(self) -> Self { self }
        pub fn w(self, _: f32) -> Self { self }
    }
}
struct Header;
impl Header { fn new() -> gpui::Div { div() } }
"""

CASES = [
    ("ui! { di$0 }", "div()", "di"),
    ("ui! { div$0 }", "div()", "div"),
    ("ui! {$0}", "div()", ""),
    ("ui! { di$0 {} }", "div()", "di"),
    ("ui! { div$0 {} }", "div()", "div"),
    ("ui! { div { di$0 } }", "div()", "di"),
    ("ui! { div { div$0 } }", "div()", "div"),
    ("ui! { div { svg$0 } }", "svg()", "svg"),
    ("ui! { div { Header$0 } }", "Header", "Header"),
    ("ui! { div {$0} }", "div()", ""),
    ("ui! { div { $0 } }", "Header", ""),
    ("ui! { div { div { $0 } } }", "div()", ""),
    ("ui! { div { div$0, svg {} } }", "div()", "div"),
    ("ui! { div { div {}, $0 } }", "div()", ""),
    ("ui! { div @[fl$0] {} }", "flex", "fl"),
    ("ui! { div @[flex$0] {} }", "flex", "flex"),
    ("ui! { div @[$0] {} }", "flex", ""),
    ("ui! { div @[flex, $0] {} }", "flex_col", ""),
    ("ui! { div @[fl$0] }", "flex", "fl"),
    ("ui! { div { .$0 } }", "flex()", ""),
]


class LanguageServer:
    def __init__(self, root, log):
        self.process = subprocess.Popen(
            ["rust-analyzer"],
            cwd=root,
            env={**os.environ, "CARGO_TARGET_DIR": "target"},
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=log,
        )
        self.messages = queue.Queue()
        self.sequence = 0
        self.ready = False
        threading.Thread(target=self.read_messages, daemon=True).start()

    def read_messages(self):
        while True:
            length = 0
            while line := self.process.stdout.readline():
                if line == b"\r\n":
                    break
                if line.lower().startswith(b"content-length:"):
                    length = int(line.split(b":", 1)[1])
            if not line:
                self.messages.put({"error": "rust-analyzer exited unexpectedly"})
                return
            self.messages.put(json.loads(self.process.stdout.read(length)))

    def send(self, message):
        body = json.dumps({"jsonrpc": "2.0", **message}).encode()
        self.process.stdin.write(f"Content-Length: {len(body)}\r\n\r\n".encode() + body)
        self.process.stdin.flush()

    def receive(self, timeout):
        message = self.messages.get(timeout=timeout)
        if "error" in message:
            raise RuntimeError(message["error"])
        if "method" in message and "id" in message:
            self.send({"id": message["id"], "result": None})
        if message.get("method") == "experimental/serverStatus":
            status = message["params"]
            if status["health"] == "error":
                raise RuntimeError(status["message"])
            self.ready = status["quiescent"]
        return message

    def request(self, method, params):
        self.sequence += 1
        self.send({"id": self.sequence, "method": method, "params": params})
        deadline = time.monotonic() + 120
        while True:
            message = self.receive(max(0.01, deadline - time.monotonic()))
            if message.get("id") == self.sequence and "method" not in message:
                return message.get("result")

    def notify(self, method, params):
        self.send({"method": method, "params": params})


def check_completions(root, server):
    uri = (root / "src/main.rs").as_uri()
    server.request("initialize", {
        "processId": os.getpid(),
        "rootUri": root.as_uri(),
        "capabilities": {
            "textDocument": {"completion": {"completionItem": {"snippetSupport": True}}},
            "experimental": {"serverStatusNotification": True},
        },
        "initializationOptions": {"checkOnSave": False, "procMacro": {"enable": True}},
    })
    server.notify("initialized", {})
    server.notify("textDocument/didOpen", {
        "textDocument": {
            "uri": uri, "languageId": "rust", "version": 1,
            "text": (root / "src/main.rs").read_text(),
        },
    })
    deadline = time.monotonic() + 120
    while not server.ready:
        server.receive(max(0.01, deadline - time.monotonic()))

    for version, (body, expected, prefix) in enumerate(CASES, 2):
        marked = CALLER + f"fn main() {{ let _ = {body}; }}\n"
        before = marked[:marked.index("$0")]
        cursor = {"line": before.count("\n"), "character": len(before.rsplit("\n", 1)[-1])}
        server.notify("textDocument/didChange", {
            "textDocument": {"uri": uri, "version": version},
            "contentChanges": [{"text": marked.replace("$0", "")}],
        })
        result = server.request("textDocument/completion", {
            "textDocument": {"uri": uri}, "position": cursor, "context": {"triggerKind": 1},
        })
        items = result.get("items", []) if isinstance(result, dict) else result or []
        candidate = next((item for item in items if item["label"] == expected), None)
        assert candidate is not None, f"{body}: missing {expected!r} in {items!r}"
        assert candidate["textEdit"]["range"] == {
            "start": {**cursor, "character": cursor["character"] - len(prefix)},
            "end": cursor,
        }, f"{body}: incorrect replacement range: {candidate!r}"
        print(f"PASS {body} -> {expected}", flush=True)

    server.request("shutdown", None)
    server.notify("exit", None)
    server.process.wait(timeout=10)


def main():
    repository = Path(__file__).resolve().parent.parent
    with tempfile.TemporaryDirectory(prefix="gpui-markup-completion-") as directory:
        root = Path(directory)
        (root / "src").mkdir()
        (root / "Cargo.toml").write_text(
            '[package]\nname = "completion-caller"\nversion = "0.0.0"\nedition = "2024"\n'
            '[workspace]\n[dependencies]\n'
            f'gpui-markup = {{ path = {json.dumps(str(repository))} }}\n'
        )
        (root / "src/main.rs").write_text(CALLER + "fn main() { let _ = ui! { div {} }; }\n")
        with (root / "server.log").open("w+") as log:
            server = LanguageServer(root, log)
            try:
                check_completions(root, server)
            except Exception:
                log.seek(0)
                print(log.read())
                raise
            finally:
                if server.process.poll() is None:
                    server.process.kill()
                    server.process.wait()


if __name__ == "__main__":
    main()
