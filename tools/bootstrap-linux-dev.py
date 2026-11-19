#!/usr/bin/env python3
"""Provision the Linux development service helper without a release package."""

import os
import hashlib
from pathlib import Path
import stat
import shutil
import subprocess
import sys
import tempfile


DESTINATION = Path("/usr/lib/msc-development/agent/msc")
PACKAGED = [
    Path(f"/usr/lib/{name}/agent/msc")
    for name in ("Minecraft Server Controller", "msc-desktop-web", "MSC 2", "msc2-desktop-web")
]


def protected(path, directory=False):
    metadata = path.lstat()
    kind = stat.S_ISDIR if directory else stat.S_ISREG
    if not kind(metadata.st_mode) or metadata.st_uid != 0 or metadata.st_mode & 0o022:
        raise RuntimeError(f"Expected a protected root-owned {'directory' if directory else 'file'}: {path}")


def same_build(left, right):
    def digest(path):
        with path.open("rb") as stream:
            return hashlib.file_digest(stream, "sha256").digest()
    return digest(left) == digest(right)


def main():
    source = Path(sys.argv[1]).resolve(strict=True)
    if os.geteuid() != 0:
        # Package helpers retain priority, matching the desktop's lookup order.
        for candidate in [*PACKAGED, DESTINATION]:
            if candidate.is_file():
                protected(candidate)
                for parent in candidate.parents:
                    protected(parent, directory=True)
                if not os.access(candidate, os.X_OK):
                    raise RuntimeError(f"Service helper is not executable: {candidate}")
                # Packaged helpers are maintained by the package manager. A
                # development helper must track the build's privileged commands.
                if candidate != DESTINATION or same_build(source, candidate):
                    return 0
                break
        print("MSC development needs administrator authorization to install or update its service helper.", flush=True)
        print("This installs the locally built helper only. Use Install/Repair in the app to register the Background Service.", flush=True)
        return subprocess.call(["pkexec", sys.executable, "-I", str(Path(__file__).resolve()), str(source)])

    if Path("/sys/fs/selinux/enforce").exists():
        if not shutil.which("semanage") or not shutil.which("restorecon"):
            raise RuntimeError("SELinux setup needs semanage and restorecon; on Fedora install policycoreutils-python-utils")
    if "PKEXEC_UID" not in os.environ:
        raise RuntimeError("Run this setup as your regular user; it requests authorization through pkexec")
    # Never follow links or write through directories controlled by another user.
    for directory in reversed(DESTINATION.parent.parent.parents):
        protected(directory, directory=True)
    for directory in [DESTINATION.parent.parent, DESTINATION.parent]:
        directory.mkdir(mode=0o755, exist_ok=True)
        protected(directory, directory=True)
    if DESTINATION.exists() or DESTINATION.is_symlink():
        protected(DESTINATION)
        if same_build(source, DESTINATION):
            return 0
    with source.open("rb") as stream:
        metadata = os.fstat(stream.fileno())
        if not stat.S_ISREG(metadata.st_mode) or metadata.st_uid != int(os.environ["PKEXEC_UID"]):
            raise RuntimeError("The development engine must be a regular file owned by the authorizing user")
        payload = stream.read()
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(dir=DESTINATION.parent, delete=False) as output:
            temporary = Path(output.name)
            output.write(payload)
            output.flush()
            os.fchmod(output.fileno(), 0o755)
            os.fsync(output.fileno())
        temporary.replace(DESTINATION)
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)
    try:
        if Path("/sys/fs/selinux/enforce").exists():
            rule = str(DESTINATION)
            result = subprocess.run(["semanage", "fcontext", "-a", "-t", "bin_t", rule])
            if result.returncode:
                subprocess.run(["semanage", "fcontext", "-m", "-t", "bin_t", rule], check=True)
            subprocess.run(["restorecon", "-F", str(DESTINATION)], check=True)
    except (OSError, subprocess.CalledProcessError):
        DESTINATION.unlink()
        raise
    print(f"Installed development service helper: {DESTINATION}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, RuntimeError, ValueError, subprocess.CalledProcessError) as error:
        sys.exit(f"MSC development helper setup: {error}")
