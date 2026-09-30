#!/usr/bin/env python3
"""Publish docs/USER_MANUAL.md to this repository's GitHub wiki.

One wiki page per section: each `##` chapter and each `###` section becomes a
page named `Section-<number>-<slug>`, the way the manual's own numbering reads
(`Section-3.2.1` -> `Section-3-2-1-...`). A chapter's own text, and any `####`
subsections under it, live on the section page they belong to.

Three things the manual does internally that a wiki breaks, and what this does
about them:

- **Cross-references** (`[§3.2.8](#328-11-m-and-the-citizens-band-wsjt-cb)`) point
  at an anchor in the *same* document. On the wiki the target is a different
  page, so each link is rewritten to `[§3.2.8](Section-3-2-8-...#anchor)`, using
  GitHub's heading-slug rule for the anchor so it lands on the right sub-heading.
- **Images** (`![...](images/04-band-mode-popup.jpg)`) are a path in the repo,
  which the wiki has no `images/` beside. They are rewritten to the raw file on
  `main`, so the wiki always shows the current picture and carries no copies.
- **The table of contents** is rebuilt, since its anchors are page-local.

The manual stays the single source of truth: edit it, re-run this, and the wiki
is regenerated. Pages not produced by this run are left alone (a hand-written
wiki page survives).

Usage:
    tools/manual-to-wiki.py [--dry-run] [--remote <wiki-url>]

Requires `git` and, for the push, credentials that can write the wiki (the same
ones `git push` to the repo uses).
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
MANUAL = REPO / "docs" / "USER_MANUAL.md"
# Where the images live on main, so a wiki page shows the repo's current copy.
RAW_IMAGE_BASE = "https://raw.githubusercontent.com/madmedicnl/sdroxide-brown/main/docs/images/"

HEADING = re.compile(r"^(#{2,4})\s+(.*?)\s*$")
# An internal link: `](#anything)`.
LINK = re.compile(r"\]\(#([^)]+)\)")
# An image: `![alt](path)`.
IMAGE = re.compile(r"!\[([^\]]*)\]\(([^)]+)\)")


def slug(text: str) -> str:
    """GitHub's heading slug: lower-case, punctuation dropped, spaces to `-`.

    Matches the anchors the manual already uses (`#328-11-m-and-the-citizens-band-wsjt-cb`),
    so a rewritten link lands on the same sub-heading it used to.
    """
    s = text.strip().lower()
    s = re.sub(r"[^\w\s-]", "", s)  # drop punctuation, keep word chars/space/hyphen
    s = re.sub(r"\s+", "-", s.strip())
    return s


class Section:
    def __init__(self, number: str, title: str, level: int):
        self.number = number  # "3.2.1" or "3"
        self.title = title  # "3.2.1  Some heading"
        # Page name: `Section-3-2-1-...` from the leading number.
        num_slug = number.replace(".", "-")
        rest = slug(re.sub(r"^[\d.]+\s*", "", title))
        self.page = f"Section-{num_slug}-{rest}" if rest else f"Section-{num_slug}"
        self.level = level
        self.lines: list[str] = []
        # Anchors of every heading inside the section (### and ####), so a link
        # to a sub-heading can be pointed at this page with that anchor.
        self.anchors: set[str] = set()

    def add_anchor(self, heading: str) -> None:
        # The anchor keeps the section number: GitHub slugs "3.2.8 11 m..."
        # to "328-11-m...", which is exactly what the manual's links use.
        self.anchors.add(slug(heading))


def leading_number(title: str) -> str | None:
    m = re.match(r"\s*(\d+(?:\.\d+)*)[.\s]", title)
    return m.group(1) if m else None


def parse(manual: str) -> tuple[list[Section], list[str]]:
    """Split into sections, plus the preamble before the first `##`."""
    sections: list[Section] = []
    preamble: list[str] = []
    current: Section | None = None
    in_toc = False

    for line in manual.splitlines():
        m = HEADING.match(line)

        # While skipping the manual's own table of contents, watch only for the
        # divider that ends it — every line here is a TOC row or blank, and the
        # `not m` branch below would otherwise swallow the `---` forever.
        if in_toc:
            if line.strip() == "---":
                in_toc = False
            continue

        if not m:
            if current is None:
                preamble.append(line)
            else:
                current.lines.append(line)
            continue

        hashes, text = m.group(1), m.group(2)
        level = len(hashes)
        # Skip the manual's own table of contents: it is rebuilt for the wiki.
        if level == 2 and text.strip().lower() == "table of contents":
            in_toc = True
            current = None
            continue

        if level == 2:
            # A chapter. Start a section if it is a numbered one; skip the
            # unnumbered dividers.
            num = leading_number(text)
            if num is None:
                current = None
                continue
            current = Section(num, text, level)
            current.lines.append(f"# {text}")
            current.add_anchor(text)
            sections.append(current)
        elif level == 3 and current is not None:
            num = leading_number(text)
            if num is None:
                # A ### without a number stays on the current page.
                current.lines.append(line)
                current.add_anchor(text)
                continue
            # A numbered ### is its own page.
            current = Section(num, text, level)
            current.lines.append(f"# {text}")
            current.add_anchor(text)
            sections.append(current)
        else:
            # #### and deeper: stay on the current page.
            if current is not None:
                current.lines.append(line)
                current.add_anchor(text)
    return sections, preamble


def build_page(section: Section, index: dict[str, str]) -> str:
    """Render one section's markdown, rewriting links and images."""
    out: list[str] = []
    for line in section.lines:
        line = IMAGE.sub(lambda m: f"![{m.group(1)}]({RAW_IMAGE_BASE}{m.group(2).split('/')[-1]})", line)
        line = LINK.sub(lambda m: rewrite_link(m.group(1), index), line)
        out.append(line)
    body = "\n".join(out).rstrip() + "\n"
    # A footer linking back to the manual and the home page, on every page.
    return (
        body
        + "\n---\n\n"
        + "*From the [User Manual](https://github.com/madmedicnl/sdroxide-brown/blob/main/docs/USER_MANUAL.md). "
        + "[Home](Home).*\n"
    )


def rewrite_link(anchor: str, index: dict[str, str]) -> str:
    """`#328-...` -> the wiki page + anchor that holds it, else the anchor as-is."""
    target = index.get(anchor)
    if target is None:
        # Try a looser match: the anchor may be a sub-heading slug that lives on
        # a page reached by its own heading.
        return f"#{anchor}"
    page, sub = target
    return f"{page}#{sub}" if sub else page


def build_index(sections: list[Section]) -> dict[str, str]:
    """anchor-slug -> (page, sub-anchor) for every heading the manual links to."""
    index: dict[str, str] = {}
    for s in sections:
        # The section's own heading anchor.
        own = slug(s.title)
        index[own] = (s.page, "")
        # Its full-text anchor too (the manual writes both forms).
        index[slug(s.title)] = (s.page, "")
        for a in s.anchors:
            index[a] = (s.page, a)
    return index


def build_home(sections: list[Section]) -> str:
    """The Home page: the manual's table of contents, as wiki links."""
    lines = [
        "# SDR Oxide Brown — User Manual",
        "",
        "The manual, one page per section. The single source is "
        "[`docs/USER_MANUAL.md`](https://github.com/madmedicnl/sdroxide-brown/blob/main/docs/USER_MANUAL.md); "
        "these pages are generated from it.",
        "",
    ]
    for s in sections:
        indent = "" if s.level == 2 else "    "
        label = s.title if s.level == 2 else s.title
        lines.append(f"{indent}- [{label}]({s.page})")
    lines.append("")
    return "\n".join(lines)


def push(pages: dict[str, str], remote: str, dry: bool) -> None:
    for name, _ in sorted(pages.items()):
        pass  # names logged below
    if dry:
        for name in sorted(pages):
            print(f"  would write {name}.md ({len(pages[name])} bytes)")
        return
    with tempfile.TemporaryDirectory() as tmp:
        d = Path(tmp)
        subprocess.run(["git", "init", "-q"], cwd=d, check=True)
        subprocess.run(["git", "remote", "add", "origin", remote], cwd=d, check=True)
        subprocess.run(["git", "fetch", "-q", "--depth=1", "origin"], cwd=d, check=False)
        # Check out the wiki's existing pages where there are any, so pages this
        # run does not produce are kept.
        subprocess.run(["git", "checkout", "-q", "-B", "master", "origin/master"], cwd=d, check=False)
        for name, text in pages.items():
            (d / f"{name}.md").write_text(text)
        subprocess.run(["git", "add", "-A"], cwd=d, check=True)
        done = subprocess.run(
            ["git", "diff", "--cached", "--quiet"], cwd=d
        ).returncode
        if done == 0:
            print("wiki already up to date; nothing to push")
            return
        subprocess.run(
            ["git", "-c", "user.name=madmedicnl", "-c", "user.email=royschuurmans@gmail.com",
             "commit", "-q", "-m", "Publish the user manual from docs/USER_MANUAL.md"],
            cwd=d, check=True,
        )
        subprocess.run(["git", "push", "-q", "origin", "master"], cwd=d, check=True)
        print(f"pushed {len(pages)} pages to the wiki")


def wiki_exists(remote: str) -> bool:
    """Whether the wiki git repo exists yet.

    GitHub creates `<repo>.wiki.git` only once the wiki has its **first page**,
    and offers **no API** for that — it is a one-time step in the web UI
    (`https://github.com/<owner>/<repo>/wiki` -> *Create the first page*). Until
    then a push gets "Repository not found". This check turns that into a plain
    instruction rather than a git error.
    """
    out = subprocess.run(
        ["git", "ls-remote", remote], capture_output=True, text=True
    )
    return out.returncode == 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--dry-run", action="store_true", help="print what would be written, push nothing")
    ap.add_argument(
        "--remote",
        default="https://github.com/madmedicnl/sdroxide-brown.wiki.git",
        help="the wiki git remote",
    )
    args = ap.parse_args()

    manual = MANUAL.read_text()
    sections, _ = parse(manual)
    if not sections:
        print("no sections found — has the manual's heading style changed?", file=sys.stderr)
        return 1
    index = build_index(sections)
    pages = {"Home": build_home(sections)}
    # A section number can repeat a page name only if two headings share a
    # number; the manual does not, but guard anyway.
    for s in sections:
        if s.page in pages:
            print(f"warning: duplicate page name {s.page}", file=sys.stderr)
        pages[s.page] = build_page(s, index)
    print(f"{len(pages)} pages from {len(sections)} sections")
    if not args.dry_run and not wiki_exists(args.remote):
        print(
            "\nThe wiki does not exist yet. GitHub creates it with its first "
            "page, which has no API:\n"
            "  1. Open https://github.com/madmedicnl/sdroxide-brown/wiki\n"
            "  2. Click \"Create the first page\", save anything (this run "
            "overwrites it)\n"
            "  3. Re-run this script\n",
            file=sys.stderr,
        )
        return 1
    push(pages, args.remote, args.dry_run)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
