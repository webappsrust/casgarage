# Contributing to CasGarage

Thank you for your interest in contributing to CasGarage! We welcome contributions from the community.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and professional in all interactions.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/casapps/casgarage/issues)
2. If not, create a new issue with:
   - Clear description of the problem
   - Steps to reproduce
   - Expected vs actual behavior
   - System information (OS, version, etc.)
   - Relevant logs

### Suggesting Features

1. Check existing [Issues](https://github.com/casapps/casgarage/issues) for similar suggestions
2. Create a new issue with:
   - Clear description of the feature
   - Use case and benefits
   - Possible implementation approach

### Pull Requests

1. **Fork the repository**
   ```bash
   git clone https://github.com/casapps/casgarage.git
   cd casgarage
   git checkout -b feature/your-feature-name
   ```

2. **Set up development environment**
   ```bash
   docker-compose up -d
   ```

3. **Make your changes**
   - Write clean, documented code
   - Follow Rust best practices
   - Add tests for new functionality
   - Update documentation as needed
   - No inline comments for future work - use TODO.md

4. **Test your changes**
   ```bash
   ./scripts/test.sh
   cargo fmt -- --check
   cargo clippy -- -D warnings
   ```

5. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: add new feature"
   ```

   Follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation changes
   - `style:` - Code style changes (formatting)
   - `refactor:` - Code refactoring
   - `test:` - Adding or updating tests
   - `chore:` - Maintenance tasks

6. **Push and create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a pull request on GitHub.

## Development Guidelines

### Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Follow Rust naming conventions
- Keep functions small and focused
- Document public APIs

### Project Structure

```
casgarage/
├── src/           # Rust backend source
├── frontend/      # Leptos WASM frontend
├── tests/         # All test files
├── scripts/       # Build and deployment scripts
├── docker/        # Docker configurations
├── k8s/           # Kubernetes manifests
├── docs/          # Documentation
└── TODO.md        # Project todos (NO inline TODOs)
```

### Testing

- Write unit tests for new functions
- Add integration tests for new features
- Ensure all tests pass: `cargo test`
- Aim for high test coverage

### Documentation

- Update CLAUDE.md for architectural changes
- Update relevant docs/ files
- Add inline documentation for public APIs
- Update README.md if user-facing changes

### Docker Development

All development should use Docker:

```bash
# Development server with hot reload
docker-compose up

# Run tests
docker-compose -f docker-compose.test.yml up

# Build production image
docker build -t casgarage:local .
```

### Temporary Files

All temporary files must use `/tmp/casgarage` directory:
- Never use system `/tmp` directly
- Always scope to project temporary directory
- Clean up temporary files appropriately

### TODO Management

- **NO inline comments with "TODO", "FIXME", "HACK", etc.**
- All future work goes in `TODO.md`
- Keep `TODO.md` synchronized with actual work
- Format: `- [ ] Description (owner, priority)`

## Review Process

1. Automated checks must pass (CI/CD)
2. Code review by maintainer(s)
3. Address review feedback
4. Approval and merge

## Questions?

- Open a [Discussion](https://github.com/casapps/casgarage/discussions)
- Join our community channels
- Email: casjay@yahoo.com

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

Thank you for contributing to CasGarage! 🎉
