<!-- omit in toc -->
# Contributing to Software-Challenge Python Client

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions. 🎉

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support the project and show your appreciation, which we would also be very happy about:
> - Star the project
> - Refer this project in your project's readme

<!-- omit in toc -->
## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [I Have a Question](#i-have-a-question)
- [I Want To Contribute](#i-want-to-contribute)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Enhancements](#suggesting-enhancements)
- [Your First Code Contribution](#your-first-code-contribution)
- [Improving The Documentation](#improving-the-documentation)
- [Styleguides](#styleguides)
- [Commit Messages](#commit-messages)


## Code of Conduct

This project and everyone participating in it is governed by the
[Software-Challenge Python Client Code of Conduct](https://github.com/FalconsSky/socha-python-clientblob/master/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
to .


## I Have a Question

> If you want to ask a question, we assume that you have read the available [Documentation]().

Before you ask a question, it is best to search for existing [Issues](https://github.com/FalconsSky/socha-python-client/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/FalconsSky/socha-python-client/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions (nodejs, npm, etc), depending on what seems relevant.

We will then take care of the issue as soon as possible.

<!--
You might want to create a separate issue tag for questions and include it in this description. People should then tag their issues accordingly.

Depending on how large the project is, you may want to outsource the questioning, e.g. to Stack Overflow or Gitter. You may add additional contact and information possibilities:
- IRC
- Slack
- Gitter
- Stack Overflow tag
- Blog
- FAQ
- Roadmap
- E-Mail List
- Forum
-->

## I Want To Contribute

> ### Legal Notice <!-- omit in toc -->
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.

### Reporting Bugs

<!-- omit in toc -->
#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions (Make sure that you have read the [documentation](). If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/FalconsSky/socha-python-clientissues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside of the GitHub community have discussed the issue.
- Collect information about the bug:
- Stack trace (Traceback)
- OS, Platform and Version (Windows, Linux, macOS, x86, ARM)
- Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
- Possibly your input and the output
- Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->
#### How Do I Submit a Good Bug Report?

> You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue tracker, or elsewhere in public. Instead sensitive bugs must be sent by email to .
<!-- You may add a PGP key to allow the messages to be sent encrypted as well. -->

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/FalconsSky/socha-python-client/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the *reproduction steps* that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).

<!-- You might want to create an issue template for bugs and errors that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->


### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for Software-Challenge Python Client, **including completely new features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the community to understand your suggestion and find related suggestions.

<!-- omit in toc -->
#### Before Submitting an Enhancement

- Make sure that you are using the latest version.
- Read the [documentation]() carefully and find out if the functionality is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/FalconsSky/socha-python-client/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to you to make a strong case to convince the project's developers of the merits of this feature. Keep in mind that we want features that will be useful to the majority of our users and not just a small subset. If you're just targeting a minority of users, consider writing an add-on/plugin library.

<!-- omit in toc -->
#### How Do I Submit a Good Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/FalconsSky/socha-python-client/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why. At this point you can also tell which alternatives do not work for you.
- You may want to **include screenshots and animated GIFs** which help you demonstrate the steps or point out the part which the suggestion is related to. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and [this tool](https://github.com/colinkeenan/silentcast) or [this tool](https://github.com/GNOME/byzanz) on Linux. <!-- this should only be included if the project has a GUI -->
- **Explain why this enhancement would be useful** to most Software-Challenge Python Client users. You may also want to point out the other projects that solved it better and which could serve as inspiration.

<!-- You might want to create an issue template for enhancement suggestions that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Your First Code Contribution
- Familiarize yourself with the project: Before you make your first contribution, take some time to understand the project and its goals. Read through the documentation and the source code to get a feel for the project structure and coding style.

- Find an issue to work on: Look through the open issues in the project's repository to find something that interests you or that you feel you can tackle. If you can't find an issue that suits your skills, consider creating a new issue to suggest a feature or fix a bug.

- Fork the repository: Fork the project repository to your own GitHub account so you can make changes to the code.

- Create a new branch: Create a new branch for your changes, using a descriptive name that makes it clear what you're working on.

- Make your changes: Make the changes you want to contribute, following the project's coding style and commenting your code appropriately.

- Test your changes: Test your changes thoroughly to ensure that they work as expected and don't break any existing functionality.

- Commit and push your changes: When you're happy with your changes, commit them to your branch and push them to your fork.

- Submit a pull request: Submit a pull request to the project's repository, describing the changes you've made and explaining why they're useful.

- Wait for feedback: The project maintainers will review your pull request and provide feedback or suggest changes. Be open to feedback and willing to make changes to your code if necessary.

- Celebrate: Congratulations! You've made your first code contribution to an open-source project!

By following these steps, you can make your first code contribution to a project and start building your skills as a developer. Good luck, and happy coding!

### Improving The Documentation

- Familiarize yourself with the project's documentation: Before you start making improvements to the documentation, read through it thoroughly to understand what's already there and what needs improvement.

- Identify areas for improvement: Look for areas where the documentation could be clearer, more complete, or more user-friendly. Consider areas where examples or additional explanations would be helpful.

- Make a plan: Decide what changes you want to make and how you will make them. Consider breaking down the documentation into smaller, manageable pieces if the changes are extensive.

- Write clear, concise documentation: When making changes to the documentation, aim for clarity and conciseness. Use simple language and clear examples to help users understand the information.

- Test the documentation: Test the documentation to make sure that it's accurate and effective. Ask others to review it as well, and incorporate their feedback as necessary.

- Submit your changes: Submit your changes to the project's repository, following the same steps as for making a code contribution.

- Wait for feedback: The project maintainers will review your changes and provide feedback or suggest changes. Be open to feedback and willing to make changes to your documentation if necessary.

By making improvements to the documentation, you can help others understand and use the project more easily. Your contributions will be invaluable in making the project a success!

## Styleguides
The following styleguides should be followed when contributing code to this project:

- PEP 8: Follow the guidelines set forth in the Python Enhancement Proposal 8 (PEP 8) for code style, including indentation, line length, naming conventions, and whitespace usage.

- Docstrings: Include docstrings for all functions and classes, following the guidelines in PEP 257. The docstrings should provide clear and concise explanations of the purpose and behavior of the code.

- Comments: Use comments liberally to explain your code, especially in complex or non-obvious sections. The comments should be written in clear, concise language and should explain the purpose of the code, not simply repeat what the code is doing.

- Testing: Write comprehensive tests for your code to ensure that it works as expected and that any changes you make do not break existing functionality.

- Code Reviews: Be open to code reviews from other contributors and be willing to incorporate feedback into your code.

By following these styleguides, you can help maintain a consistent and high-quality codebase for this project.

### Commit Messages
- Use the present tense: Write your commit messages in the present tense, using phrases like "change" or "add" instead of "changed" or "added".

- Follow the Karma Runner format: Use the following format for your commit messages: `<type>(<scope>): <subject>`. 
  The `<type>` should describe the nature of the change (e.g. `fix`, `feat`, `docs`, `style`, etc.). The `<scope>` should describe the part of the code affected by the change, 
  and the `<subject>` should provide a brief description of the change.

- Limit the subject line to 72 characters: Keep the subject line of your commit message concise and to the point, using no more than 72 characters.

- Provide more detail in the body: If necessary, provide additional detail about the changes in the body of the commit message, 
  separated from the subject line by a blank line.

- Reference issues: If your changes are related to a specific issue or bug, include a reference to that issue in your commit message.

Example:
  ```
  fix(authentication system): validate users before logging in

  The authentication system was failing to properly validate users, causing them to be unable to log in. 
  This change adds additional checks to the authentication process to ensure that users are correctly validated.

  Fixes #123.
  ```
By following the Karma Runner convention for commit messages, you can make it easier for others to understand the changes you've made and why. 
This will help maintain the project's history and make it easier for others to contribute to the project in the future.
