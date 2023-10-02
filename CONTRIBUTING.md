# MaidRunner Open Source Project Contribution Guidelines

Welcome to the MaidRunner open source project! We're excited to have you contribute to our versatile task automation software designed to serve as the foundation for various cybersecurity modules. MaidRunner provides capabilities for tasks such as forensic research, OSINT (Open Source Intelligence), scanning, backup and copying, intrusion testing of applications and APIs, and more.

To ensure a smooth and collaborative development process, please follow the guidelines outlined below when contributing to MaidRunner.

### Contribution Workflow

MaidRunner follows a Continuous Integration/Continuous Deployment (CI/CD) model, replacing the traditional Git Flow model. Our development process is centered around creating feature branches, automated testing, and merging changes into the master branch.

### Branch Strategy

- Create a new branch for your feature or bug fix based on the master branch.

  - Branch naming convention: feature/feature_name or bugfix/bug_description

- Develop your feature or fix within your branch.

- Ensure that your code changes are accompanied by appropriate unit tests and documentation.

- Commit your changes with clear, concise commit messages following our commit message conventions.

- Push your feature branch to the remote repository.

- Create a pull request (PR) targeting the master branch.
        Ensure that the PR description includes details about the changes made and the problem they solve.

- The CI/CD pipeline will automatically run tests and checks on your PR.

- Address any feedback or issues raised in the PR review.

- Once the PR passes all tests and reviews, it will be merged into the master branch by the project maintainers.

### Commit Message Conventions

*When making commits, please follow these guidelines for clear and informative commit messages:*

- Use the present tense: "Add feature" not "Added feature" or "Adding feature."
- Use imperative mood: "Fix bug" not "Fixes bug" or "Fixed bug."

Example commit messages:

- "Update documentation for new feature"
- "Fix issue with file validation in backup module"
- "Add unit tests for API scanning functionality"

### Getting Started

To get started with contributing to MaidRunner, follow these steps:

1. Fork the MaidRunner repository on GitHub.
2. Clone your forked repository to your local machine:

``` bash
git clone <https://github.com/th3maid/MaidRunner.git>
```

Create a new branch for your contribution:

``` bash
git checkout -b feature/your-feature
```

Make your changes and commit them following the commit message conventions.

Push your branch to your GitHub fork:

``` bash
git push origin feature/your-feature
```

Create a pull request to the master branch of the main MaidRunner repository.

### Code of Conduct

Please adhere to our Code of Conduct when participating in the MaidRunner community. We promote a welcoming and inclusive environment for all contributors.

Thank you for contributing to MaidRunner.

Note: This Contribution Guidelines document is subject to change, so make sure to check it regularly for updates.
