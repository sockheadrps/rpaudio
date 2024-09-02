import pytest
import datetime

@pytest.hookimpl(tryfirst=True)
def pytest_sessionfinish(session, exitstatus):
    now = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    print(f"\n {now} - All tests completed.")
