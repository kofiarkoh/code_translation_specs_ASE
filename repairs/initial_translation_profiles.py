from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class InitialTranslationProfile:
    name: str
    needs_spec: bool
    needs_code: bool
    prompt_template: str


SPECS_ONLY_PROFILE = InitialTranslationProfile(
    name="specs_only",
    needs_spec=True,
    needs_code=False,
    prompt_template="""You are a senior systems engineer fluent in C and Rust, with strong expertise in program semantics and safe systems programming.

Task:
Translate the given Specification into Rust code.

Primary goal:
Preserve semantics based ONLY on the Specification provided. Do not assume any external code or source.

Instructions:
1) Treat the Specification as the only source of truth for behavior.
2) Preserve observable I/O behavior described in the Specification.
3) Preserve corner-case behavior implied by the Specification. For example, if the Specification relies on:
   - integer overflow / wraparound,
   - bit-level tricks,
   - undefined/implementation-defined behavior,
   then emulate the intended behavior as closely as possible in Rust.
4) If the Specification is incomplete, ambiguous, or inconsistent:
   - do not invent extra functionality
   - choose the narrowest reasonable interpretation justified by the Specification
   - produce compilable Rust that stays as faithful as possible to the stated behavior
5) Prefer safe, idiomatic Rust:
   - Avoid `unsafe` unless necessary; if used, justify it in a short comment.
6) You may add comments as needed to explain non-obvious translation choices, especially if they relate to preserving semantics or handling corner cases.
7) Any explanation must appear only as valid Rust comments inside the code and not after the code.
8) Return ONLY the Rust code in a single ```rust code block.


Specification:
```text
{input_text}
```""",
)


CODE_ONLY_PROFILE = InitialTranslationProfile(
    name="code_only",
    needs_spec=False,
    needs_code=True,
    prompt_template="""You are a senior systems engineer fluent in C and Rust, with strong expertise in program semantics and safe systems programming.

Task:
Translate the given C code into Rust.

Primary goal:
Preserve semantics based ONLY on the C code provided. Do not assume any external specification.

Instructions:
1) Treat the C code as the only source of truth for behavior.
2) Preserve I/O behavior exactly (stdin/stdout formatting, whitespace, newlines).
3) Preserve corner-case behavior implied by the C code. For example, if the C code relies on:
   - integer overflow / wraparound,
   - bit-level tricks,
   - undefined/implementation-defined behavior,
   then emulate the intended behavior as closely as possible in Rust.
4) Prefer safe, idiomatic Rust:
   - Avoid `unsafe` unless necessary; if used, justify it in a short comment.
5) You may add comments as needed to explain non-obvious translation choices, especially if they relate to preserving semantics or handling corner cases or just as a human developer would to clarify intent.
6) Return ONLY the Rust code in a single ```rust code block.

Now translate the following.

=== C CODE START ===
{code}
=== C CODE END ===""",
)


CODE_AND_SPEC_PROFILE = InitialTranslationProfile(
    name="code_and_spec",
    needs_spec=True,
    needs_code=True,
    prompt_template="""You are a senior systems engineer fluent in C and Rust, with strong expertise in program semantics and specification-driven translation.

Task:
Translate the given C code into Rust, using the provided code and specification as the source-of-truth for intended behavior.

Primary goal:
Preserve semantics. The Rust code must satisfy the specification (functional behavior, preconditions, postconditions, invariants, and error behavior).

Instructions:
1) Read the SPEC first and treat it as the definitive description of expected behavior.
2) Use the C code only to resolve ambiguities and edge cases and do not implement any non implemented function.
3) If the C code conflicts with the spec, follow the spec and explicitly note the mismatch.
4) Prefer safe, idiomatic Rust:
   - Strongly Avoid `unsafe` unless necessary; if used, justify it in a short comment. Prioritise converting unsafe code to safe code.
5) Preserve I/O behavior exactly (stdin/stdout formatting, whitespace, newlines).
6) Avoid unsafe unless absolutely necessary.
7) You may add comments as needed to explain non-obvious translation choices, especially if they relate to preserving semantics or handling corner cases or just as a human developer would to clarify intent.
8) Return ONLY the Rust code in a single ```rust code block.

Now translate the following code using the specification.

=== SPECIFICATION START ===
{spec}
=== SPECIFICATION END ===

=== C CODE START ===
{code}
=== C CODE END ===""",
)


PROFILES = {
    SPECS_ONLY_PROFILE.name: SPECS_ONLY_PROFILE,
    CODE_ONLY_PROFILE.name: CODE_ONLY_PROFILE,
    CODE_AND_SPEC_PROFILE.name: CODE_AND_SPEC_PROFILE,
}


def get_initial_translation_profile(profile_name: str) -> InitialTranslationProfile:
    try:
        return PROFILES[profile_name]
    except KeyError as exc:
        valid = ", ".join(sorted(PROFILES))
        raise ValueError(f"Unknown initial translation profile: {profile_name}. Valid options: {valid}") from exc


def list_profile_specs(profile: InitialTranslationProfile, problem_dir: Path, max_files: int) -> list[Path]:
    if not profile.needs_spec:
        return []
    return sorted(p for p in problem_dir.glob("*_spec.txt") if p.is_file())[:max_files]


def list_profile_code_files(profile: InitialTranslationProfile, problem_dir: Path, max_files: int) -> list[Path]:
    if not profile.needs_code:
        return []

    direct_files = sorted(p for p in problem_dir.glob("*.c") if p.is_file())
    if direct_files:
        return direct_files[:max_files]

    language_dir = problem_dir / "C"
    if language_dir.is_dir():
        nested_files = sorted(p for p in language_dir.glob("*.c") if p.is_file())
        return nested_files[:max_files]

    return []


def build_initial_translation_prompt(
    profile: InitialTranslationProfile,
    problem_id: str,
    spec_text: str = "",
    code_text: str = "",
) -> str:
    return profile.prompt_template.format(
        problem_id=problem_id,
        spec=spec_text.rstrip(),
        code=code_text.rstrip(),
        input_text=(spec_text if profile.needs_spec and not profile.needs_code else code_text).rstrip(),
    )
