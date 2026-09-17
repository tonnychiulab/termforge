---
name: secure-by-construction
description: >
  Use when implementing a feature, adding a web page or frontend, adding CI
  or GitHub Actions, handling input, WASM, or writing an OpenSpec change
  (實作、加網頁、加前端、加 CI、處理輸入). Do not use when the user asks for a
  code review, security audit, or penetration test.
---

# Secure by Construction

Security scenes exist **before** implementation code. Read the OpenSpec baseline; do not re-run a CIA audit.

## Phase 1 — Load

**Entry:** about to change code, `dist/`, CI, WASM, input handling, or OpenSpec.

1. Read `openspec/specs/secure-by-construction.md`.
2. Identify the OpenSpec change this work belongs to under `openspec/changes/`.
3. Read that change's proposal, design, and tasks for 適用／不適用.

**Exit:** you have the seven clauses and this change's marks, or you know there is no change.

## Phase 2 — Gate

**Entry:** Phase 1 done.

1. No OpenSpec change for this work? **STOP.** Do not write implementation.
2. Any of the seven clauses neither 適用 nor 不適用 with a reason? **STOP.**
3. 適用 but no GIVEN/WHEN/THEN about **this** change? **STOP.**
4. About to complete feature tasks while 適用 security tasks are empty? **STOP.**

**Exit:** every clause is marked and 適用 scenes exist. Only then implement.

## Phase 3 — Implement

**Entry:** Phase 2 passed.

1. Implement only what the scenes allow.
2. Do not start a CIA report, Open Code Review, Codex Security, or a review skill. Those are after-the-fact nets (CI may run OCR on the PR).

**Exit:** the diff matches the scenes.

## Rationalizations

| Excuse | Reality |
|---|---|
| 很趕 / 不要開 OpenSpec | Missing scenes means stop, not skip. |
| OpenSpec 之後再補 | Feature tasks cannot finish while security tasks are empty. |
| 維護者叫我跳過 | The baseline still applies. |
| 這只是小遊戲所以不必限 | The spec forbids that exemption unless 不適用 names a reason. |
| 先改 HTML／workflow 再補規格 | If implementation landed first, revert it. Open the change first. |
| 使用者要 code review | Wrong skill. Use the review or audit skill. |

## Red flags — STOP

- jsDelivr / unpkg / other CDN as a runtime script or stylesheet
- `_headers` treated as GitHub Pages security
- Deploy path in docs or CI that is not the real Pages source
- Token, `.env`, or private key in the diff
- 「之後再補安全」
