# Sanity

Sanity is a language agnostic program that is designed to run as part of your
project's test or deployment infrastructure.

# Installation

```
brew tap jsyeo/sanity
brew install sanity
```

# Usage

To use sanity in your project, simply run `sanity init` to create an empty
`.sanity.yml` configuration file. This file contains the phases and commands
that you wish to run.

```
sanity init
```

Once you have defined the commands that you want to run, execute `sanity` to
run them.

## Example `.sanity.yml` Files

### JavaScript

```
install: npm install
lint: npm lint
unit: npm test
```

### Ruby

```
install: bundle install
syntax:
  - rubocop
  - reek
unit: rspec
security: brakeman
```

### Java

```
install: mvn install
syntax: mvn compile
unit: mvn test
```
