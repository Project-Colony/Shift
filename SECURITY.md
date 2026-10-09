# Security Policy

Shift has no network code and writes no files, but it decodes image files it
did not create: whatever you open, and the other images in its folder as you
move to them. A crafted PNG, JPEG, WebP, GIF or BMP that crashes Shift, hangs
it, exhausts its memory or does worse is in scope.

## Supported versions

Only the **latest release** receives security fixes. Shift ships as a single
binary, through Colony and the release page, so the fix for a vulnerability is
the next release, not a patch to an older one.

## Reporting a vulnerability

Please report vulnerabilities **privately** via
[GitHub Security Advisories](https://github.com/Project-Colony/Shift/security/advisories/new)
("Report a vulnerability"). Do not open a public issue for exploitable bugs.

Include the version (`shift --version`), the platform, and the file or steps
that reproduce it. The report stays private until a fixed release is out.
