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
