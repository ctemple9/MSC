# MSC — Rolling Plan

## Phases

This table is the quick map; detailed steps and historical findings are linked from the [rolling plan archive](rolling-plan-archive.md). Phase gates and scope are defined in the [port plan](msc-port-plan.md).

| Phase | Name | Current disposition |
|---|---|---|
| Setup | Repository and project documents | Complete |
| 0 | Baseline and harness | Complete |
| 1 | Domain types and pure rules | Complete |
| 2 | API contract and operation model | Complete |
| 3 | Safety substrate | Complete |
| 4 | Java lifecycle | Complete |
| 5 | Configuration and migration | Complete |
| 6 | Worlds and backups | Implemented; historical records archived |
| 7 | Server families and provisioning | Implemented; historical records archived |
| 8 | Mods, plugins and modpacks | Implemented; historical records archived |
| 9 | Networking and helpers | Implemented; historical records archived |
| 10 | Bedrock runtimes | Implemented; historical records archived |
| 11 | Desktop clients | Implemented; historical records archived |
| 12 | Client redesign | Complete |
| 13 | Original terminal UI | Retired by D-034 |
| 14 | Server time and scheduling | Historical work archived |
| 15 | Server state and interaction fixes | Historical work archived |
| 16 | Release safety and codebase readiness | Owner reports the tracked follow-up work resolved; historical acceptance packet retained |
| 17 | Local CLI refinement | Historical work archived |
| 18 | 3D world map | Owner reports tracked map follow-up resolved; acceptance history archived |
| 19 | Complete local uninstall | Implementation records archived |
| 20 | Faster native file imports and uploads | Implementation records archived |
| 21 | Storage inventory and management | Implementation records archived |
| 22 | Restored full-screen TUI | Accepted by Cameron 2026-10-30; implementation records archived |
| 23 | Repository presentation, engine indicator and release follow-ups | Complete; records archived at Cameron’s request 2026-11-05 |
| 24 | Modpack management and content exports | Complete by owner direction; steps archived; illustrated guides deferred; independent gate review pending |
| 25 | Windows spot fixes | Active; owner verifies each issue before commit |
| 26 | Product and background service naming | P26.1 implemented; owner verification pending |

## Current work

P26.1 is the owner-authorized, single-commit naming migration recorded below.
Implementation is complete with physical owner verification pending. This does
not close Phase 25 or imply a phase gate review or release.

Phase 25 follows Cameron's November 10 direction: implement only reported
issues, preserve other platforms, and leave each fix uncommitted until Cameron
confirms it works. This overrides the usual commit-before-verification order. Phase 24's completed steps are in the
[archive](rolling-plan-archive.md#phase-24--modpack-management-and-content-exports).
Illustrated launcher guides (P24.20) are deferred by Cameron. Independent gate
review and unrecorded physical evidence remain tracked in the
[acceptance record](addons/phase24-acceptance.md).

Owner acceptance - 2026-11-11: Cameron requested marking P25.1-P25.5 done
and committing them after the performance recheck. Earlier pending/uncommitted
notes below describe the implementation history. This acceptance supersedes
those statuses without claiming unrecorded platform or scenario checks passed.
The original thumbnail commit is retained; P25.2-P25.5 receive separate commits.
No phase gate review, release or publication is implied.

## Phase 25 ? Windows spot fixes

### P25.1 — allow provider catalog images in the desktop app

**Status:** DONE - owner accepted and authorized commit on 2026-11-11.

**Files:** `clients/desktop-web/src-tauri/tauri.conf.json`; `docs/msc/rolling-plan.md`.

**What:** Owner requested fixing blocked Modrinth and CurseForge images after
Windows Intel 0.1.44 first-server setup showed missing datapack thumbnails.
Added Modrinth's HTTPS CDN and HTTPS CurseForge CDN subdomains to the desktop
image allowlist, retaining every existing source and all other security
directives. This shared desktop configuration applies on Windows, macOS and
Linux; no platform-specific behavior or UI code changes. The existing
CurseForge media host was already allowed, so this addresses blocking on other
CDN subdomains rather than claiming all CurseForge images were blocked.
Owner follow-up November 11: Modrinth thumbnails appeared. CurseForge image
verification remains pending because its API key was not configured; see P25.3.
No tests added or run; JSON formatting and diff inspection are the relevant
non-test checks. Physical thumbnail/gallery verification remains pending.

**Verify:** `cd clients/desktop-web; npx tauri dev` — in the native app, search
Modrinth and CurseForge datapacks during creation, then inspect project
thumbnails and galleries; repeat representative mod/modpack searches. Confirm
existing player images still load. Windows owner verification and macOS/Linux
physical regression checks remain pending. The installed 0.1.44 app needs a
rebuilt native shell to receive this configuration change.

**Batch:** This issue only; stop for owner verification.

**Commit:** Previously committed prematurely as `P24.45: allow modrinth and curseforge catalog images` (`c8133dda`); history retained. Reclassified as P25.1 by owner direction. Owner accepted this step on 2026-11-11; original commit history retained.


### P25.2 - repair an existing Windows desktop engine for the current app build

**Status:** DONE - owner accepted and authorized commit on 2026-11-11.

**Files:** `crates/msc-engine/build.rs`; `clients/desktop-web/src-tauri/src/lib.rs`; `clients/desktop-web/src-tauri/src/windows_service.rs`; `clients/desktop-web/src-tauri/src/windows_service.ps1`; `packaging/windows/desktop-service-lifecycle.ps1`; this plan.

**What:** Repair previously called first-install registration, which refuses an
existing service. Route Windows repair through the existing protected service
coordinator to replace the owned executable bundle and explicitly start it.
Retain the service account, stored password, environment, data directory and
startup policy. Validate account/data/port and desktop ownership before changing
anything; retain the graceful maintenance handshake and rollback record. A
legacy desktop build can be repaired to a different build after its digest and
standard owner data directory are validated. Marked headless installs remain
independent. Transport the embedded elevated script compressed in memory to
stay within Windows command-line limits without an elevated temporary script.
Resolve the machine-staged binary after repair. macOS/Linux service paths are
unchanged. No tests added or run; no live service repair performed by the engine.
PowerShell parser checks and `git diff --check` passed; Rust formatting applied
to the changed files. `cargo clippy --manifest-path
clients/desktop-web/src-tauri/Cargo.toml --locked --lib -- -D warnings` stopped
on the pre-existing `clippy::needless_return` in `installing_user` at
`src/lib.rs:1553`; an infrastructure dead-code warning was also reported.
No unrelated lint fixes made; no clean-Clippy claim. Owner verification pending.

**Verify:** `cd clients/desktop-web; npx tauri dev` ? close the installed release
app first, click Repair engine in the development app and approve Windows UAC.
Confirm Engine connected, existing servers/settings remain, and Modrinth and
CurseForge images load (P25.1). Repair again to confirm repeat use. The two apps
share one engine; different builds require repair when switching between them.
Owner physical verification remains pending.

**Batch:** P25.2 only; owner has now authorized commit.

**Commit:** `P25.2: repair existing windows desktop engine builds`

**Owner verification follow-up:** Cameron approved UAC but the app returned to
unavailable. Windows Application Error events identify `0xc00000fd` (stack
overflow) in the newly staged debug engine, at `__chkstk` according to its PDB.
The service path was restored by rollback. Inspection of the debug PE reports
a 1 MiB stack reserve and an unoptimized TUI future function frame of 2,749,624
bytes, which exceeds that reserve. Added `crates/msc-engine/build.rs` to reserve
16 MiB of stack address space only for Windows MSVC debug engine executables;
pages are committed on demand. Release engines and other platforms keep their
existing linker settings. This is a same-step repair; P25.2 remains uncommitted
and owner verification has not passed.

Follow-up non-test verification: `cargo build --locked -p msc-engine` passed
with existing warnings. PE header inspection confirms the rebuilt debug
executable reserves 16,777,216 bytes of stack. No service launch or tests were
run; Cameron must restart `npx tauri dev` to stage it, then retry Repair engine.

### P25.3 — show the existing CurseForge key form for catalog searches

**Status:** DONE - owner accepted and authorized commit on 2026-11-11.

**Files:** `crates/msc-engine/src/routes/components.rs`; `clients/desktop-web/src/lib/sections/components/ProviderCatalogBrowser.svelte`; this plan.

**What:** Owner screenshot shows the creation wizard's plugin browser reporting
no CurseForge key with only Retry. Its existing key-entry form requires the
`missing_curseforge_api_key` response code, but the shared catalog error mapper
returned `provider_unavailable` for every provider error. Return the specific
missing-key code with HTTP 409, matching the world-pack route convention. Keep
other provider failures unchanged. Reuse the existing form, engine secret storage,
permission checks and retry-after-save behavior; no UI redesign or platform
branch changes. No tests added or run. `cargo fmt --all --check`,
`cargo check --locked -p msc-engine` (existing warnings only), and
`git diff --check` passed.

**Verify:** `cd clients/desktop-web; npx tauri dev` — repair the engine to stage
the rebuilt source, then open Add Server → Add-ons → Browse plugins → CurseForge
without a configured key. Confirm the existing password field, console link and
Save key control appear. Save a key and confirm results/thumbnails load without
losing staged selections. Confirm Modrinth still browses normally.

**Batch:** P25.3 only; owner has now authorized commit.

**Commit:** `P25.3: expose curseforge key entry for catalog searches`

P25.3 owner UI follow-up: screenshot confirms the missing-key form now appears.
Removed the redundant API key focus link and explanatory paragraph at Cameron's
request; retained the console link, password field, Save key action and existing
permission handling. Key saving/catalog results still await owner verification.
Prettier check passed. No commit or tests. This frontend-only adjustment updates
in the running development app without another engine repair.


### P25.4 - use the creation draft for World step context

**Status:** DONE - owner accepted and authorized commit on 2026-11-11.

**Files:** `clients/desktop-web/src/lib/sections/fleet/wizard/WorldStep.svelte`;
`clients/desktop-web/src/lib/sections/worlds/WorldSettingsForm.svelte`;
`crates/msc-engine/src/routes/capabilities.rs`; this plan.

**What:** Owner selected Purpur and installed/selected Java 25, but World showed
Paper and host-default Java 21. World omitted the selected runtime path from its
capability request and refresh key. Include it, snapshot edition/flavor/runtime
across asynchronous requests, and reject responses for a different edition or
flavor instead of displaying incorrect context. The current source already sent
the flavor; its reported mismatch was not independently reproduced. Prevent
explicit creation contexts from inheriting missing flavor/loader/type values
from the existing active server. Ordinary active-server capability requests
retain their fallback behavior; Bedrock sends no Java runtime context. No tests
added or run. No Java installation deleted by the engine.

**Verify:** `cd clients/desktop-web; npx tauri dev` ? repair the rebuilt engine,
create Purpur with latest Minecraft, install/select Java 25, and enter World.
Confirm Purpur and available Java runtime; go back, change flavor/version and
select its compatible runtime, then confirm refreshed context. Check Bedrock
shows Bedrock without Java context and an existing server's settings still use
that server. Owner verification pending.

**Batch:** P25.4 only; owner has now authorized commit.

**Commit:** `P25.4: bind world context to creation selections`

Retry preparation: inspected the managed Windows installation at
`C:/Users/Cameron/AppData/Local/MSC2/runtimes/temurin-25-windows-x64`.
Owner can remove that exact directory after closing the wizard and stopping
any Minecraft server using it; no other Java installation is targeted.

P25.4 context also displays the detected Java major beside availability, so the
owner can verify Java 25 directly. `cargo check --locked -p msc-engine` passed
with existing warnings. Frontend type-check and formatting verification recorded
below. No runtime removal or tests performed.

P25.4 final frontend checks: Prettier passed; `npm run check` passed with
0 errors and 10 existing unused CSS warnings. Owner verification pending.


### P25.5 - avoid repeated Windows machine-build hashing during status refresh

**Status:** DONE - owner accepted and authorized commit on 2026-11-11.

**Files:** `clients/desktop-web/src-tauri/src/lib.rs`; this plan.

**What:** Owner reports desktop lag, Windhawk popups and local status network
failures. Read-only sampling found desktop using 25.5 CPU seconds over two wall
seconds and about 1.4 GiB working memory; engine service remained running and
healthz returned 204. Windhawk was idle during that sample; no causal claim for
its popups. Windows expected-engine lookup reread and hashed all three executables
on every guarded API request. Cache successful validation of the immutable,
administrator-protected machine build per desktop process, serialize first
verification, and revalidate when the expected build path changes. Still query
actual service identity for every lookup; failures are not cached. Other
platform paths unchanged. No tests or live operations run.

**Verify:** `cd clients/desktop-web; npx tauri dev` ? after closing the old app,
check local server status refreshes reliably and desktop CPU/memory settles;
retry after a Repair to a new build. Windhawk popup cause remains unconfirmed.

**Batch:** P25.5 only; owner has now authorized commit.

**Commit:** `P25.5: cache protected windows engine build verification`

P25.5 checks: Rust formatting and `git diff --check` passed; native desktop
`cargo check --manifest-path clients/desktop-web/src-tauri/Cargo.toml --locked
--lib` passed with an existing infrastructure dead-code warning. Owner
performance/network verification remains pending.

P25.5 accepted recheck: desktop used 0.11 CPU seconds over 3 seconds and 73 MB
RAM; engine was running at 53 MB and healthz returned 204. About 9 GB RAM was
free. This confirms lower load in that sample; Windhawk causality and sustained
status-refresh reliability were not independently established.

Commit preparation: workspace formatting passed. Engine and native desktop
Clippy completed with pre-existing warnings (including needless returns); no
unrelated warning fixes, tests, release actions or platform-pass claims.


### P25.6 - fix Windows updater permissions and indicator lock status

**Status:** Implemented; owner verification pending. Cameron explicitly
authorized fixing both reported issues and committing them together on
2026-11-15, overriding the usual Phase 25 verify-before-commit order for this
step only. No acceptance or phase closure is implied.

**Files:** `clients/desktop-web/src-tauri/src/update_windows.rs`;
`clients/desktop-web/src-tauri/src/engine_indicator.rs`; this plan.

**What:** The updater walks the executable and its ancestors to check ownership
and write/delete/permission-change rights. It incorrectly rejected an
Authenticated Users rule on `C:\` marked InheritOnly: that rule describes child
objects and grants no access to the inspected ancestor itself. Skip only such
inheritance-only entries. Continue checking every path's trusted owner and all
applicable inherited/explicit allow entries; retain rejection of writable
executables, deletable ancestors and untrusted owners. The same production
check is used before scheduling and before an elevated helper runs.

Factor the existing script builder so one essential Windows regression can
exercise the production decision with four in-memory permission scenarios:
inheritance-only allow, executable-write denial, ancestor-delete denial and
untrusted-owner denial. This protects the observed update failure and the
security boundary existing tests missed. It uses synthetic Windows security
objects and a process-local Get-Acl replacement; no filesystem ACL changes,
elevation, network, sleeps or installed app/service operations. Expected runtime
under 10 seconds excluding compilation. Test added, not run.

**Verify:** `cargo test --manifest-path clients/desktop-web/src-tauri/Cargo.toml --lib windows_desktop_regression_`
- Cameron runs the two focused regressions on Windows; verify two tests run.
Full installed-update acceptance requires a build containing this change.

**Batch:** These two owner-reported Windows fixes only; one commit as requested.

**Commit:** `P25.6: fix Windows updater permissions and indicator lock status`.

The installed 0.1.43 app, staged 0.1.44 MSI, machine permissions and service were
not modified. Fixing a future target build alone cannot change the permission
check already running inside 0.1.43; replacing that desktop with a corrected
build requires a separate manual installation/build step, not an automatic
release or live-service action in this task.

The second fix recognizes fs2's platform-specific contention error as a running
indicator, alongside the existing WouldBlock handling. Windows returns error 33
when the indicator legitimately owns its coordination-file lock; that is not a
failed service action. Other lock errors still propagate. Keep the lock held
while inspecting status; do not stop or restart the indicator or service.

A second essential regression uses a unique temporary file and separate handles
to verify missing/unlocked => not running, held lock => running, released lock =>
not running. This catches the exact real Windows contention error existing
coverage missed. It uses no installed paths, separate processes, network or
sleeps; expected runtime below 100 ms excluding compilation. Cleanup removes
only its own file and temporary directory. Test added, not run.

P25.6 non-test checks: native desktop `cargo clippy --locked --manifest-path
clients/desktop-web/src-tauri/Cargo.toml --lib`, `cargo check --locked
--manifest-path clients/desktop-web/src-tauri/Cargo.toml --tests` (compile only),
Rust formatting, production-script/fixture PowerShell parsing,
`python tools/naming-audit.py` and `git diff --check` passed. Existing
infrastructure dead-code and desktop needless-return warnings remain. No test
was executed and no installed application, service or Windows ACL was changed.

## Phase 26 — Product and background service naming

### P26.1 — rename the product and background service throughout the repository

**Status:** Implemented; owner verification pending. Cameron authorized autonomous
execution and one commit for the complete change on 2026-11-15. This record does
not close the phase gate or claim physical upgrade acceptance.

**Files:** Current source, desktop/TUI/CLI copy, manifests, build/release tools,
packaging, content, fixtures, current documentation and links; `docs/msc/` replaces
the former documentation root; `crates/msc-engine/` names the Rust service.
`AGENTS.md` and `CLAUDE.md` stay synchronized. See `naming-migration.md` and its
compatibility inventory for exact retained identities and limitations.

**What:** Use Minecraft Server Controller (MSC) for the product and MSC Background
Service in the interfaces, with engine internally. Preserve installed identities,
data/secret locations, wire fields and the private updater protocol. Copy old
browser preferences into the new namespace without overwriting current values.
Keep old Linux package locations recognizable and old Windows desktop executables
available to rollback discovery. Preserve signing keys, the full artifact set,
checksums and update metadata; add no release gates. No tests added or run.

**Verify:** `python tools/naming-audit.py` — read-only static naming inventory.
Owner manual checks: desktop/TUI service pages and CLI help use current names;
existing servers, saved hosts, credentials and preferences remain available;
fresh install, update, repair, rollback and uninstall retain ownership on all
supported platforms. Implementation checks are recorded in the migration document;
physical macOS/Linux and installer acceptance remain owner verification.

**Batch:** One owner-authorized autonomous step; one commit for all changes.

**Commit:** `P26.1: rename product and background service throughout repository`


### P26.2 ? follow the owner-renamed GitHub repository

**Status:** Implemented; owner verification pending. Owner confirmed the remote
rename and local remote update on 2026-11-15. The already-published P26.1 commit
is preserved; this bounded follow-up has one commit.

**Files:** README/security links; desktop bundle homepage; Windows installer
support links; desktop/CLI updater defaults; uninstall recovery configuration;
update fixture repository; naming migration record and compatibility inventory;
this plan.

**What:** Use `ctemple9/MSC` for current repository references. Preserve historical
links, private updater switches, signing configuration and manifest filenames.
Read-only GitHub inspection confirmed that both repository addresses expose the
same 38 releases and both serve the v0.1.44 update manifest/signature. These
requests do not establish full installed-client upgrade acceptance. No tests,
installer execution, push or release publication.

**Verify:** `python tools/naming-audit.py` ? retained historical/compatibility
spellings match their inventory; the new canonical address is used by current
download/support links and updater defaults. Non-test compile/lint and syntax
checks are recorded below after completion.

**Batch:** This repository-address follow-up only.

**Commit:** `P26.2: update links and updater defaults for renamed repository`

P26.2 checks: `python tools/naming-audit.py`, workspace/native desktop Rust
format checks, workspace/native desktop `cargo clippy --locked`, desktop JSON
and installer XML parsing, and `git diff --check` passed. Clippy reports the
existing platform-dependent dead-code/unused/needless-return warnings. No tests
were run. Live requests verified old/new repository metadata, the updater's
release-list endpoint, and both v0.1.44 manifest/signature download addresses;
no installer or installed-client update was executed.


### P26.3 - prepare and publish v0.1.45

**Status:** Release candidate prepared; publication requested by Cameron on
2026-11-16. Physical owner verification and phase gates remain pending.

**Files:** Engine/desktop package manifests and lockfiles; static/shared bundle
identity and its existing fixture; README; `release/v0.1.45.md`; this plan.

**What:** Synchronize product versions at 0.1.45, update current download/help
references and document the manual Windows upgrade needed when an older running
updater rejects the standard protected installation. Include committed naming,
Windows service, provider/catalog and creation fixes since v0.1.44. Publish via
the existing build-only four-runner release workflow, retaining all nine
packages, checksums, signing/notarization and signed update metadata. No new
workflow gates, tests, live local installation or tag movement.

**Verify:** `gh release view v0.1.45 --repo ctemple9/MSC` - inspect the published
prerelease, nine platform packages, checksum file, release notes and signed
metadata. Publication itself is not owner physical acceptance.

**Batch:** This version/README/release step only. Monitor the first release
attempt; inspect all failures before deciding on any further costly attempt.

**Commit:** `P26.3: prepare v0.1.45 release and update readme`.

P26.3 preflight: all engine/desktop/shared/static/lockfile version identities
agree at 0.1.45. Workspace/native desktop formatting and Clippy passed with
existing warnings; the frontend build and static bundle identity passed.
Release YAML parses with only the existing build/publish jobs; all eight
referenced packaging tools resolve. Naming inventory and whitespace checks
passed. Existing signing secret/variable names remain configured. The previous
v0.1.44 release workflow completed successfully. No tests or installer/service
operations were run locally. The release tag is new and must not be moved.


### P26.4 — bootstrap Linux source development without a release package

**Status:** Implemented; owner verification pending.

**Files:** Linux service platform, native desktop helper lookup, desktop engine
preparation, `tools/bootstrap-linux-dev.py`, README and this plan.

**What:** Provision a dedicated root-owned development helper from the local
engine build through an explicit pkexec authorization when no installed helper
exists. Do not overwrite package helpers or install/start a service during
preparation. Preserve old package path lookup, protect destination ancestry,
install atomically, and label the bootstrap executable for SELinux. Correct
the service executable label rule to cover deployed `agent/` paths and both
current and previous product directories. Keep service registration/repair in
the existing explicit app action. No tests added or run; no live bootstrap,
service changes, package installation or release operations performed.

**Verify:** `cd clients/desktop-web && npx tauri dev` — authorize the one-time
helper setup on a machine without an MSC package, then use Install/Repair engine
and confirm local connection; repeat after an engine change with Repair engine.
Also confirm an existing old-path installation remains discoverable.

**Batch:** This Linux development fix only, one commit as requested.

**Commit:** `P26.4: bootstrap linux development service helper`

Checks: targeted Linux platform and native desktop `cargo clippy --locked`
passed with existing unused-variable/dead-code warnings. Rust formatting, Node
syntax, Python syntax and whitespace checks passed. No tests run. Physical
verification and administrator authorization remain with Cameron. The separately
changed desktop Cargo lockfile (package ordering only) was left out of this step.
