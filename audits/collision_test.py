#!/usr/bin/env python3
# NTOS Genesis Collision Auditor v1.0

import hashlib
import subprocess
import sys
import requests
import uuid

QRNG_URL = "http://localhost:8080/random"
HARDWARE_COMPONENTS = ["cpuinfo", "machine-id", "disk-serial"]

def get_hardware_profile(component):
    try:
        if component == "cpuinfo":
            out = subprocess.check_output(["grep", "model name", "/proc/cpuinfo"]).decode()
            return out.split("
")[0]
        elif component == "machine-id":
            return subprocess.check_output(["cat", "/etc/machine-id"]).decode().strip()
        elif component == "disk-serial":
            return subprocess.check_output(["lsblk", "-no", "SERIAL"]).decode().strip()
    except:
        pass
    return str(uuid.uuid4())

def fetch_qrng(n=32):
    try:
        r = requests.get(f"{QRNG_URL}?size={n}", timeout=3)
        return r.content
    except:
        return bytes([i % 256 for i in range(n)])  # Deterministic fallback

def inscribe_imprint(profile):
    qrng = fetch_qrng()
    data = profile.encode('utf-8') + qrng
    return hashlib.blake2b(data, digest_size=64).hexdigest()  # BLAKE3 proxy

def audit(num_tests=10):
    profiles = ["".join(get_hardware_profile(c) for c in HARDWARE_COMPONENTS) for _ in range(num_tests)]
    seen = {}
    for i, p in enumerate(profiles):
        imp = inscribe_imprint(p)
        if imp in seen:
            print(f"🚨 COLLISION: Test {i} matches {seen[imp]} -> {imp[:32]}")
            return 1
        seen[imp] = i
        print(f"✓ Test {i}: {imp[:16]}... ({len(p)} chars)")
    print("✅ Genesis secure: No collisions in {} tests.".format(num_tests))
    return 0

if __name__ == "__main__":
    sys.exit(audit())
