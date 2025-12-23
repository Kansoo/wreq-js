# wreq-js

[![npm](https://img.shields.io/npm/v/wreq-js.svg)](https://www.npmjs.com/package/wreq-js)
[![CI](https://github.com/sqdshguy/wreq-js/actions/workflows/test.yml/badge.svg)](https://github.com/sqdshguy/wreq-js/actions/workflows/test.yml)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/sqdshguy/wreq-js)

High-performance HTTP client for Node.js with real-browser TLS and HTTP/2 fingerprints, powered by Rust.

- Native performance (no browser/process spawning)
- Real browser TLS fingerprints (JA3/JA4)
- HTTP/2 impersonation (SETTINGS/PRIORITY/header ordering)
- Multiple browser profiles (Chrome/Firefox/Safari/Edge/Opera/OkHttp)
- WebSocket support with fingerprint consistency
- Prebuilt native binaries for macOS/Linux/Windows
- TypeScript-ready with generated definitions

## Documentation

All guides, concepts, and API reference live at:

- https://wreq.sqdsh.win

(If you're looking for examples, sessions/cookies, proxy usage, streaming, WebSockets, or the full API surface - it's all there.)

## Installation

```bash
npm install wreq-js
# or
yarn add wreq-js
pnpm add wreq-js
bun add wreq-js
```

Prebuilt binaries are provided for:
- macOS (Intel & Apple Silicon)
- Linux (x64 & ARM64, glibc & musl)
- Windows (x64)

If a prebuilt binary for your platform/commit is unavailable, the package will build from source (requires a Rust toolchain).

## Quick start

```ts
import { fetch } from 'wreq-js';

const res = await fetch('https://example.com/api', {
  browser: 'chrome_142',
  os: 'windows',
});

console.log(await res.json());
```

## Use sessions (recommended)

For **most real-world workloads**, start with a session and reuse it across requests.
This keeps TLS/cookies warm and avoids paying setup costs on every call.

```ts
import { createSession } from 'wreq-js';

const session = await createSession({ browser: 'chrome_142', os: 'windows' });

try {
  const a = await session.fetch('https://example.com/a');
  const b = await session.fetch('https://example.com/b');
  console.log(a.status, b.status);
} finally {
  await session.close();
}
```

More session patterns: https://wreq.sqdsh.win

## When to use

Use `wreq-js` when you need `fetch()`-style ergonomics but want the network layer to look like a real browser.
If you need DOM/JS execution, CAPTCHA solving, or full browser automation, use Playwright/Puppeteer instead.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Origins

This is a maintained fork of [will-work-for-meal/node-wreq](https://github.com/will-work-for-meal/node-wreq) (originally named `node-wreq`), with ongoing updates, compatibility fixes, and performance work.

## Acknowledgments

- [wreq](https://github.com/0x676e67/wreq) - Rust HTTP client with browser impersonation
- [wreq-util](https://github.com/0x676e67/wreq-util) - source of up-to-date browser profiles
- [Neon](https://neon-bindings.com/) - Rust ↔ Node.js bindings
