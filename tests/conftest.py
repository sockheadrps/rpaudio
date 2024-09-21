import pytest
import datetime

@pytest.hookimpl(tryfirst=True)
def pytest_sessionfinish(session, exitstatus):
    # Get the current UTC time as a Unix timestamp
    now_utc = datetime.datetime.utcnow()
    unix_timestamp = int(now_utc.timestamp())
    formatted_time = now_utc.strftime("%Y-%m-%d %H:%M:%S")
    
    print(f"\n {unix_timestamp} - All tests completed.")
