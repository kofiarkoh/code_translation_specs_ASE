from __future__ import annotations


SYSTEM_PROMPT = "You are a careful software specification and C code synthesis assistant."


INITIAL_SPEC_PROMPT_TEMPLATE = """You are an expert senior software engineer.
Given source code, generate a precise, structured behavioral specification that states WHAT the code guarantees, not HOW it achieves it.

Output must use exactly these sections and headings:
1. Functional Description
2. Inputs
3. Outputs
4. Preconditions
5. Postconditions
6. Invariants
7. Error Handling / Exceptional Behavior
8. State Transitions (if applicable)
9. Non-functional Constraints (if inferable)

Content requirements:
- Be concise and contract-focused; omit incidental or low-level details.
- Do not reference language/library APIs, format strings, or identifiers.
- Describe I/O abstractly; never cite exact parsing patterns or format specifiers.
- Do not restate implementation structure or control flow.
- If behavior is ambiguous, state explicit assumptions.
- Avoid extreme assumptions. Capture only behavior implied by the code's observable contract.
- Return only the nine sections with the exact headings and numbering specified.

Now generate the specification for the code below:

```c
{code}
```"""


SPEC_TO_C_PROMPT_TEMPLATE = """You are a senior systems engineer fluent in C, with strong expertise in program semantics and systems programming.

Task:
Translate the given Specification into C code.

Primary goal:
Preserve semantics based ONLY on the Specification provided. Do not assume any external code or source.

Instructions:
1) Treat the Specification as the only source of truth for behavior.
2) Preserve observable I/O behavior described in the Specification.
3) Preserve corner-case behavior implied by the Specification. For example, if the Specification relies on:
   - integer overflow / wraparound,
   - bit-level tricks,
   - undefined/implementation-defined behavior,
   then emulate the intended behavior as closely as possible in C.
4) If the Specification is incomplete, ambiguous, or inconsistent:
   - do not invent extra functionality
   - choose the narrowest reasonable interpretation justified by the Specification
   - produce compilable C code that stays as faithful as possible to the stated behavior

6) You may add comments as needed to explain non-obvious translation choices, especially if they relate to preserving semantics or handling corner cases.
7) Any explanation must appear only as valid C comments inside the code and not after the code.
8) Return ONLY the C code in a single ```c code block.

Specification:
```text
{spec}
```"""


DIAGNOSIS_PROMPT_TEMPLATE = """Diagnosis generation

You are analyzing a natural-language specification against a source program.

You are given:
1. The source program
2. The current specification derived from the source
3. A failing test case showing that code generated from the specification does not match the source behavior

Your task is NOT to repair the specification yet.
Your task is to infer what this failure suggests about the specification.

Important rules:
- Treat the source program as the only ground truth.
- Treat the failing test only as evidence of possible mismatch.
- Do NOT suggest patching the specification to satisfy this one test case.
- Infer only general issues supported by the source program.
- Do NOT restate the test as a special-case rule.

Please analyze:
- what part of the source behavior is likely missing, vague, or incorrect in the current specification
- whether the failure suggests a missing edge case, missing constraint, wrong abstraction, wrong transition rule, wrong counting rule, wrong termination condition, wrong ordering rule, or wrong assumption

Output format:
A. Likely mismatch category
B. What the current specification may be missing or misstating
C. What general behavior should be clarified in the specification
D. Why this clarification is supported by the source program

Source program:
```c
{original_code}
```

Current specification:
```text
{current_spec}
```

Generated C program from the current specification:
```c
{generated_code}
```

Failing execution report:
```text
{report_text}
```"""


SPEC_FIX_PROMPT_TEMPLATE = """Fix spec

You are revising a natural-language specification for a program.

You are given:
1. The source program
2. The current specification
3. An analysis of likely mismatches between the source and the current specification

Your task:
Revise the specification so that it more faithfully captures the source program.

Important rules:
- The source program is the only ground truth.
- Preserve correct parts of the current specification.
- Revise only parts that are incomplete, ambiguous, over-abstracted, or incorrect.
- Do NOT optimize for any specific failing test.
- Do NOT mention specific test inputs, outputs, or examples in the revised specification.
- Add only behavior justified by the source program.
- Prefer general semantic clarification over ad hoc patching.

Focus especially on clarifying:
- input assumptions and constraints
- edge cases
- state transitions
- counting logic
- ordering/tie-breaking
- termination behavior
- output semantics

Output format:
Just provide the revised specification

Source program:
```c
{original_code}
```

Current specification:
```text
{current_spec}
```

Mismatch analysis:
```text
{diagnosis_text}
```"""
