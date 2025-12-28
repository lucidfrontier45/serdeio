# Development Workflow and Task Completion

## Task Completion Checklist
After making any changes to the codebase, follow this mandatory sequence:

### 1. Code Validation
```bash
cargo check
```
- Verify the code compiles without errors
- Check all imports and references are correct

### 2. Linting and Code Quality
```bash
cargo clippy -- -D warnings
```
- Fix all clippy warnings (treat warnings as errors)
- Pay attention to performance and style suggestions
- Ensure code follows Rust best practices

### 3. Code Formatting
```bash
cargo fmt
```
- Format code according to rustfmt standards
- Ensure consistent indentation and style
- Verify no formatting conflicts

### 4. Testing
```bash
cargo test
```
- Run all unit tests
- Verify new functionality works as expected
- Ensure no regressions in existing functionality

### 5. Feature Testing (if applicable)
```bash
cargo test --all-features
cargo test --features csv
cargo test --features yaml
```
- Test with different feature combinations
- Verify feature-gated code works correctly
- Ensure no feature-specific regressions

## Development Guidelines

### Making Changes
1. **Understand the Context**: Read existing code patterns before making changes
2. **Follow Conventions**: Adhere to established coding standards (see code_style_conventions.md)
3. **Add Tests**: Include comprehensive tests for new functionality
4. **Feature Gates**: Properly gate optional dependencies with feature flags
5. **Error Handling**: Use anyhow for consistent error management

### Testing Strategy
- **Unit Tests**: Every module should have comprehensive unit tests
- **Integration Tests**: Test interactions between components
- **Edge Cases**: Include tests for error conditions and edge cases
- **Cursor Testing**: Use `std::io::Cursor` for in-memory testing
- **Test Data**: Use raw string literals with `r#"...""#` for test data

### Code Review Points
- **API Consistency**: Ensure new code follows existing API patterns
- **Error Handling**: Verify proper error propagation and context
- **Performance**: Check for unnecessary allocations or inefficient patterns
- **Documentation**: Ensure public APIs are properly documented
- **Feature Flags**: Verify optional code is properly feature-gated

### Common Pitfalls to Avoid
- Don't use `unwrap()` in library code - prefer proper error handling
- Don't forget to feature-gate optional dependencies
- Don't ignore clippy warnings - they often indicate real issues
- Don't forget to include test cases for error conditions
- Don't break the consistent API pattern across backends

## Git Workflow (if committing)
```bash
git status
git add .
git commit -m "descriptive commit message"
git push
```

Note: Only commit when explicitly requested by the user.

## Emergency Rollback
If changes break existing functionality:
1. Identify the problematic commit using `git log`
2. Revert the changes: `git revert <commit-hash>`
3. Run the full test suite to verify recovery
4. Investigate root cause before reapplying changes