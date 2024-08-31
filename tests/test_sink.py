import pytest
import asyncio
import rpaudio

def on_audio_stop():
    print("Audio has stopped")

@pytest.fixture
def audio_handler():
    handler = rpaudio.AudioSink(callback=on_audio_stop)
    handler.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\Acrylic.mp3")
    return handler

@pytest.mark.asyncio
async def test_play_audio(audio_handler):
    audio_handler.play()
    await asyncio.sleep(0.1)
    assert audio_handler.is_playing is True

@pytest.mark.asyncio
async def test_pause(audio_handler):
    audio_handler.play()
    await asyncio.sleep(0.1)
    audio_handler.pause()
    await asyncio.sleep(0.1)
    assert audio_handler.is_playing is False

@pytest.mark.asyncio
async def test_resume(audio_handler):
    audio_handler.play()
    await asyncio.sleep(0.1)
    audio_handler.pause()
    await asyncio.sleep(0.1)
    audio_handler.play()
    await asyncio.sleep(0.1)
    assert audio_handler.is_playing is True

@pytest.mark.asyncio
async def test_set_volume(audio_handler):
    audio_handler.play()
    await asyncio.sleep(0.1)
    audio_handler.set_volume(0.5)
    await asyncio.sleep(0.1)
    assert audio_handler.get_volume() == 0.5

@pytest.mark.asyncio
async def test_try_seek(audio_handler):
    audio_handler.play()
    await asyncio.sleep(0.1)
    audio_handler.try_seek(33.5)
    await asyncio.sleep(0.1)
    assert audio_handler.get_pos() >= 33.5

@pytest.mark.asyncio
async def test_get_pos(audio_handler):
    audio_handler.play()
    await asyncio.sleep(0.1)
    pos = audio_handler.get_pos()
    assert pos >= 0

@pytest.mark.asyncio
async def test_set_speed(audio_handler):
    audio_handler.play()
    await asyncio.sleep(0.1)
    audio_handler.set_speed(1.5)
    await asyncio.sleep(0.1)
    assert audio_handler.get_speed() == 1.5

@pytest.mark.asyncio
async def test_get_speed(audio_handler):
    audio_handler.play()
    await asyncio.sleep(0.1)
    speed = audio_handler.get_speed()
    assert speed >= 1.0

@pytest.mark.asyncio
async def test_stop(audio_handler):
    audio_handler.play()
    await asyncio.sleep(0.1)
    audio_handler.stop()
    await asyncio.sleep(0.1)
    assert audio_handler.is_playing is False
