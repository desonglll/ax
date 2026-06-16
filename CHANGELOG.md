# Changelog

This document logs the development history and version alterations of Project Ax.

## [0.2.0] - 2026-06-16

### Added
- Created `justfile` in the project root to automate database setup, testing, compiling, and documentation.
- Integrated `mdBook` documentation inside the `docs` directory, detailing the design, API reference, database schema, and developer guidelines.
- Created `CHANGELOG.md` to keep record of modifications.
- Introduced safety checks on the sorting column parameter `order_by` and order parameter `sort` to mitigate SQL injection.
- Added comprehensive test coverage validating safety check behaviors, unauthorized request blocking, and comment pagination.

### Changed
- Ported all source code comments from Chinese to GNU-style English.
- Refactored `get_comment_by_query` and `get_comment_by_query_db` to support pagination parameters (`limit` and `offset`).

### Fixed
- Fixed unauthenticated access bypass in comment and post handlers where the result of `login_in_unauthentic` was discarded.
- Fixed route parameter definition for comment deletion by appending `/delete/{id}` placeholder.
- Fixed panics caused by unwrapping empty query parameter options in backend handlers.
- Fixed a trigger logic error where reactions on comments updated the reaction counts of posts containing the same numerical ID.
- Fixed a trigger logic error where `average_comment_count` was calculated using post dislikes instead of comment count statistics.
- Fixed user stats updates to properly trigger and calculate metrics when posts are deleted.
- Fixed GitHub Actions CI workflow to target the correct database name (`ax`), install development tools via precompiled binaries (`taiki-e/install-action`), utilize dependency caching (`swatinem/rust-cache`), and execute validation tasks using `just` recipes.

### Removed
- Deleted obsolete Bash script `scripts/init_db.sh`.
