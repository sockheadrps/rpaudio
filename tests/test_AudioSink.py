import pytest
import asyncio
from unittest.mock import MagicMock
import rpaudio

@pytest.fixture
def audio_handler():
    mock_callback = MagicMock()

    handler = rpaudio.AudioSink(callback=mock_callback)
    handler.load_audio(r"examples/ex.wav")

    return handler, mock_callback

@pytest.mark.asyncio
async def test_play_audio(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    assert handler.is_playing is True

@pytest.mark.asyncio
async def test_pause(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.pause()
    await asyncio.sleep(0.1)
    assert handler.is_playing is False

@pytest.mark.asyncio
async def test_resume(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.pause()
    await asyncio.sleep(0.1)
    handler.play()
    await asyncio.sleep(0.1)
    assert handler.is_playing is True

@pytest.mark.asyncio
async def test_set_volume(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.set_volume(0.5)
    await asyncio.sleep(0.1)
    assert handler.get_volume() == 0.5

@pytest.mark.asyncio
async def test_try_seek(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.try_seek(4)
    await asyncio.sleep(0.1)
    assert handler.get_pos() >= 4

@pytest.mark.asyncio
async def test_get_pos(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    pos = handler.get_pos()
    assert pos >= 0

@pytest.mark.asyncio
async def test_set_speed(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.set_speed(1.5)
    await asyncio.sleep(0.1)
    assert handler.get_speed() == 1.5

@pytest.mark.asyncio
async def test_get_speed(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    speed = handler.get_speed()
    assert speed >= 1.0

@pytest.mark.asyncio
async def test_stop(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.stop()
    await asyncio.sleep(0.1)
    assert handler.is_playing is False

@pytest.mark.asyncio
async def test_metadata(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    metadata = handler.metadata
    assert isinstance(metadata, dict)
    assert 'album_artist' in metadata
    assert 'album_title' in metadata
    assert 'artist' in metadata
    assert 'duration' in metadata
    assert 'channels' in metadata

@pytest.mark.asyncio
async def test_callback_called(audio_handler):
    handler, mock_callback = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.stop()
    await asyncio.sleep(0.1)
    mock_callback.assert_called_once()
