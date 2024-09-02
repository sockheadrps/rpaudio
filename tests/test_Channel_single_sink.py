import asyncio
from unittest.mock import MagicMock
import pytest
import rpaudio


@pytest.fixture
def audio_channel():
    """Fixture to create an AudioChannel with two AudioSinks."""
    mock_callback_1 = MagicMock()
    mock_callback_2 = MagicMock()

    audio_1 = rpaudio.AudioSink(callback=mock_callback_1)
    audio_1.load_audio(r"examples/ex.wav")

    audio_2 = rpaudio.AudioSink(callback=mock_callback_2)
    audio_2.load_audio(r"examples/ex.wav")

    channel_1 = rpaudio.AudioChannel()
    channel_1.auto_consume = True
    channel_1.push(audio_1)
    channel_1.push(audio_2)

    return channel_1, mock_callback_1, mock_callback_2


@pytest.mark.asyncio
async def test_play_audio(audio_channel):
    """Test that audio starts playing."""
    channel, _, _ = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    assert channel.current_audio.is_playing is True


@pytest.mark.asyncio
async def test_pause(audio_channel):
    """Test pausing audio."""
    channel, _, _ = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.current_audio.pause()
    await asyncio.sleep(0.1)
    assert channel.current_audio.is_playing is False


@pytest.mark.asyncio
async def test_resume(audio_channel):
    """Test resuming audio."""
    channel, _, _ = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.current_audio.pause()
    await asyncio.sleep(0.1)
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    assert channel.current_audio.is_playing is True


@pytest.mark.asyncio
async def test_set_volume(audio_channel):
    """Test setting the volume of audio."""
    channel, _, _ = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.current_audio.set_volume(0.5)
    await asyncio.sleep(0.1)
    assert channel.current_audio.get_volume() == 0.5


@pytest.mark.asyncio
async def test_try_seek(audio_channel):
    """Test seeking to a specific position in the audio."""
    channel, _, _ = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.current_audio.try_seek(4)
    await asyncio.sleep(0.1)
    assert channel.current_audio.get_pos() >= 2


@pytest.mark.asyncio
async def test_get_pos(audio_channel):
    """Test getting the current position of the audio."""
    channel, _, _ = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    pos = channel.current_audio.get_pos()
    assert pos >= 0


@pytest.mark.asyncio
async def test_set_speed(audio_channel):
    """Test setting the playback speed of the audio."""
    channel, _, _ = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.current_audio.set_speed(1.5)
    await asyncio.sleep(0.1)
    assert channel.current_audio.get_speed() == 1.5


@pytest.mark.asyncio
async def test_get_speed(audio_channel):
    """Test getting the playback speed of the audio."""
    channel, _, _ = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    speed = channel.current_audio.get_speed()
    assert speed >= 1.0


@pytest.mark.asyncio
async def test_stop(audio_channel):
    """Test stopping the audio playback."""
    channel, _, _ = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    while channel.current_audio is not None:
        channel.current_audio.stop()
        await asyncio.sleep(0.1)
    assert channel.current_audio is None


@pytest.mark.asyncio
async def test_callbacks_called(audio_channel):
    """Test that callbacks are called when audio is played."""
    channel, mock_callback_1, mock_callback_2 = audio_channel
    channel.current_audio.play()
    while channel.current_audio is not None:
        channel.current_audio.stop()
        await asyncio.sleep(0.1)

    mock_callback_1.assert_called_once()
    mock_callback_2.assert_called_once()
