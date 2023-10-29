# MaidRunner Open Source Project Contribution Guidelines

Welcome to the MaidRunner project! MaidRunner provides capabilities for
tasks such as forensic research, OSINT (Open Source Intelligence),
scanning,
backup and copying, intrusion testing of applications and APIs, and more.

To ensure a smooth and collaborative development process, please
follow the
guidelines outlined below when contributing to MaidRunner.

**Contribution Workflow**

MaidRunner follows a Continuous Integration/Continuous Deployment (CI/CD)
model, replacing the traditional Git Flow model. Our development
process is
centered around: push changes on trunk branch, automated testing and
delivery.

For internal development within your organization, you can leverage
TBD and
build systems running on your servers to speed up the process. This
includes
real-time code reviews using shared programming strategies.

For community contributions, we conduct manual code reviews and approve
merges after evaluating each pull request. The code must meet certain
quality
standards, including unit and integration tests.

If a requirement is not present but a pull request adds substantial
value, we
will analyze it and attempt to implement the rest of the
requirements. However,
maintaining good coding practices and consistency are non-negotiable.

**Trunk-Based Development**

This project utilizes a Trunk-Based Development (TBD) approach for
its software
development lifecycle. Trunk-Based Development is a software development
methodology that emphasizes continuous integration and collaboration among
developers. Here's a brief overview of how we implement TBD in this
project:
Branching Strategy

*"In Trunk-Based Development, we maintain a single long-lived branch
called
the "trunk" or "trunk" branch. All developers work directly on this branch
for most of their coding tasks."*

We greatly appreciate your dedication and contributions to our project. To
ensure that we maintain the highest standards of development practices
and align with our chosen methodology, Trunk-Based Development (TBD), we
kindly request that you take a moment to review the content available on
trunkbaseddevelopment.com before submitting any changes.

**Here's why this is important:**

- Alignment with Best Practices: Trunk-Based Development (TBD) is
a cornerstone
of our development process. The website provides comprehensive insights
into
TBD principles, benefits, and practical implementations that can greatly
benefit your contributions.

- Efficient Collaboration: Understanding TBD principles helps us work
together
more efficiently, reducing the likelihood of merge conflicts and ensuring
a smoother development process.

- Quality Assurance: By familiarizing yourself with TBD, you can
ensure that
your changes align with the methodology's principles, leading to
higher code
quality and better integration into the main branch.

- Reduced Disruptions: Proper adherence to TBD minimizes disruptions
to the
main branch, allowing us to maintain a stable and deployable codebase at
all times.

We believe that taking the time to review the resources available on
<a href="http://trunkbaseddevelopment.com">trunk-based development
website</a>
will enhance the quality of your contributions and ultimately benefit the
entire project.

**If you have any questions about our project, please feel free to open an
issue on github.**

**Branch Strategy for Community**

- Create a new branch for your feature or bug fix based on the trunk
branch.

  - Branch naming convention: feature/feature_name or
  bugfix/bug_description

- Develop your feature or fix within your branch.

- Ensure that your code changes are accompanied by appropriate unit tests
and documentation.

- Commit your changes with clear, concise commit messages following our
commit message conventions.

- Push your feature branch to the remote repository.

- Create a pull request (PR) targeting the trunk branch.
        Ensure that the PR description includes details about the changes
        made and the problem they solve.

- The CI/CD pipeline will automatically run tests and checks on your PR.

- Address any feedback or issues raised in the PR review.

- Once the PR passes all tests and reviews, it will be merged into
the trunk
branch by the project maintainers.

**Commit Message Conventions**

*When making commits, please follow these guidelines for clear and
informative
commit messages:*

- Use the present tense: "Add feature" not "Added feature" or "Adding
feature."
- Use imperative mood: "Fix bug" not "Fixes bug" or "Fixed bug."

Example commit messages:

- "Update documentation for new feature"
- "Fix issue with file validation in backup module"
- "Add unit tests for API scanning functionality"

**Getting Started**

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

Make your changes and commit them following the commit message
conventions.

Push your branch to your GitHub fork:

``` bash
git push origin feature/your-feature
```

Create a pull request to the trunk branch of the main MaidRunner
repository.

**Code of Conduct**

Please adhere to our Code of Conduct when participating in the MaidRunner
community. We promote a welcoming and inclusive environment for all
contributors.

Thank you for contributing to MaidRunner.

Note: This Contribution Guidelines document is subject to change, so make
sure to check it regularly for updates.
