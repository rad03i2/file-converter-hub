# Security Policy

## Supported versions

Security fixes target the latest version on the default branch.

## Reporting

Please use GitHub's private vulnerability reporting feature when available. Do not publish exploitable details before a fix can be prepared.

## Security model

File Converter Hub is offline by design: it performs no network requests and stores no credentials. Output replacement requires explicit `--overwrite`. Treat untrusted files as untrusted input and run the tool with ordinary user privileges, not as administrator/root.

The project does not claim to sanitize malicious document formats; it converts only the documented formats.

Maintainer: Radwan Abdulhadi Ahmed / رضوان عبدالهادي أحمد / @rad03i2.
