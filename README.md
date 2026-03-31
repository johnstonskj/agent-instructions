# Agent Instructions

This repository consists of a set of `AGENTS.md` files, along with supplemental artifacts for agentic development. The goal is to be able to pick a language and an agent and have a pair of ready-to-go agent guardrails.

## Languages

Each language **must** provide **at least** an `AGENTS.md` file with the following outline.

```md
1. Your Core Principals
   1. Agent Behavior
2. Preferred Tools
   1. Build
   2. Continuous Integration/Deployment
   3. Logging/Tracing/Metrics
   4. Additional Commons
3.Code Generation
   1. Code Style and Formatting
   2. Documentation
   3. Error Handling
4. Testing
   1. Code Coverage
   2. Benchmarks
5. Version Control
   1. Code Reviews
   2. Commit Checklist
   3. Release Management
6. Documentation
7. Appendix: Use Cases
8. Appendix: File Templates
```

Notes:

- Section 2, on preferred tools,
  - **should** include any additional runtime tools, such as a database or sidecar, or tools such as profilers.
  - **should** include any CI/CD details on specific workflows especially those required for pul-request and merge validation.
- Section 3, on code generation,
  - **should** include as many additional sub-sections to cover considerations when authoring modules, classes, types.
  - **should** include additional sub-sections for performance, security and other non-functional concerns.
- Section 4, on testing,
  - **should** include any requirements for test setup with external dependencies.
  - **should** includre details for any property or fuzz testing tools and workflows.
- Section 7, on use-cases, **should** provide specific additional guardrails based on usage patterns, e.g. tui vs web-ui.
- Section 8, on file templates, **should** provide template files for code generation to use.

### Rust

Use [`rust/AGENTS.md`](./rust/AGENTS.md).

## Agents

Each agent file should include specific guidance based on that agents behavior only.

### Claude

The default behavior for the `CLAUDE.md` file is to reference the `AGENTS.md` file, as in the example [`claude/CLAUDE-DEFAULT.md`](./claude/CLAUDE-DEFAULT.md).
