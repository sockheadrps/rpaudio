import json
import re
import tomllib


def get_version_from_pyproject():
    with open('pyproject.toml', 'rb') as f:
        toml_data = tomllib.load(f)
        return toml_data['project']['version']


def pytest_sessionfinish(session, exitstatus):
    report_file = "tests/report.json"
    readme_file = "README.md"

    with open(report_file, 'r') as f:
        data = json.load(f)

    total_tests = data['summary']['total']
    passed_tests = data['summary'].get('passed', 0)

    coverage = f"{passed_tests}/{total_tests}"

    version = get_version_from_pyproject()

    pytest_badge_pattern = r"!\[Pytest\].*"
    version_badge_pattern = r"!\[Version\].*"

    new_pytest_badge = f"![Pytest](https://img.shields.io/badge/Pytest-{coverage}-brightgreen)"
    new_version_badge = f"![Version](https://img.shields.io/badge/Version-{version}-blue)"

    with open(readme_file, 'r') as f:
        readme_content = f.read()

    updated_content = re.sub(pytest_badge_pattern,
                             new_pytest_badge, readme_content)

    if re.search(version_badge_pattern, updated_content):
        updated_content = re.sub(
            version_badge_pattern, new_version_badge, updated_content)
    else:
        updated_content = new_pytest_badge + "\n" + \
            new_version_badge + "\n" + updated_content

    with open(readme_file, 'w') as f:
        f.write(updated_content)

    print(
        f"Updated README.md with Pytest coverage: {coverage} and Version: {version}")
