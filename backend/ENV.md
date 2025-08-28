Backend environment variables

- CONTEXT_DIR: base dir for chat history (default: context)
- CONTEXT_MAX_LINES: max lines kept in a file before trimming (default: 500)
- CONTEXT_MAX_BYTES: max bytes per file before trimming (default: 1_000_000)
- CONTEXT_DAILY_ROTATION: if true, rotate files daily with -YYYYMMDD suffix (default: true)
- CONTEXT_ARCHIVE_GZ: if true, gzip previous days’ files (default: true)
- CONTEXT_FLUSH_MS: buffered write flush interval in ms; 0 disables buffering (default: 0)
- MASK_PII: enable masking of PII like emails/phones (default: true)
- MASK_REGEX: semicolon-separated custom regex to mask (default: empty)
- MASK_ROLES: comma-separated roles to mask: user,assistant,system (default: user)
- CHAT_RATE_LIMIT_PER_MIN: max chat requests per minute per key (default: 120)
- CHAT_RATE_KEY: rate limit key, either "auth" or "chat" (default: auth)

How to use
- Create a .env file in repo root or `backend/` and set variables.
- The app loads env via dotenv at startup; env vars override .env.

Example .env
CONTEXT_DIR=./context
CONTEXT_MAX_LINES=1000
CONTEXT_MAX_BYTES=2000000
CONTEXT_DAILY_ROTATION=true
CONTEXT_ARCHIVE_GZ=true
CONTEXT_FLUSH_MS=0
MASK_PII=true
MASK_REGEX=\b\d{16}\b;secret\w+
MASK_ROLES=user,assistant
CHAT_RATE_LIMIT_PER_MIN=60
CHAT_RATE_KEY=auth

