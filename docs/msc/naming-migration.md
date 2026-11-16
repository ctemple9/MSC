# Minecraft Server Controller naming migration

Owner instruction, 2026-11-15. Decision: [D-046](msc-decisions.md#d-046--product-and-background-service-naming).

The product is **Minecraft Server Controller (MSC)**. Use **MSC** after its
introduction. The always-on component is **MSC Background Service**, shortened
to **Background Service** in navigation and **service** in actions. Source
components use **engine**, including `crates/msc-engine/`. The command remains
`msc`; `msc start engine` also accepts `service` and the former target as hidden
compatibility aliases. This is a terminology change, not a server-data move.

Current documents live in `docs/msc/`; their filenames use MSC. Historical
evidence keeps the names recorded at the time. Links into moved documents are
updated. Git history is untouched. The local checkout directory is not renamed
while tools are using it, and this repository task does not change remote repository settings.

## Documented compatibility exceptions

The following are deliberately retained machine spellings, not current product
labels. Changing these without a separate migration can disconnect existing
servers, credentials, saved settings or running update transactions. The exact
remaining tokens, their files, counts and reasons are recorded in
[`naming-compatibility.json`](naming-compatibility.json). The read-only
`python tools/naming-audit.py` command checks that inventory; it does not launch
tests, contact a service or change an installed application.

| Retained identity | Why it remains |
|---|---|
| `com.ctemple.msc2` and related service/helper/indicator labels | Existing application identity, service ownership, keychain authorization and startup registration. The Windows service display name is **MSC Background Service**. The MSI upgrade code remains unchanged. |
| `MSC2`, `MSC 2`, `.msc2`, `/usr/lib/msc2`, `/var/lib/msc2`, `/run/msc2` in installed data/secret/ownership paths | Existing worlds, configuration, credentials, service metadata, ownership markers and socket locations stay discoverable. These are not new user-facing names. |
| `.msc2-content`, `.msc2-playit`, `.msc2-broadcast`, `.msc2-staging` and ownership markers | Persisted server-local content indexes, helper state and safe cleanup boundaries. Renaming the folder without migrating its indexes would orphan stored content. |
| The deployed `agent/` resource/build directory, `agent.log`, indicator lock/control filenames and `local-agent` host identity | Old and new installers, running services, stored host-scoped preferences and rollback processes must agree on these locations and identities. Source directories, modules and tool filenames use engine. |
| `MSC2_*` environment settings and service metadata comments | Existing installed service definitions, headless automation, configured signing secrets and controlled verification inputs continue to work. This step does not silently change an administrator's configuration contract. |
| `msc2_<id>_<secret>`, bootstrap proof domains and local CLI pipe names | Credential and authenticated local protocol continuity. A branding change must not invalidate credentials or change the signed proof format. |
| `agentVersion`, `agentHostId`, `agentState`, `agentId`, corresponding Rust fields, update compatibility-window fields, operation result fields, `playit_agent_id` and storage category `agent` | Existing wire/configuration shapes, stored snapshots, old hosts and old clients remain compatible. Interfaces explain them with current service terminology. |
| Private `--msc2-*` updater switches and `--agent-indicator` | An already-running older executable can launch its replacement and complete health checks, repair and rollback. These switches are not advertised as product terminology. |
| `msc2-update-manifest.json` and `.sig` | Existing clients discover and verify the same signed update metadata. New downloadable application/archive filenames use `msc-`; checksums and signatures cover those exact filenames. |
| Previous desktop executable/package names and install locations | Upgrade, repair, rollback and uninstall recognize the installation they actually find. New source/package identifiers use `msc-desktop-web`. |
| `ctemple9/msc2` in historical evidence links | Cameron renamed the repository to `ctemple9/MSC` on 2026-11-15. Current download/support links and updater defaults use that canonical address. Earlier installed clients still use the former address through GitHub redirects; historical evidence links retain their recorded spelling. |
| HTTP `User-Agent`, `ureq::Agent`, OpenSSH `IdentityAgent`, Playit keys/origin values/URLs, npm dependency names | External protocols and dependency APIs belong to their providers and must retain their real spelling. The user-facing SSH choice is **SSH key helper**. |
| `AGENTS.md` references | Required instruction filename. Its content and `CLAUDE.md` remain identical. Coding contributors are distinguished from the background service. |

Historical exceptions include audit/source inventories, corpus evidence, old
release notes, archived workflows, archived implementation records and vendored
third-party source/license material. Their names are evidence, not current
branding. Sample paths and fixtures that exercise a retained compatibility
identity remain in the inventory for the same reason as production code.

## Upgrade behavior and limits

Desktop startup copies old `msc2.*` browser preferences into `msc.*` before
application modules read them. Existing current-namespace values win. Old keys
are retained for recovery; an explicit client reset clears both namespaces.
Saved remote profiles accept the earlier manual-address field and SSH choice.
The application identifier is stable so the webview and native credential store
continue to locate the same data.

Linux package scripts recognize old owned command symlinks while installing the
new package's command. Service executable ownership checks and SELinux labeling
cover the new product directory and established previous directories. Windows
maintenance can locate either previous desktop executable when retaining a
rollback hash. No running service, installer or remote host is changed by this
repository task.

## Verification record

No tests were added or run, and no release was launched. Non-test verification
covers naming inventory, document links, generated API consistency, frontend
type checking/build, Rust formatting and compilation/linting, native Windows
installer-helper compilation, and script/configuration syntax inspection.
Completed non-test commands:

- `python tools/naming-audit.py` (exact compatibility inventory).
- `cargo check --workspace --locked` and `cargo clippy --workspace --locked`.
- `cargo check --locked --manifest-path clients/desktop-web/src-tauri/Cargo.toml`
  and the corresponding native desktop Clippy command.
- `cargo check --locked --tests --manifest-path clients/desktop-web/src-tauri/Cargo.toml`
  (compilation only; no test executed).
- `cargo check --locked --manifest-path packaging/windows/lifecycle-helper/Cargo.toml`.
- Rust formatting for the workspace and standalone desktop/helper manifests.
- In `clients/desktop-web`: `npm run check`,
  `npm run api:generate -- --check`, and `npm run build`.
- Read-only document-link inspection (zero newly broken Markdown links),
  Python AST and shell syntax inspection (104 files), PowerShell parser
  inspection, JSON parsing (128 files), release-workflow YAML parsing, and
  staged whitespace inspection.

Compiler/linter warnings remain in pre-existing platform-dependent code; the
frontend reports ten existing unused-selector warnings plus existing bundling
warnings. No unrelated warning fixes were included.

The additional `cargo check --workspace --all-targets --locked` compilation
stopped at a pre-existing inference error in the unchanged
`crates/msc-application/tests/pack_update_comparison.rs:37` (E0282: the map key's
string type is not explicit). That file is byte-identical to the starting
revision and is outside the rename. This broader check did not pass; no test
was executed. Production workspace and native desktop checks passed.

Cameron's physical acceptance remains pending: fresh installation and upgrade
from an existing version on Windows/macOS/Linux; repair, rollback and uninstall;
saved servers/hosts/credentials/preferences; desktop/TUI service naming; CLI
help and service controls. Cross-platform source inspection and a Windows
build do not establish physical macOS/Linux acceptance.


## Repository address follow-up (P26.2)

Cameron confirmed renaming the GitHub repository and changing his Git remote on
2026-11-15. The current address is `https://github.com/ctemple9/MSC`; the local
remote was inspected and already uses that address. Current desktop/CLI updater
defaults, uninstall recovery configuration, installer support links and README/
security links now point directly to the canonical repository. Existing update
manifest filenames, signing keys and redirect-host restrictions are unchanged.

Read-only GitHub requests returned HTTP 200 for the new and former repository
addresses and each updater's `/releases?per_page=100` endpoint. Both returned
38 releases, headed by v0.1.44. The v0.1.44 manifest (3,081 bytes) and detached
signature (89 bytes) were downloaded successfully through both repository
addresses, ending on the already-allowed GitHub release asset host. This checks
metadata access and redirection, not a complete installed-client update or a
new signature-verification result. `/releases/latest` returns 404 for both
addresses because the published builds are prereleases; the updater uses the
release-list endpoint instead. No installer was downloaded or executed.

The original P26.1 commit was already published, so this follow-up is a separate
commit rather than a rewrite of shared history. No push, test suite or release
publication is part of the follow-up.
