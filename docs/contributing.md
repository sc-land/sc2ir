# SC2IR Contributing Guide

Thank you for considering contributing to SC2IR! This document will guide you through the contribution process.

## Getting Started

1. **Fork the repository**: Start by forking the repository on GitHub.

2. **Clone your fork**:
   ```sh
   git clone https://github.com/your-username/sc2ir.git
   cd sc2ir
   ```

3. **Set up the development environment**:
   ```sh
   cargo build
   ```

4. **Run the tests** to make sure everything is working:
   ```sh
   cargo test
   ```

## Development Workflow

1. **Create a new branch** for your feature or bugfix:
   ```sh
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**: Implement your feature or fix the bug.

3. **Follow the coding standards**: Make sure your code follows the [coding standards](./coding_standards.md).

4. **Add or update tests**: Your changes should include tests that verify your new functionality.

5. **Update documentation**: If you're adding new features or changing behavior, update the relevant documentation.

6. **Run the tests** to make sure everything still works:
   ```sh
   cargo test
   ```

7. **Format your code** using rustfmt:
   ```sh
   cargo fmt
   ```

8. **Commit your changes** with a descriptive commit message:
   ```sh
   git commit -m "Add feature: your feature description"
   ```

9. **Push your changes** to your fork:
   ```sh
   git push origin feature/your-feature-name
   ```

10. **Submit a pull request** from your branch to the main repository.

## Pull Request Guidelines

- **One feature per pull request**: Keep pull requests focused on a single feature or bugfix.
- **Describe your changes**: Provide a clear description of what your pull request does.
- **Reference issues**: If your pull request fixes an issue, reference it in the description (e.g., "Fixes #123").
- **Update documentation**: Make sure any new features or changes are documented.
- **Pass all tests**: Your pull request should pass all tests.

## Code Review Process

1. A maintainer will review your pull request.
2. They may ask for changes or clarification.
3. Once approved, your pull request will be merged.

## Reporting Issues

If you find a bug or have a feature request, please open an issue on GitHub:

1. **Check for existing issues**: Make sure your issue hasn't already been reported.
2. **Use a clear title**: Write a title that clearly describes the issue.
3. **Provide details**: Include as much information as possible about the issue.
4. **Steps to reproduce**: If you're reporting a bug, include steps to reproduce it.
5. **Environment information**: Include information about your environment (Rust version, OS, etc.).

## Adding New Conversion Types

If you're adding support for a new conversion type:

1. **Define the trait**: Add a new trait in the `traits` module.
2. **Implement the conversion**: Create an implementation in the appropriate module.
3. **Add tests**: Write tests to verify your implementation.
4. **Update documentation**: Document your new trait and implementation.
5. **Export the trait**: Add the trait to the public exports in `lib.rs`.

## License

By contributing to SC2IR, you agree that your contributions will be licensed under the project's license.
