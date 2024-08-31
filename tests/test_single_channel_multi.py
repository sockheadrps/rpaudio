import pytest
import asyncio
from unittest.mock import MagicMock
import rpaudio

@pytest.fixture
def audio_channel():
    mock_callback_1 = MagicMock()
    mock_callback_2 = MagicMock()

    audio_1 = rpaudio.AudioSink(callback=mock_callback_1)
    audio_1.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\ex.wav")
    
    channel = rpaudio.AudioChannel()
    channel.auto_consume = True
    channel.push(audio_1)

    return channel, mock_callback_1, mock_callback_2

@pytest.mark.asyncio
async def test_initialization(audio_channel):
    channel, _, _ = audio_channel
    assert isinstance(channel, rpaudio.AudioChannel)
    assert channel.auto_consume is True

@pytest.mark.asyncio
async def test_push_audio(audio_channel):
    channel, _, mock_callback_2 = audio_channel
    chan_len = len(channel.queue_contents)
    audio_2 = rpaudio.AudioSink(callback=mock_callback_2)
    audio_2.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\Acrylic.mp3")
    channel.push(audio_2)
    assert len(channel.queue_contents) == chan_len + 1

@pytest.mark.asyncio
async def test_auto_consume(audio_channel):
    channel, _, _ = audio_channel
    channel.auto_consume = False
    assert channel.auto_consume is False
    channel.auto_consume = True
    assert channel.auto_consume is True

@pytest.mark.asyncio
async def test_drop_current_audio(audio_channel):
    channel, _, _ = audio_channel
    channel.current_audio.play()
    await asyncio.sleep(0.1)
    channel.drop_current_audio()
    assert channel.current_audio is None

@pytest.mark.asyncio
async def test_current_audio(audio_channel):
    channel, _, _ = audio_channel
    assert channel.current_audio is not None

@pytest.mark.asyncio
async def test_autoplay_second_song():
    channel = rpaudio.AudioChannel()
    channel.auto_consume = True
    

    mock_callback_1 = MagicMock()
    mock_callback_2 = MagicMock()

    audio_1 = rpaudio.AudioSink(callback=mock_callback_1)
    audio_1.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\ex.wav")
    
    channel.push(audio_1)

    audio_2 = rpaudio.AudioSink(callback=mock_callback_2)
    audio_2.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\Acrylic.mp3")
    channel.push(audio_2)


    mock_callback_3 = MagicMock()
    mock_callback_4 = MagicMock()

    audio_3 = rpaudio.AudioSink(callback=mock_callback_3)
    audio_3.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\Acrylic.mp3")
    channel.push(audio_3)

    audio_4 = rpaudio.AudioSink(callback=mock_callback_4)
    audio_4.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\Acrylic.mp3")
    channel.push(audio_4)

    await asyncio.sleep(0.5)

    assert channel.current_audio is not None
    contents = len(channel.queue_contents)
    while channel.queue_contents:
        await asyncio.sleep(0.3)
        channel.current_audio.stop()
        contents -= 1
        assert contents == len(channel.queue_contents) - 1
    await asyncio.sleep(0.3)


    mock_callback_1.assert_called_once()
    mock_callback_2.assert_called_once()
    mock_callback_3.assert_called_once()
    mock_callback_4.assert_called_once()

