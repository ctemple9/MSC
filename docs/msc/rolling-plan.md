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


### P26.5 — shorten the sidebar engine button

**Status:** Implemented; owner visual verification pending.

**Files:** `ControlSidebar.svelte`; this plan.

**What:** Cameron approved the compact sidebar label **Engine** in place of
**Background Service** on 2026-11-16 so the adjacent Start button no longer
stretches to accommodate a wrapped label. This is a sidebar-label exception to
the user-facing Background Service naming contract; other labels stay as they are.

**Verify:** `cd clients/desktop-web && npx tauri dev` — confirm Engine fits on
one line next to Start and still opens the Background Service screen.

**Batch:** This sidebar label only.

**Commit:** `P26.5: shorten sidebar engine button label`

Checks: frontend `npm run build` passed with existing CSS/import/chunk warnings;
`git diff --check` passed. No tests added or run.


### P26.6 — reuse memory allocation in Configure and its guided tour

**Status:** Implemented; owner verification pending.

**Files:** Shared RAM fields; Edit Server General tab; wizard Configure,
Confirm and draft/request model; onboarding guide, source map, anchors and
corpus expectations; create API contract/generated types/DTO; engine create
route and CLI request construction; existing API conformance case; this plan.

**What:** Reuse the exact Edit Server minimum/maximum RAM controls, units,
step size and styling in Configure. Keep edits in the wizard draft through
Back/Continue and show them in Confirm. Preserve existing 2/4 GB defaults
(3/6 GB for modded Java) until edited; reject empty, negative and non-finite
values using the existing editor's numeric rules. Optional minRamGB/maxRamGB
creation fields apply only to the newly created record before registration,
including Java, staged-pack and Bedrock paths; omitted fields retain existing
engine defaults. Existing CLI/TUI callers continue to omit them. Edit Server's
autosave behavior remains in its parent, while creation makes no RAM mutation
until Create.

Add an engine-owned Memory tour card after Crossplay and before Xbox Broadcast,
in the same order as the Configure section. Anchor the real fields and reuse
the standard informational Next card, scrolling and spotlight; no mandatory
edit, extra dialog or change to Configure's Continue/Java-selection behavior.
Record the owner-requested extension honestly in the guide source map rather
than inventing an MSC 1 tour source. Update the existing corpus expectation to
41 steps. Extend the existing API conformance input with fractional RAM values;
no new tests or redundant coverage and no additional runtime. Tests not run.

**Verify:** `cd clients/desktop-web && npx tauri dev` — restart the tour, choose
Start Fresh, and confirm Memory follows Crossplay (or the Bedrock fields),
before Xbox Broadcast/settings review. Adjust RAM, move forward/back, create,
and confirm Edit Server reads the chosen pair. Check standard/modded defaults,
empty-value blocking, and Edit Server's existing autosave form.

**Batch:** This Configure memory and onboarding slice only.

**Commit:** `P26.6: reuse ram allocation in server creation and onboarding`

Checks: `npm run check` passed (0 errors, existing CSS warnings);
`npm run build` passed with existing CSS/import/chunk warnings. Engine
`cargo clippy --locked -p msc-engine`, Rust formatting, touched frontend
formatting, JSON parsing and `git diff --check` passed. The explicit CLI
request initializer was updated after the initial compile caught its missing
optional fields. No live server creation or tests performed. The separate
desktop lockfile change remains outside this commit.


### P26.7 — simplify first-world generation choices

**Status:** Implemented; owner visual and creation verification pending.

**Files:** Shared WorldSettingsForm wizard presentation; onboarding guide;
this plan.

**What:** Owner-approved simplification of the Add Server first-world form:
keep World Type, Generate Structures (Java), and Bonus Chest, retaining existing
edition/capability restrictions and the Bedrock Starting Map option. Offer
Default, Superflat, Large Biomes and Amplified on Java; Default/Flat on Bedrock.
Remove Single Biome from the wizard choices because its custom biome field is
no longer offered. Hide Minecraft Folder Name, Flat Preset JSON, Biome Source,
Generator Options and raw Data Pack IDs in this creation flow. Packs browsing
and import remain in the dedicated Packs section. Do not add an Advanced panel.
Selecting Java Superflat supplies one bedrock layer, two dirt layers, one grass
layer and a plains biome automatically; selecting another type clears the
wizard's custom generator settings. The existing Bedrock flat preset is owned
by the engine. Retain world name, seed, difficulty and game mode in Essentials.
Keep the shared form's existing Worlds create/edit controls and stored/custom
profiles available outside the Add Server wizard. Update the existing World
Generation tour explanation in place, preserving card order and anchors.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Add Server → Start Fresh
→ World → World Generation: confirm only the simple edition-appropriate
controls appear, and Essentials/Packs remain available. Select Superflat,
continue/back and confirm the selection persists; create it and verify standard
grass terrain. Switch Superflat back to Default before creation and confirm
normal terrain. Repair engine and restart the tour to read the updated card.

**Batch:** This creation-form simplification only.

**Commit:** `P26.7: simplify world generation in server creation`

Checks: frontend `npm run check` passed (0 errors, existing CSS warnings);
`npm run build` passed with existing CSS/import/chunk warnings. Touched Svelte
formatting, Rust format check, engine Clippy, onboarding JSON parsing and
`git diff --check` passed; Clippy reports existing warnings. No tests added or
run, and no live world or service changes. Separate desktop lockfile change
remains outside this step.


### P26.8 — align the creation world-generation controls

**Status:** Implemented; owner visual verification pending.

**Files:** WorldSettingsForm wizard layout; this plan.

**What:** Give World Type the full available width in the creation wizard.
Stack Generate Structures and Bonus Chest beneath it with both labels aligned
left, switches aligned right and descriptions below. Reuse the existing inline
toggle layout with consistent spacing, retaining the responsive single-column
flow and onboarding anchor. No setting, default or creation behavior changes.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Add Server → World →
World Generation; check the full-width selector and aligned switch rows at
normal and narrow window widths. Confirm switches and type selection still work.

**Batch:** This generation layout adjustment only.

**Commit:** `P26.8: align world generation controls in creation`

Checks: `npm run check` passed with 0 errors and existing CSS warnings;
touched Svelte formatting and `git diff --check` passed. No tests added or run.


### P26.9 — align creation gameplay rules with world generation

**Status:** Implemented; owner visual verification pending.

**Files:** WorldSettingsForm wizard layout; this plan.

**What:** Reuse the creation-only single-column layout for Gameplay Rules.
Hardcore, Allow Commands and edition-specific switches now use full-width
rows with labels left and switches on the same right edge as World Generation.
Separate the existing gamerule editor with a quiet divider and an Additional
Rules label, keeping version information and editor actions aligned below.
Keep rule editing, capability restrictions, onboarding anchors and saved values.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Add Server → World →
Gameplay Rules; confirm switches line up with World Generation above and the
Additional Rules editor fits normal and narrow windows. Check Add Rule and
Edit as Text still open and preserve entered rules.

**Batch:** This Gameplay Rules layout adjustment only.

**Commit:** `P26.9: align gameplay rules in server creation`

Checks: `npm run check` passed (0 errors, existing CSS warnings); touched
Svelte formatting and `git diff --check` passed. No tests added or run.


### P26.10 — remove generation switch descriptions

**Status:** Implemented; owner visual verification pending.

**Files:** WorldSettingsForm wizard copy; this plan.

**What:** Remove the explanatory sentences below Generate Structures and Bonus
Chest in the creation wizard. Both switches now occupy single rows with the
same 18px spacing as Gameplay Rules. Keep the World Type hint and existing
capability/lifecycle notices.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Add Server → World;
expand World Generation and Gameplay Rules and compare the switch-row spacing.

**Batch:** This description removal only.

**Commit:** `P26.10: remove generation switch descriptions`

Checks: `npm run check` passed with 0 errors and existing CSS warnings;
touched Svelte formatting and `git diff --check` passed. No tests added or run.


### P26.11 — collapse Essentials and guide its expansion

**Status:** Implemented; owner visual and tour verification pending.

**Files:** WorldSettingsForm, TourOverlay action map, onboarding guide/source
map and corpus expectation; this plan.

**What:** Present Essentials as a closed-by-default native disclosure using
the same summary, chevron, spacing and body layout as the other world sections.
Keep all field bindings and the existing onboarding anchor. Insert Open Essentials
immediately before the existing Essentials review card; clicking the real
section advances to review, using the same action behavior as World Generation
and Gameplay Rules. Preserve the existing Okay/hide-card flow for editing fields.
Record the new disclosure's owner-directed provenance; update the tour count
to 42. No automatic expansion, setting changes or extra UI.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Add Server → World;
confirm Essentials starts collapsed, opens/closes and retains field edits.
Repair engine and restart the tour: verify Open Essentials precedes Essentials,
expanding advances to review, and Okay allows editing before the generation card.

**Batch:** This Essentials disclosure and onboarding adjustment only.

**Commit:** `P26.11: collapse world essentials and guide expansion`

Checks: `npm run check` passed (0 errors, existing CSS warnings), and
`npm run build` passed with existing CSS/import/chunk warnings. Touched frontend
formatting, onboarding JSON parsing and `git diff --check` passed. No tests
added or run. Separate desktop lockfile change remains outside this step.


### P26.12 — simplify world settings context copy

**Status:** Implemented; owner visual verification pending.

**Files:** WorldSettingsForm; this plan.

**What:** Remove the world-versus-server ownership paragraph as requested.
Rename the context heading and accessible label to Settings Context, rendered
in title case. Use the existing software display-name formatter so Paper,
Fabric, NeoForge and other known flavors have their established capitalization.
Remove the redundant context bottom border so Essentials supplies the single
divider below it. No setting, runtime value or capability behavior changes.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Add Server → World;
confirm Settings Context shows a capitalized software name and the removed
paragraph no longer separates the context from Essentials.

**Batch:** This world-form copy change only.

**Commit:** `P26.12: simplify world settings context copy`

Checks: `npm run check` passed (0 errors, existing CSS warnings); touched
Svelte formatting and `git diff --check` passed. The subsequent redundant-border
removal was inspected in the diff. No tests added or run.


### P26.13 — confirm leaving onboarding before creator Back

**Status:** Implemented; owner interaction verification pending.

**Files:** Onboarding shared state, TourOverlay and AddServerWizard; this plan.

**What:** Defer the creator's Back action while the guided tour is active.
Use the existing tour card presentation for “Do you want to exit onboarding?”
with Cancel and Yes, exit onboarding. Yes invokes the existing skip/completion
path (including its persistence/restart behavior) before going back exactly one
wizard page. Cancel leaves the wizard, values, tour index and prior hidden-card
state unchanged. Reveal the confirmation even on hidden tips and the popup-free
World review step; suppress tip-header actions and anchor advancement while it
is pending. Block background pointer interaction, focus Cancel, contain keyboard
focus, and treat Escape as cancellation. Clear deferred navigation if the
wizard or tour unmounts. Ordinary non-tour Back and creation/busy guards remain.

**Verify:** `cd clients/desktop-web && npx tauri dev` — restart onboarding and
press Back in Configure, Network, World and Confirm (including hidden tips and
World review). Cancel must keep page, input values and tour position; Yes must
end the tour and go back one page with values intact. Confirm Escape cancels,
Tab stays within the confirmation, and Back works normally outside onboarding.
Restart the tour afterwards to confirm it can be reopened normally.

**Batch:** This tour/creator Back confirmation only.

**Commit:** `P26.13: confirm onboarding exit before creator back navigation`

Checks: frontend `npm run check` passed (0 errors, existing CSS warnings);
`npm run build` passed with existing CSS/import/chunk warnings. After adding
keyboard focus containment, the type-check passed again. Touched frontend
formatting and `git diff --check` passed. No tests added or run; verify the
visible interaction manually rather than adding source-text or timer-sensitive
tests. The separate desktop lockfile change remains outside this step.


### P26.14 — group Crossplay and Xbox Broadcast before memory

**Status:** Implemented; owner visual and tour verification pending.

**Files:** ConfigureStep, onboarding guide; this plan.

**What:** Put the conditional Xbox Broadcast section immediately after the
Crossplay section and move Memory to the bottom of Configure. Preserve existing
Java Crossplay eligibility and Bedrock broadcast visibility. Swap the broadcast
and memory onboarding cards so the tour follows the visible form order; keep
anchors, card IDs and the existing conditional tour filtering.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Configure a Java server,
enable Crossplay and confirm Xbox Broadcast appears immediately underneath,
with Memory last. Toggle Crossplay off and confirm Memory remains last. Repair
engine and restart onboarding; check Crossplay → Xbox Broadcast (when enabled)
→ Memory → Review Settings, with no missing-anchor card when Crossplay is off.

**Batch:** This Configure and tour reorder only.

**Commit:** `P26.14: group broadcast before memory in server configuration`

Checks: `npm run check` passed (0 errors, existing CSS warnings); touched
Svelte formatting, onboarding JSON parsing and `git diff --check` passed.
No tests added or run.


### P26.15 — browse provider modpacks from the import creation path

**Status:** Implemented; owner interaction verification pending.

**Files:** Wizard UploadStep; this plan.

**What:** Shorten the native drop-zone label to Drop here. Add Browse Modpacks
alongside the local source pickers, reusing the existing PackCatalogSheet with
its Modrinth/CurseForge selector, project/version selection and key handling.
Do not apply a preselected software/version filter before a pack is chosen.
Pass the catalog's already-inspected staged archive into the same modpack
summary/draft path as local archives, retaining its originating host API and
wizard cleanup ownership. Existing Continue then follows Network → World →
Confirm and creates from the selected pack's manifest. Keep parent navigation
busy while the browser is open. Local folder/ZIP/archive actions remain available.
Update the introductory copy to explain both local and provider starting points.
No new browser or provisioning API. The existing guided tour stays on Start Fresh.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Add Server → Import or
Create from Modpack → Browse Modpacks; choose a Modrinth pack/version and confirm
the pack summary, detected loader/version and Network/World/Confirm flow.
Repeat browsing with CurseForge (including its existing missing-key prompt).
Cancel browsing to return to Upload; check local source pickers and Drop here.
Only create a server when ready for that manual verification.

**Batch:** This provider-browser entry point and drop copy only.

**Commit:** `P26.15: browse modpacks from the import creation flow`

Checks: `npm run check` and `npm run build` passed with existing CSS and
import/chunk warnings; touched Svelte formatting and `git diff --check` passed.
No tests or live provider downloads/server creation performed. Separate desktop
lockfile change remains outside this step.


### P26.16 — collect Java inline during fresh server configuration

**Status:** Implemented; owner interaction verification pending.

**Files:** New JavaSetupField; ConfigureStep, wizard controller/model; tour
anchors/filter/overlay and guide/source map/corpus count; focused Configure gate
regression cases; this plan.

**What:** Add a Java Executable path row to Configure after Source, with Detect,
Browse and Install Java actions patterned after Edit Server. Start blank and
keep the path scoped to this server's draft; do not save a host default. Resolve
the selected software/version's Minecraft release to show its required Java
major, pin the resolved release, and require a nonblank path for that current
setup before Continue. Leave Continue blocked while requirements are loading
or fail to resolve, with an inline error/retry. Keep paths when navigating back;
clear stale selections when software/version or host changes. Restore the
pinned version list on returning to Configure. Share the existing required-Java
version mapping with the retained picker. Detect opens an
explicit picker that disables older/unknown runtimes, Browse selects an
executable, and the existing installer populates the returned runtime path.
Manual paths retain the engine's final creation-time compatibility validation.

Fresh Configure Continue advances directly without the old automatic chooser.
If a later pack changes the runtime context, return to Configure instead of
opening that chooser. Preserve imported-pack runtime selection and the existing
creation-error recovery path. Add a Java-only tour card after Source and before
Crossplay, bringing the corpus to 43 steps. Temporarily hide the tour overlay
while Java helper sheets are open, restoring it afterwards without advancing.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Start Fresh → Java →
Configure: verify blank path disables Continue and required Java matches the
selected release. Use Detect/Browse/Install and confirm path population; type
or clear a manual path; continue directly to Network with no chooser. Back must
retain the path; changing software/version must require reselection. Check
Bedrock needs no Java, imported-pack selection remains available, and the
restarted tour handles the new Java card and helper sheets. Repair engine to
load the updated guide before restarting the tour.

**Batch:** This inline Java setup change only.

**Commit:** `P26.16: collect java executable in server configuration`

Essential test rationale: blank/whitespace paths or stale/unresolved runtime
choices must not enable Java Configure Continue; Bedrock must remain exempt.
Two focused cases exercise those public navigation decisions with a local draft,
no network, timers, filesystem or process dependence. Expected milliseconds;
not run. No additional test framework or duplicated screenshot/source assertions.

Checks: final `npm run check` passed (0 errors, existing CSS warnings);
`npm run build` passed with existing CSS/import/chunk warnings. Touched
frontend formatting, Rust format check, engine Clippy, guide JSON parsing and
`git diff --check` passed. No Java installation, live server creation or
automated tests performed. Separate desktop lockfile change remains outside
this step.


### P26.17 — explain the engine and control panel on the welcome page

**Status:** Implemented; owner visual verification pending.

**Files:** EngineSetupSection; this plan.

**What:** Replace How does MSC work? with the three owner-approved explanations:
the background service runs Minecraft wherever it is hosted; the desktop app
and terminal interface control the same engine; closing the window leaves the
engine and servers running, using the agreed TV remote analogy. End with the
approved Stop / Stop Engine instructions and “Or turn off the computer.”
Keep the numbered layout and use no hyphens/dashes in the new prose. Remove
the old duplicate closing-window explanation heading.

**Verify:** `cd clients/desktop-web && npx tauri dev` — open Engine and read
How does MSC work?; confirm the agreed text appears in the existing three-step
layout and the stopping instructions sit beneath it.

**Batch:** This Engine-page explanation only.

**Commit:** `P26.17: explain the engine and control panel on the welcome page`

Checks: final `npm run check` passed with 0 errors and existing CSS warnings;
touched Svelte formatting and `git diff --check` passed. Removed the unused
heading selector after replacing the old explanation. No tests added or run.


### P26.18 — shorten the engine explanation with optional detail

**Status:** Implemented; owner visual verification pending.

**Files:** EngineSetupSection; this plan.

**What:** Show the three concise owner-approved engine/control-panel/window-close
explanations in the existing numbered layout. Keep the full previous explanation
inside one closed-by-default More about the engine disclosure underneath, using
the page's existing disclosure styling. Keep stopping instructions visible below
it. No layout fillers, separate disclosures per step or service behavior changes.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Engine → How does MSC
work?; compare the compact explanation with More about the engine closed, then
expand it to read the preserved full copy. Stopping instructions remain outside
the disclosure. Check the column at normal and narrow window widths.

**Batch:** This explanation presentation only.

**Commit:** `P26.18: shorten engine explanation with optional detail`

Checks: `npm run check` passed (0 errors, existing CSS warnings); touched
Svelte formatting and `git diff --check` passed. No tests added or run.

### P26.19 — move manage to the sidebar and engine to the header

**Status:** Implemented; owner visual and onboarding verification pending.

**Files:** ApplicationShell; ControlSidebar; TopBar; ShellIcon; TourOverlay;
onboarding guide; this plan.

**What:** Replace the sidebar Engine button with Manage and remove Manage from
the server picker. Move Engine into the header as the third small icon, directly
after the console toggle, using a square E in the existing shell icon style and
button dimensions. Point the server-list onboarding card directly at Manage and
remove the old dropdown-specific card hiding behavior.

**Verify:** `cd clients/desktop-web && npx tauri dev` — check Manage beside
Start opens the server list, and the third header icon opens Engine with either
sidebar state. Start onboarding and click its highlighted Manage button; confirm
the tour advances to Create Server.

**Batch:** This navigation and matching onboarding change only.

**Commit:** `P26.19: move manage to sidebar and engine to header`

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings).
`npm run build` and `git diff --check` passed; existing build warnings remain.
Touched Svelte formatting passed. No tests added or run.

### P26.20 — automatically reveal active onboarding targets

**Status:** Implemented; owner walkthrough verification pending.

**Files:** TourOverlay; tourAnchors; this plan.

**What:** Replace the single delayed scroll attempt with ongoing visibility
checks in the existing tour geometry loop. Reveal each target inside its
scrollable ancestors with 16 pixels of breathing room, leaving already visible
targets stationary. This handles delayed mounting, changing fields, expanding
sections and resizing without step-specific fixes. Oversized sections reveal
their beginning. Suspend scrolling while the card is hidden, Java selection is
pending, or the exit confirmation is open. Use immediate scrolling to avoid
restarting animations during layout changes and respect reduced motion.

**Verify:** `cd clients/desktop-web && npx tauri dev` — walk through server
creation onboarding, including version, Java, memory and the world disclosures.
Each active target should automatically appear in the sheet content area.
Resize to a shorter window and confirm the current target is revealed again.
Hide a form tip and confirm the form can be freely scrolled.

**Batch:** Shared onboarding target visibility only.

**Commit:** `P26.20: automatically reveal active onboarding targets`

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings);
touched formatting and `git diff --check` passed. No tests added or run.

### P26.21 — match creation Java detection to the server editor

**Status:** Implemented; owner visual verification pending.

**Files:** JavaSetupField; this plan.

**What:** Match the editor's detected-runtime row typography and layout: Java
version first, quiet runtime name alongside, smaller muted executable path
underneath, and a Use action at the right. Match the bounded 280-pixel scrolling
list, hover/selection borders and Close footer. Preserve the creation flow's
required Java explanation and disabled incompatible runtimes.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Add Server → Configure →
Java → Detect. Compare typography, list scrolling and Close footer against Edit
Server → Java → Detect. Select a compatible runtime and confirm its path appears;
incompatible runtimes remain unavailable.

**Batch:** Creation Java detection presentation only.

**Commit:** `P26.21: match creation java detection to editor styling`

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings);
touched Svelte formatting and `git diff --check` passed. No tests added or run.

### P26.22 — unify sidebar connection and enabled services

**Status:** Implemented; owner visual verification pending.

**Files:** ControlSidebar; ConsoleAccessSection; this plan.

**What:** Combine Services and How to connect into one Connection disclosure,
keeping the address visibility toggle. Show playit.gg controls only when its
server setting is enabled and Xbox Broadcast only when enabled for a supported
server. Remove disabled-service setup prompts. Hide the empty service group
without leaving extra spacing; clear old service facts when switching servers.

**Verify:** `cd clients/desktop-web && npx tauri dev` — expand Connection on a
server with both services disabled: only connection addresses should appear.
Enable each service in the editor and confirm its controls appear in Connection.
Switch servers and check the enabled services match the selected server. Check
the address visibility toggle and collapse/expand behavior.

**Batch:** Sidebar connection grouping and service visibility only.

**Commit:** `P26.22: unify sidebar connection and enabled services`

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings);
touched Svelte formatting and `git diff --check` passed. No tests added or run.

### P26.23 — remove sidebar time shortcut explanation

**Status:** Implemented; owner visual verification pending.

**Files:** QuickCommandsSection; this plan.

**What:** Remove “Dawn, dusk, and night keep the current Minecraft day.” from
sidebar Quick commands.

**Verify:** `cd clients/desktop-web && npx tauri dev` — expand Quick commands
and confirm the sentence below the time shortcuts is gone.

**Batch:** Requested sidebar sentence only.

**Commit:** `P26.23: remove sidebar time shortcut explanation`

Checks: inspected the two-line removal and intact Svelte conditional;
`git diff --check` passed. No tests added or run.

### P26.24 — remove worlds settings explanation

**Status:** Implemented; owner visual verification pending.

**Files:** WorldsSection; this plan.

**What:** Remove the paragraph explaining world-slot settings versus server
settings from the Worlds tab.

**Verify:** `cd clients/desktop-web && npx tauri dev` — open Worlds and confirm
the explanatory paragraph below the header is gone.

**Batch:** Requested Worlds tab paragraph only.

**Commit:** `P26.24: remove worlds settings explanation`

Checks: inspected the paragraph removal; `git diff --check` passed.
No tests added or run.

### P26.25 — publish 0.1.46 for Windows and Mac verification

**Status:** Release prepared; publication and owner platform verification pending.

**Files:** Engine/desktop version manifests and lockfiles; bundle identity;
release notes; this plan.

**What:** Bump the shared release version to 0.1.46 and publish the accumulated
owner-requested interface changes through the existing build-only beta workflow.
Preserve the complete package set, checksums and signed update metadata.

**Verify:** `gh release view v0.1.46` — confirm publication and the complete
asset set, then install the Windows/Mac desktop packages for physical acceptance.

**Batch:** Version, release notes and publication only.

**Commit:** `P26.25: prepare 0.1.46 beta release`

No test suites authorized or run. Release workflow remains unchanged.

### P26.26 — refresh README for 0.1.46

**Status:** Implemented; owner documentation review pending.

**Files:** README; this plan.

**What:** Update published release links and update examples to 0.1.46.
Refresh first-server instructions for Manage, inline Java selection, memory
allocation and import modpack browsing. Explain the background service versus
desktop/terminal controls using the agreed wording, document the header Engine
button and unified Connection section, and remove the remaining MSC 2 wording
from the handbook introduction.

**Verify:** `git show --stat --oneline HEAD` — review the README changes and
follow the 0.1.46 release and local release-notes links.

**Batch:** README refresh only.

**Commit:** `P26.26: refresh readme for 0.1.46`

Checks: verified 0.1.46 is a published prerelease using GitHub's release API;
local README link targets and `git diff --check` passed. No tests added or run.

### P26.27 — read Windows server CPU and RAM usage

**Status:** Implemented; owner Windows interaction verification pending.

**Files:** Infrastructure process metrics; this plan.

**What:** Replace the Windows no-reading fallback with the existing sysinfo
dependency's native process CPU and working-set memory readings. Retain sample
state for CPU deltas, reset it when the server process changes, and reuse the
last reading for requests inside the library's minimum sampling interval.
Report RAM in MiB and CPU as a percentage of whole-machine capacity, matching
the existing macOS/Linux convention. Leave CPU absent until the second sample
rather than reporting a false idle baseline. Both Overview and Performance
consume the existing shared API fields; no frontend or release change.

**Verify:** `cd clients/desktop-web && npx tauri dev` — when ready to stop the
current server, install the rebuilt background service through Engine repair,
then start Paper. Confirm RAM appears and CPU becomes numeric after subsequent
refreshes in Overview and Performance. Compare broadly with the Java process
in Task Manager; stop/start or switch servers and confirm readings follow the
current process rather than the previous server.

**Batch:** This Windows metrics implementation only.

**Commit:** `P26.27: read windows server cpu and ram usage`

Checks: `cargo fmt --all` and `cargo clippy -p msc-engine` passed, with existing
Windows warnings outside the changed code. An initial strict Clippy invocation
with `-D warnings` stopped on the existing BEDROCK_HELPER_SOCKET_MODE warning.
`git diff --check` passed. Cargo's unrelated lockfile package reordering was
discarded; no dependency changes were needed.

No tests added or run; no running server or installed service modified.
Separate reported uninstall issue: Windows MSI discovery still filters for the
old DisplayName MSC, while current packages use Minecraft Server Controller.
That finding is outside this metrics step.

### P26.28 — recognize current Windows packages during uninstall

**Status:** Implemented; owner uninstall preview and interaction verification pending.

**Files:** Infrastructure native uninstall discovery; this plan.

**What:** Accept Minecraft Server Controller as well as the older MSC Windows
MSI DisplayName. Retain publisher, WindowsInstaller and product-GUID checks;
do not invoke registry-supplied uninstall commands. Explain an unrecognized
installation as a verification failure rather than asserting that a downloaded
package is a developer checkout. Read-only registry inspection confirmed this
computer has Minecraft Server Controller 0.1.46, Publisher ctemple,
WindowsInstaller 1, in C:\Program Files\Minecraft Server Controller.
Build a corrected local CLI so the owner can preview/remove that installation
without replacing installed files or waiting for another published release.

**Verify:** `.\target\debug\msc.exe uninstall --dry-run` — run from the
repository root. Confirm the Windows MSI is listed as verified and the preview
has no blocked installation finding. This command removes nothing. If choosing
complete removal afterwards, close the desktop and use
`.\target\debug\msc.exe uninstall --danger`; review its inventory and type the
requested confirmation only when ready to permanently delete local worlds,
backups and MSC data. Windows Settings package removal is a different operation
and does not perform the complete local data cleanup.

**Batch:** This Windows uninstall discovery correction only.

**Commit:** `P26.28: recognize current windows packages during uninstall`

Checks: `cargo fmt --all`, `cargo clippy -p msc-engine`,
`cargo build -p msc-engine --bin msc` and `git diff --check` passed.
Existing Windows compiler/Clippy warnings remain outside the changed code.
The corrected executable is target/debug/msc.exe; the installed desktop and its
bundled CLI remain unchanged. Discarded Cargo's unrelated lockfile reordering.

No tests added or run. No uninstall, service stop or installed-file replacement
performed. Owner retains execution of the removal preview and destructive action.

### P26.29 — publish 0.1.47 with Windows metrics and uninstall fixes

**Status:** Publication failed; superseded by the owner-requested P26.30 correction.

**Files:** Engine/desktop version manifests and lockfiles; release notes; this plan.

**What:** Publish the Windows process-metrics and MSI discovery corrections as
0.1.47 through the existing build-only workflow. Preserve all nine packages,
checksums and signed update metadata. Read-only inspection before release found
no installed desktop MSI, service, running MSC processes, or MSC installation/
data trees on Cameron's Windows computer. A retained worker report records an
earlier partial removal with MSI error 1603; current inspection found no remaining
registered package or installation directory. This does not assert acceptance
of every uninstall continuation or of the new release's physical behavior.

**Verify:** `gh release view v0.1.47` — confirm the published prerelease contains
all nine packages, checksums and signed update metadata; install the matching
Windows build to verify CPU/RAM and complete-uninstall preview.

**Batch:** Version, release notes, tag push and publication only. Update README
as P26.30 after publication is confirmed.

**Commit:** `P26.29: prepare 0.1.47 windows fixes release`

Checks: parsed both Rust manifests/lockfiles and desktop JSON manifests;
all eight version declarations agree on 0.1.47. `git diff --check` passed.
The previous release workflow succeeded; the new tag did not exist remotely.

No tests authorized or run; release workflow unchanged. Prior implementation
formatting, Clippy and Windows CLI build passed with existing warnings.

Release run 44229679279 failed on all four platforms at release identity
validation before Rust release builds or packaging. The frontend bundle identity
remained at 0.1.46 while the other declarations and tag were 0.1.47. All failed
job logs were inspected. No release was published and the tag was not moved.

### P26.30 — correct release identity and publish 0.1.48

**Status:** Published; owner platform verification pending.

**Files:** Engine/desktop manifests and lockfiles; frontend bundle identity;
release notes; this plan.

**What:** Correct the omitted frontend bundle version and synchronize all nine
release version declarations at 0.1.48. Publish the same bounded Windows CPU/RAM
and MSI uninstall discovery fixes with a new tag; retain the failed 0.1.47 tag.
Inspect the workflow's actual identity expressions locally before pushing.
Keep the existing build-only workflow, nine packages, checksums and signed update
metadata. Expected hosted-build cost remains approximately 36–40 minutes;
identity correction resolves the observed failure, while downstream release
builds, packaging and notarization remain unverified until this run completes.

**Verify:** `gh release view v0.1.48` — confirm the published prerelease has all
nine packages, checksums and signed update metadata. Owner Windows acceptance
of CPU/RAM readings and complete-uninstall preview remains pending.

**Batch:** This corrected release only; README follows as P26.31 after publication.

**Commit:** `P26.30: synchronize 0.1.48 release identity`

Checks: parsed Rust/JSON manifests and lockfiles, and extracted the bundle
identity using the workflow's version-line pattern; all nine declarations agree
on 0.1.48. `git diff --check` passed. The new tag does not exist remotely.

No tests authorized or run. No workflow gates added or changed.

Publication evidence: run 44231049047 succeeded on all four platform builds and
the publish job. GitHub reports v0.1.48 as a published prerelease with all nine
desktop/headless packages, SHA256SUMS, signed update manifest and detached
signature, and release notes. The 0.1.47 tag remains unchanged.

### P26.31 — update README for published 0.1.48

**Status:** Implemented; owner documentation review pending.

**Files:** README; this plan.

**What:** Point current downloads, headless update command, release notes and
Windows manual-update guidance to published 0.1.48. Summarize restored Windows
CPU/RAM readings and corrected current-product MSI discovery during complete
uninstall. Preserve historical recovery guidance and physical acceptance limits.

**Verify:** `git show --stat --oneline HEAD` — review README and follow its
0.1.48 release and release-notes links.

**Batch:** README and release-publication evidence only.

**Commit:** `P26.31: update readme for published 0.1.48`

Checks: GitHub confirms a published prerelease with nine nonempty uploaded
packages and all required metadata. SHA256SUMS lists exactly those nine packages
and agrees with GitHub's recorded SHA-256 digests. README local links and
`git diff --check` passed. Installer contents and physical behavior remain for
owner verification; packages were not installed or executed.

No tests added or run; no workflow changes or installed application changes.

### P26.32 — welcome new desktop users before engine installation

**Status:** Implemented; owner visual and first-launch verification pending.

**Files:** App; InstallWelcome; shared OnboardingCard; TourOverlay; this plan.

**What:** Display a client-owned welcome card after the splash in the Tauri
app when its local engine is not installed and no remote host is saved.
Show “Welcome to MSC!” and “Please install the engine or connect to a host to
get started.” with one Okay button. Share the centered card component with
the tour invitation so dimensions, typography, surface and actions match.
Remember dismissal in the MSC client preferences independently of tour completion;
no engine API is required. Focus Okay, contain Tab and allow Escape to dismiss,
restoring prior focus afterwards. Installed/stopped/unavailable engines and saved
remote-host users do not receive this installation prompt. The existing engine
setup and later tour sequence remain available. Clearing MSC client preferences
also clears the welcome dismissal; no live preferences were changed here.

**Verify:** `cd clients/desktop-web && npx tauri dev` — on a fresh desktop client
profile with no installed engine or saved remote host, confirm the welcome appears
after the splash and matches the tour invitation. Click Okay, restart, and confirm
it stays dismissed. Check keyboard focus, Tab and Escape, installation/host
connection access after dismissal, and the later tour invitation after setup.
Use a separate clean environment rather than uninstalling a working engine merely
to check this prompt.

**Batch:** This pre-installation welcome card only.

**Commit:** `P26.32: welcome desktop users before engine installation`

Checks: `npm run check` passed with 0 errors and 10 existing CSS warnings;
`npm run build` passed with existing CSS/import/chunk warnings. Touched Svelte
formatting and `git diff --check` passed. No tests added or run; physical first-run
verification remains with Cameron. The pre-existing desktop lockfile ordering
change is excluded from this step.

### P26.33 — compact the server creation success screen

**Status:** Implemented; owner visual verification pending.

**Files:** AddServerWizard; ConfirmStep; this plan.

**What:** On successful creation, switch the wizard from its fixed 740-pixel
large sheet to the content-sized small sheet and remove the completed step
tracker. Release the wizard/content height and scrolling constraints so Done
sits directly beneath the result with normal spacing. Keep the server-created
heading, warnings and import-specific guidance. Fresh-world success now reads:
“Your server is ready for its first start. Choose Initiate in the server controls
to finish setup.” Earlier steps retain their existing dimensions and navigation.

**Verify:** `cd clients/desktop-web && npx tauri dev` — create a server when
ready for manual verification; confirm the success sheet is compact, has no step
tracker, and shows its heading, guidance and right-aligned Done without the
large blank area. Confirm Done closes the wizard and earlier pages retain their
large working layout. Check a narrow/short window and retained creation warnings
or import guidance when applicable.

**Batch:** Success-screen presentation and fresh-world guidance only.

**Commit:** `P26.33: compact the server creation success screen`

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings); touched
Svelte formatting and `git diff --check` passed. No tests added or run and no
live server creation performed. Pre-existing desktop lockfile ordering is
excluded from this step.

### P26.34 — keep console visibility and show startup progress in its heading

**Status:** Implemented; owner interaction verification pending.

**Files:** App; ApplicationShell; ConsoleDock; lifecycle presentation type; this plan.

**What:** Remove Start's automatic console expansion. Preserve the user's
collapsed or expanded state and manual toggle/resize controls. Display a small
progress bar and status text beside Console in the dock heading in both states.
Reuse the existing start-operation polling, with count-based fill when the engine
supplies valid progress and an indeterminate bar during the initial request or
readiness wait. No simulated percentages or extra polling. Scope progress by
host/server, clear it in startup cleanup on success/failure, and preserve the
existing startup failure sheet. Truncate long status text with the full message
available on hover. No stop/start behavior or engine timing changes.

**Verify:** `cd clients/desktop-web && npx tauri dev` — collapse Console and start
an initialized server: it must remain collapsed with progress beside Console.
Expand it during startup and confirm progress stays in the heading; manually
collapse it again. Confirm progress clears after startup, Start leaves an already
expanded console open, and switching servers/hosts does not show another server's
progress. Check failure cleanup when a real startup failure is available, and
check heading layout at narrow window widths.

**Batch:** Console visibility on Start and inline startup progress only.

**Commit:** `P26.34: show startup progress without expanding the console`

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings); touched
frontend formatting and `git diff --check` passed. No tests added or run; no live
servers started or stopped. Existing operation/status polling is reused rather
than adding timing-sensitive progress tests. Pre-existing desktop lockfile
ordering remains outside this step.

### P26.35 — lead initiation with progress and collapse its console

**Status:** Implemented; owner interaction verification pending.

**Files:** FirstStartSheet; this plan.

**What:** Replace the always-visible first-start console with a closed-by-default
Show console disclosure, switching its label to Hide console when opened.
Retain the live buffered output and Clear action inside. Show a full-width
progress bar while initiation is busy and a completed bar on success. Use the
existing first/second-pass operation counts when available, otherwise show
indeterminate progress; reset count state between passes/retries and do not
reuse counts during another phase. Existing status messages, connection choices,
errors and retry behavior remain visible independently of console expansion.
No extra polling or engine changes.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Initiate a new server when
ready for manual verification. Confirm progress appears during server preparation,
the console starts hidden, and Show console/Hide console toggle live output
without interrupting setup. Check both passes, connection selection, completion,
and Clear when expanded. A retry must not inherit a previous pass's progress.

**Batch:** Initiation progress and optional console presentation only.

**Commit:** `P26.35: show initiation progress with an optional console`

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings); touched
Svelte formatting and `git diff --check` passed. No tests added or run; no live
initiation performed. The pre-existing desktop lockfile change is excluded.

### P26.36 — measure initiation by flavor-aware setup milestones

**Status:** Implemented; owner initiation walkthrough verification pending.

**Files:** App; FirstStartSheet; initiationProgress presentation model; this plan.

**What:** Replace raw operation fractions (process spawn reported 1/2, causing
an immediate 50% display) with a single count of setup milestones across both
passes and the intervening connection choices. Each pass tracks runtime launch,
applicable loader initialization, installed extension loading stages, world
preparation, engine readiness and confirmed shutdown. Thread the server's actual
Java flavor into the sheet. Fabric/Forge/NeoForge/Quilt receive a loader stage;
Bedrock uses its own runtime label without Java loader/plugin stages. Read
existing local component/add-on inventories once before initiation: installed
Geyser and Floodgate receive separate stages, other enabled add-ons are grouped
as mods or plugins. Unavailable inventories fall back to core runtime stages
rather than preventing initiation.

Use current-run server log hints for loading/world activity, and engine operation
results for authoritative ready/failure/stop outcomes. Logs are presentation hints,
not new readiness gates or proof that every mod works. A confirmed ready signal
settles earlier loading stages when a flavor does not emit a recognizable log.
Existing final helper diagnosis remains authoritative and unchanged. Keep the
next milestone and completed/total count visible. Progress represents setup
stages, not a prediction of remaining time. No engine lifecycle or timeout changes.

Snapshot console identities and engine timestamps before each pass; ignore older
history, error lines and non-server origins. Track hints independently of Clear,
use engine timestamps to avoid remote clock differences, and claim pass-two
state before awaiting its snapshot so connection refreshes cannot launch it twice.
Hide the progress bar entirely on completion or failure; retain Show console.

**Verify:** `cd clients/desktop-web && npx tauri dev` — initiate an uninitialized
server: confirm progress starts at zero and spawn advances one milestone, not
50%. Check the next-stage caption, connection-choice pause, second pass, and
hidden bar on completion. Compare vanilla/Paper, an installed-loader modded
server, enabled/disabled add-ons, Geyser/Floodgate when installed, and Bedrock
as available. Reuse a server with old logs to confirm they do not advance a new
attempt; Clear must not reset/replay milestones. Confirm failure/retry retains
its existing behavior and does not falsely report success.

**Batch:** Initiation milestone presentation and completed-bar removal only.

**Commit:** `P26.36: track initiation progress across runtime milestones`

Checks: final `npm run check` passed (0 errors, 10 existing CSS warnings),
`npm run build` passed with existing CSS/import/chunk warnings; touched frontend
formatting and `git diff --check` passed. No tests added or run; no live initiation,
provider downloads or installed server changes. Flavor-specific physical
walkthroughs remain owner verification. Pre-existing desktop lockfile ordering
is excluded from this step.

### P26.37 — offer hidden initiation through the main Resume button

**Status:** Implemented; owner interaction verification pending.

**Files:** ControlSidebar; this plan.

**What:** Remove the separate hidden-first-start explanation and action block
from the sidebar. When the selected server has hidden initiation, replace its
main Start/Initiate/Stop control with Resume. Resume opens the shared onboarding
card presentation with Resume setup and Discard setup choices. Explain that
discard stops the setup server and keeps its world; use the existing parent
resume/discard handlers without changing engine operations. Keep Resume available
while the hidden setup runs, preserving ordinary Start/Initiate/Stop behavior
otherwise and the existing discard/control-permission guards. Focus Resume setup,
contain Tab, dismiss on Escape without changing setup, and restore focus. Clear
the choice when selection changes or hidden initiation is no longer available.

**Verify:** `cd clients/desktop-web && npx tauri dev` — start Initiate, close the
setup sheet and confirm the sidebar has Resume with no extra setup section.
Click Resume: check the small onboarding-style card, keyboard focus and Escape.
Resume setup must reopen the existing sheet with progress intact; hide it again
and deliberately choose Discard setup to verify the existing stop/discard flow
keeps the world. Check switching servers does not offer another server's hidden
setup and normal Start/Initiate/Stop controls return when appropriate.

**Batch:** Hidden-initiation sidebar control and choice card only.

**Commit:** `P26.37: offer resume or discard from the main server control`

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings); touched
Svelte formatting and `git diff --check` passed. No tests added or run; no live
setup discarded and no server stopped. Pre-existing desktop lockfile ordering
is excluded from this step.

### P26.38 — count only initiation passes the engine performs

**Status:** Implemented; owner initiation verification pending.

**Files:** FirstStartSheet; initiationProgress; focused milestone regression tests;
this plan.

**What:** Fix the reported 3/9-then-complete display. The engine's
FirstStartCoordinator requires a second pass only when Playit or Xbox Broadcast
is applicable; a helper-free first run returns firstStartComplete directly.
The previous presentation always counted both passes and connection setup, and
its direct-completion branch only settled pass two. Build the initial denominator
from the configured transports, label helper-free work First run, and settle the
actual pass from successful structured results before entering completion.
Honor direct completion even when two passes were initially expected by removing
unperformed second-pass/connection milestones. Settle ordinary successful first
starts at readiness and retain confirmed-stop separation. If an existing-world
recovery actually enters connection setup, expand its plan for that real second
run. Recognize the engine's pass-one-ready status line as an intermediate hint.
Keep the completed-screen progress bar hidden and retain flavor/helper stages.

**Verify:** `cd clients/desktop-web && npx tauri dev` — initiate a fresh
vanilla/Paper/Purpur server with Playit and Xbox Broadcast disabled. Confirm it
counts only first-run milestones (4 without optional stages), advances at ready,
and hides the bar at completion. Compare an initiation with a transport enabled:
its two-pass stages must remain, with pass-one completion not completing pass two.
Check an existing-world retry follows its actual recovery path.

**Batch:** Initiation denominator and successful-pass settlement only.

**Commit:** `P26.38: align initiation milestones with actual engine passes`

Essential regression rationale: pure progress-state cases protect the specific
reported false denominator and wrong-pass settlement, including early direct
completion and preserving unfinished second-pass work. Existing first-start
coverage checks source text and does not exercise these state transitions.
Tests use local values, no timers, processes, filesystem or network; expected
milliseconds. Added but not run: tests/screens/initiation-progress.test.ts.

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings); touched
frontend/test formatting and `git diff --check` passed. No tests run and no live
server operations performed. Existing desktop lockfile ordering is excluded.

### P26.39 — use startup milestones in the main console heading

**Status:** Implemented; owner startup walkthrough verification pending.

**Files:** App; ConsoleDock; lifecycle progress presentation/model; focused startup
progress regression cases; this plan.

**What:** Replace normal Start's raw operation fraction with a single-runtime
milestone tracker, reusing initiation's flavor stages and console-hint parser.
Exclude pass labels, connection choices, second runs and shutdown. Detect installed
Geyser/Floodgate and enabled other add-ons from local component/add-on inventories;
include the applicable mod-loader stage and group other extensions as mods/plugins.
Show the next-step explanation beside Console, with completed/total count, in
both collapsed and expanded states. Examples include Starting paper, Initializing
the mod loader, Loading Geyser, Preparing the world and Waiting for the server-ready
signal. No simulated time-based percentage and no console auto-expansion.

Capture the pre-start console tail and ignore older engine timestamps/known lines,
error output and non-server origins. During the existing operation/readiness
loops, read progress hints at most once per second; no independent timers. Engine
operation/status readiness settles earlier loading stages even when a flavor
omits recognizable messages. Clear progress at confirmed running state and on
failure/final cleanup. Retain host/server scoping and startup diagnosis behavior.
Inventory lookup failure falls back to core runtime stages. No engine changes.

**Verify:** `cd clients/desktop-web && npx tauri dev` — start an initialized
server with Console collapsed. Check milestone fill/count and explanatory messages
beside Console, no automatic expansion, and progress disappearing on readiness.
Compare an installed-loader server, Geyser/Floodgate or other enabled add-ons,
and Bedrock when available. Check older logs do not pre-fill startup, switching
servers does not show another server's progress, failure clears the indicator,
and the expanded/narrow heading still permits console controls.

**Batch:** Normal-start console milestone progress and explanatory messages only.

**Commit:** `P26.39: show runtime milestones in the main console heading`

Essential regression rationale: normal startup must never inherit initiation's
shutdown/connection denominator or the engine's 1/2 spawn fraction. Two pure cases
protect single-run accounting, readiness settlement and ignoring stale/error/helper
logs while retaining loader/extension stages. Existing initiation tests do not
exercise this normal-start tracker. Local inputs only, no timers/network/processes;
expected milliseconds. Added but not run: tests/screens/startup-progress.test.ts.

Checks: final `npm run check` passed (0 errors, 10 existing CSS warnings) and
`npm run build` passed with existing CSS/import/chunk warnings. An initial type
check caught completion cleanup placed in the wrong scope; corrected before final
checks. Touched formatting and `git diff --check` passed. No tests run and no live
servers started/stopped. Existing desktop lockfile ordering is excluded.

### P26.40 — update startup milestones from live console output

**Status:** Implemented; owner fast-start walkthrough verification pending.

**Files:** App; ApplicationShell; ConsoleDock; this plan.

**What:** Investigate the reported vanilla 1/3 transition using the existing
server's latest.log, without starting/stopping it. The recorded run emitted
Preparing level, Preparing spawn area and Done within roughly one second;
Done reported 0.429 seconds. These messages already match the milestone parser,
but the separate once-per-second progress tail reader could miss their visible
intermediate states before operation readiness completed the progress.

Feed the existing console WebSocket history updates directly into the active
host/server startup tracker, including while collapsed. Keep bounded polling
as fallback and retain baseline/history/error/helper filtering in the tracker.
Forward fetched output before local Clear filtering so hiding console text does
not suppress progress. Ignore callbacks from obsolete stream generations and
capture poll request host/server identities to avoid cross-host progress. Remove
the tracker at readiness/final cleanup so later output cannot resurrect the bar.
No synthetic progress holds or minimum-duration delays: very fast servers may
still pass intermediate stages briefly. No extra streams, timers or engine changes.

**Verify:** `cd clients/desktop-web && npx tauri dev` — start an initialized
vanilla server with Console collapsed. Check the existing live feed updates the
milestones/messages as preparation lines arrive and clears the bar at readiness.
Expand Console to compare real output if needed. Check Clear and switching
hosts/servers do not replay old progress. A sub-second preparation stage may be
brief; startup must not be delayed to make it linger.

**Batch:** Live console delivery into existing normal-start progress only.

**Commit:** `P26.40: advance startup progress from live console output`

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings); touched
Svelte formatting and `git diff --check` passed. Final request-identity and
obsolete-stream guards were inspected after that check. No tests added or run;
no live server operations performed. Existing progress parser/state tests already
cover the observation method; this change supplies it with the real stream rather
than adding source-text or timing-based tests. Existing desktop lockfile ordering
is excluded from this step.

### P26.41 — continue onboarding through initiation and the app walkthrough

**Status:** Implemented; owner full-flow and visual verification pending.

**Files:** Guide content/source map/corpus count; FirstLaunchGate, AppTour and
checkpoint state; TourOverlay/anchors/types; App and shell/sidebar/header;
creator success action; Worlds and Help; focused continuation regression cases;
existing guide-content checker metadata; this plan.

**What:** Implement the owner-reviewed card copy and the subsequent amendment
placing optional exploration after Connection addresses. Preserve the first
28 creation cards, update the welcome copy to remove the two-minute promise,
and replace the obsolete post-creation EULA/boot/Packs-tab tail with 27 cards:
created Done; Initiate; Start/Stop; server selector/Manage; Connection; optional
Explore MSC prompt; Console; Overview; Players; World slots; backups; world
packs; conditional View Map; Performance; Components; server properties; server
editor; Files; Maintenance; Quick commands; layout; Engine; Preferences;
Help; Handbook; Router Guide; finish. Corpus now has 55 cards with complete
source mappings. Keep copy/edition/flavor variants in engine-served guide data.

Creation success no longer completes onboarding. Capture the actual created
server from the originating API, keep its Done screen visible, and wait for
successful wizard cleanup/closure before returning to the main controls.
Highlight Initiate, then pause the tour during guided setup; hidden/in-progress
or failed setup does not advance it. Successful completion and dismissal resume
the basic controls cards. Discard returns to the Initiate introduction. Saved
waiting checkpoints can reconcile a completed setup after reopening the app.
Host/server changes pause the walkthrough instead of guiding another server.

After Connection, show You’ve got the basics with Finish for now and Explore MSC.
Exploration opens the relevant tabs/disclosures, highlights real controls,
provides Back/Next/Finish, skips unavailable routes/map controls, and returns to
Overview at the finish. Expand Console for its explanation and restore its prior
visibility afterwards. Preserve address visibility. Tailor copy for Java/Bedrock,
vanilla/plugin/modded software and configured connection helpers. Navigation
never starts a server, changes settings, runs a command, installs a component,
or creates/restores a backup. Opening actual setup remains a deliberate Initiate
click, not an automatic tour action.

Save continuation checkpoints in MSC-owned preferences separately from the
legacy tour-complete flag. Help now offers Explore MSC; it resumes the optional
section without repeating creation, while Restart the guide still restarts the
full flow. Finish for now after the basics saves Console as the optional entry.
Add bounded, keyboard-contained informational cards with Escape dismissal and
focus restoration; required-action cards retain access to their real controls.
Navigation/content failures provide explicit retry/finish UI rather than looping.
An old engine corpus reports that restart/repair is needed instead of silently
ending the expanded walkthrough. Update the existing content-checker's skip
metadata expectation to match the actual Help entry point; no new workflow gate.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Repair engine if needed to
load the new bundled guide, then Help → Onboarding → Restart the guide. Create a
server and confirm the tour waits on the success Done screen, highlights Initiate,
and disappears throughout its guided setup. Hide/resume setup; check a failure
or discard when appropriate. After successful completion and Done, check Start/
Stop → server selector → Connection → optional prompt. Finish for now, then use
Help → Explore MSC to resume at Console. Walk all screens with Next/Back, including
conditional View Map, both guide views and returning to Overview. Check keyboard
navigation, console visibility restoration, paused host/server changes, saved
continuation after app restart, and Java/Bedrock/flavor-specific text as available.
Perform live setup/discard only deliberately as part of owner verification.

**Batch:** This complete owner-approved onboarding continuation only; one commit.

**Commit:** `P26.41: continue onboarding through initiation and the app walkthrough`

Essential regression rationale: three local state cases protect the real
initiation wait boundary, the owner-mandated optional prompt between Connection
and Console, and refusing another host/server with the same-looking selection.
They use guide data and pure transitions, with no timers, processes, network,
browser rendering or source-text assertions; expected milliseconds. Added but
not run: tests/help/app-tour.test.ts. No other tests added or run.

Checks: final frontend `npm run check` passed (0 errors, 10 existing CSS warnings);
`npm run build` passed with existing CSS/import/chunk warnings. Workspace Rust
format check and engine Clippy passed with existing unused-variable/mut warnings.
Touched frontend formatting and `git diff --check` passed. Inspected parsed guide
card count, duplicate IDs and complete source-map coverage. The existing content
checker/test suites were not run. Clippy's package-order-only root lockfile rewrite
was restored; the pre-existing desktop lockfile ordering remains outside this step.
No live server, engine repair, release, push or publication performed.

### P26.42 — center tour actions and keep highlight borders visible

**Status:** Implemented; owner visual verification pending.

**Files:** TourOverlay; this plan.

**What:** Center action rows on all app-walkthrough cards, including the
You’ve got the basics choice. Replace the footer Finish tour action with a
neutral top-right X carrying the Finish tour accessible label and tooltip.
Keep Back/Next below the text and retain the optional prompt's explicit
Finish for now / Explore MSC choices. Reserve heading space for X on anchored
cards. Initial keyboard focus stays on a navigation button rather than X;
existing Tab containment, Escape and checkpoint/finish behavior remain.

Clamp padded spotlight edges to a four-pixel viewport inset and use border-box
sizing, so highlighting a full-width tab pane or console cannot clip its right
or bottom outline outside the app window. No guide copy/content or lifecycle
changes; no engine repair is needed for these client-side presentation fixes.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Help → Explore MSC;
check the optional prompt's centered actions when returning with Back, the
Console/tab cards' centered Back/Next row and top-right X, and all four outline
edges on Overview and the other tabs. Resize the window, check keyboard focus,
and confirm X finishes the walkthrough while saving the existing checkpoint.

**Batch:** Reported walkthrough action alignment, finish control and clipped
spotlight borders only.

**Commit:** `P26.42: align tour actions and contain spotlight borders`

Checks: `npm run check` passed (0 errors, 10 existing CSS warnings); touched
Svelte formatting and `git diff --check` passed. No tests added or run for this
presentation-only correction. Owner visual verification remains pending.
Pre-existing desktop lockfile ordering is excluded from this step.

### P26.43 — identify Bedrock rendering and explain empty saved terrain

**Status:** Implemented; owner empty-world/map verification pending.

**Files:** Shared terrain renderer and Bedrock map route; this plan.

**What:** Before launching the shared renderer for converted Bedrock terrain,
inspect the requested dimension's region directory. Missing/empty region storage
returns dimension_empty with: “No saved Bedrock terrain yet in this dimension.
Start the server, join with Minecraft Bedrock, and explore to generate terrain.
Then stop the server and reopen the map.” Check only that dimension; retain
storage errors and unknown dimensions as distinct failures. Perform this check
once at renderer startup rather than reading a conversion manifest per tile.
The Bedrock route exposes this as HTTP 409/dimension_empty instead of a renderer
startup failure, and the existing viewer shows the explanatory message.

Select renderer edition from the existing edition identity: startup failures,
timeouts and captured failure logs now identify Bedrock terrain renderer for
Bedrock requests. Translate remaining shared dependency/artifact renderer errors
at the Bedrock response boundary; Java requests keep Java wording. No world
conversion, region generation or live server lifecycle behavior changes.

**Verify:** `cd clients/desktop-web && npx tauri dev` — Repair engine to use the
updated binary, then open a freshly initiated Bedrock world's map without saved
chunks. Confirm the no-saved-terrain message and guidance instead of NoRegions /
Java renderer failure. After deliberately joining/exploring and stopping the
server, reopen the map and confirm saved terrain can render. An unvisited Nether
must report empty even if Overworld terrain exists. Any genuine Bedrock renderer
startup error must name Bedrock; Java failures retain Java wording.

**Batch:** Bedrock renderer naming and empty-terrain handling only.

**Commit:** `P26.43: explain empty bedrock terrain and identify its renderer`

Essential regression rationale: one isolated temporary-directory case protects
missing/empty regions, ignoring unrelated files, distinguishing a populated
Overworld from an empty Nether, and preserving unknown-dimension errors. It uses
unique local paths with cleanup, no servers/network/timers, and an 8 KiB region
presence fixture; expected milliseconds. Added but not run.

Checks: Rust formatting and engine Clippy passed with existing unused-variable/
mut warnings; `git diff --check` passed. No tests or live server/map operations
run. Clippy's package-order-only root lockfile change was restored; pre-existing
desktop lockfile ordering remains excluded.
