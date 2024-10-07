import pytest
import datetime

@pytest.hookimpl(tryfirst=True)
def pytest_sessionfinish(session, exitstatus):
    now_utc = datetime.datetime.utcnow()
    unix_timestamp = int(now_utc.timestamp())
    print(f"\n {unix_timestamp} - All tests completed.")
