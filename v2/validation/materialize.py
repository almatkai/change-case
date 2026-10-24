"""Materialize test-only manifests excluded by Cargo's nested-package rules."""
from pathlib import Path
import shutil
root=Path(__file__).resolve().parent
out=root/'materialized';(out/'tests').mkdir(parents=True,exist_ok=True)
for src,dst in [('session-Cargo.toml.template','Cargo.toml'),('session-lib.rs.template','lib.rs'),('session-tests.rs.template','tests/session.rs')]:shutil.copy2(root/src,out/dst)
lock=root/'session-Cargo.lock.template'
if lock.exists():shutil.copy2(lock,out/'Cargo.lock')
