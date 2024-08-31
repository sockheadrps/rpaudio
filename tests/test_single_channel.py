import pytest
import asyncio
import rpaudio
from datetime import datetime, timedelta

@pytest.fixture
def audio_channel():
    def on_audio_stop():
        print("Audio has stopped")

    audio_1 = rpaudio.AudioSink(callback=on_audio_stop)
    audio_1.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\ex.wav")
    audio_2 = rpaudio.AudioSink(callback=on_audio_stop)
    audio_2.load_audio(r"C:\Users\16145\Desktop\code_24\rpaudio\examples\Acrylic.mp3")

    channel_1 = rpaudio.AudioChannel()
    channel_1.auto_consume = True
    channel_1.push(audio_1)
    channel_1.push(audio_2)

    return channel_1

@pytest.mark.asyncio
async def test_play_audio(audio_channel):
    audio_channel.current_audio.play()
    await asyncio.sleep(0.1)
    assert audio_channel.current_audio.is_playing is True

@pytest.mark.asyncio
async def test_pause(audio_channel):
    audio_channel.current_audio.play()
    await asyncio.sleep(0.1)
    audio_channel.current_audio.pause()
    await asyncio.sleep(0.1)
    assert audio_channel.current_audio.is_playing is False

@pytest.mark.asyncio
async def test_resume(audio_channel):
    audio_channel.current_audio.play()
    await asyncio.sleep(0.1)
    audio_channel.current_audio.pause()
    await asyncio.sleep(0.1)
    audio_channel.current_audio.play()
    await asyncio.sleep(0.1)
    assert audio_channel.current_audio.is_playing is True

@pytest.mark.asyncio
async def test_set_volume(audio_channel):
    audio_channel.current_audio.play()
    await asyncio.sleep(0.1)
    audio_channel.current_audio.set_volume(0.5)
    await asyncio.sleep(0.1)
    assert audio_channel.current_audio.get_volume() == 0.5

@pytest.mark.asyncio
async def test_try_seek(audio_channel):
    audio_channel.current_audio.play()
    await asyncio.sleep(0.1)
    audio_channel.current_audio.try_seek(2)
    await asyncio.sleep(0.1)
    assert audio_channel.current_audio.get_pos() >= 2

@pytest.mark.asyncio
async def test_get_pos(audio_channel):
    audio_channel.current_audio.play()
    await asyncio.sleep(0.1)
    pos = audio_channel.current_audio.get_pos()
    assert pos >= 0

@pytest.mark.asyncio
async def test_set_speed(audio_channel):
    audio_channel.current_audio.play()
    await asyncio.sleep(0.1)
    audio_channel.current_audio.set_speed(1.5)
    await asyncio.sleep(0.1)
    assert audio_channel.current_audio.get_speed() == 1.5

@pytest.mark.asyncio
async def test_get_speed(audio_channel):
    audio_channel.current_audio.play()
    await asyncio.sleep(0.1)
    speed = audio_channel.current_audio.get_speed()
    assert speed >= 1.0

@pytest.mark.asyncio
async def test_stop(audio_channel):
    audio_channel.current_audio.play()
    await asyncio.sleep(0.1)
    audio_channel.current_audio.stop()
    await asyncio.sleep(0.1)
    if audio_channel.current_audio is None:
      audio_channel.current_audio.stop()
      assert audio_channel.current_audio.is_playing is False
