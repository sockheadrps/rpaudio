import pytest
import asyncio
from unittest.mock import MagicMock
import rpaudio

@pytest.fixture
def audio_channel():
    # Create mock callbacks
    mock_callback_1 = MagicMock()
    mock_callback_2 = MagicMock()

    audio_1 = rpaudio.AudioSink(callback=mock_callback_1)
    audio_1.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\ex.wav")
    
    audio_2 = rpaudio.AudioSink(callback=mock_callback_2)
    audio_2.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\Acrylic.mp3")

    channel_1 = rpaudio.AudioChannel()
    channel_1.auto_consume = True
    channel_1.push(audio_1)
    channel_1.push(audio_2)

    return channel_1, mock_callback_1, mock_callback_2

@pytest.mark.asyncio
async def test_play_audio(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    assert channel.current_audio.is_playing is True

@pytest.mark.asyncio
async def test_pause(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.current_audio.pause()
    await asyncio.sleep(0.1)
    assert channel.current_audio.is_playing is False

@pytest.mark.asyncio
async def test_resume(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.current_audio.pause()
    await asyncio.sleep(0.1)
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    assert channel.current_audio.is_playing is True

@pytest.mark.asyncio
async def test_set_volume(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.current_audio.set_volume(0.5)
    await asyncio.sleep(0.1)
    assert channel.current_audio.get_volume() == 0.5

@pytest.mark.asyncio
async def test_try_seek(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.current_audio.try_seek(2)
    await asyncio.sleep(0.1)
    assert channel.current_audio.get_pos() >= 2

@pytest.mark.asyncio
async def test_get_pos(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    pos = channel.current_audio.get_pos()
    assert pos >= 0

@pytest.mark.asyncio
async def test_set_speed(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.current_audio.set_speed(1.5)
    await asyncio.sleep(0.1)
    assert channel.current_audio.get_speed() == 1.5

@pytest.mark.asyncio
async def test_get_speed(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    speed = channel.current_audio.get_speed()
    assert speed >= 1.0

@pytest.mark.asyncio
async def test_stop(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    while channel.current_audio is not None:
        channel.current_audio.stop()
        await asyncio.sleep(0.1)

    assert channel.current_audio is None

@pytest.mark.asyncio
async def test_callbacks_called(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    # Play the first audio and stop it to trigger the callback
    channel.current_audio.play()
    while channel.current_audio is not None:
        channel.current_audio.stop()
        await asyncio.sleep(0.1)


    # Assert both callbacks were called
    mock_callback_1.assert_called_once()
    mock_callback_2.assert_called_once()
