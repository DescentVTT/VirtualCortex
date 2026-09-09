# Security policy

## Supported versions

VirtualCortex is pre-1.0. Only the `main` branch receives fixes.

## Reporting a vulnerability

Please do not open a public issue for a security problem. Use GitHub's private vulnerability reporting on this repository (Security tab, "Report a vulnerability"). If that is unavailable, contact the maintainers through the repository owner's GitHub profile.

Include what you can of: the affected crate or document, a reproduction, the impact you believe it has, and whether you would like credit.

You can expect an acknowledgement within seven days and a decision on severity and remediation within thirty. We will coordinate disclosure timing with you.

## Scope

In scope:

- Memory-safety or determinism defects in any crate under `crates/`.
- Parsing of untrusted input: `.cortex` images, fabric packets, sensory event batches.
- The embodiment interface, where a defect could produce an unsafe actuator command.
- The CI and documentation tooling configuration in this repository.

Out of scope: vulnerabilities in third-party physics engines, robot controllers or operating-system tuning that this project only documents.

## Design commitments

The engine never trusts a byte it has not verified (checksums on images and packets), contains no `unsafe` code today and requires an ADR to introduce any, and is designed so that an external hardware watchdog, not the engine, is the last line of defence for a physical body. See the whitepaper, §8.9 and §8.10.
