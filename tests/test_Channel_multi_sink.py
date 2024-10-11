import asyncio
from unittest.mock import MagicMock
import pytest
import rpaudio as rpaudio
from unittest.mock import Mock
from rpaudio.effects import FadeIn, FadeOut, ChangeSpeed


@pytest.fixture
def audio_callback():
    return Mock()


@pytest.fixture
def audio_handler():
    mock_callback = MagicMock()

    handler = rpaudio.AudioSink(callback=mock_callback)
    handler.load_audio(r"tests/test_audio_files/test_md_wav.wav", force=True)

    return handler, mock_callback


@pytest.fixture
def audio_channel():
    """Fixture to create an AudioChannel with an AudioSink."""
    mock_callback_1 = MagicMock()
    mock_callback_2 = MagicMock()

    audio_1 = rpaudio.AudioSink(callback=mock_callback_1)
    audio_1.load_audio(r"tests/test_audio_files/test_md_wav.wav", force=True)

    channel = rpaudio.AudioChannel()
    channel.auto_consume = True
    channel.push(audio_1)

    return channel, mock_callback_1, mock_callback_2


def test_audio_channel_effects_chain(audio_callback):
    """Test if effects are applied correctly to the audio channel."""
    audio_2 = rpaudio.AudioSink(callback=audio_callback)
    audio_2.load_audio(r"tests/test_audio_files/test_md_wav.wav", force=True)

    channel_1 = rpaudio.AudioChannel()

    fade_in_effect = FadeIn(start_val=0.0, end_val=1.0, duration=3.0)
    fade_out_effect = FadeOut(end_val=0.0, duration=10.0)
    speed_up_effect = ChangeSpeed(end_val=1.5, duration=5.0)

    effects = [fade_in_effect, fade_out_effect, speed_up_effect]
    channel_1.set_effects_chain(effects)

    actual_effects = channel_1.effects
    for effect, actual_effect in zip(effects, actual_effects):
        if actual_effect.__class__.__name__ == 'FadeIn':
            assert isinstance(effect, FadeIn)
            assert effect.start_val == actual_effect.start_val
            assert effect.end_val == actual_effect.end_val
            assert effect.duration == actual_effect.duration

        elif actual_effect.__class__.__name__ == 'FadeOut':
            assert isinstance(effect, FadeOut)
            assert effect.start_val == actual_effect.start_val
            assert effect.end_val == actual_effect.end_val
            assert effect.duration == actual_effect.duration

        elif actual_effect.__class__.__name__ == 'ChangeSpeed':
            assert isinstance(effect, ChangeSpeed)
            assert effect.start_val == actual_effect.start_val
            assert effect.end_val == actual_effect.end_val
            assert effect.duration == actual_effect.duration

    for i, effect in enumerate(effects):
        actual_effect = actual_effects[i]
        assert isinstance(actual_effect, type(effect))
        assert actual_effect.start_val == effect.start_val
        assert actual_effect.end_val == effect.end_val
        assert actual_effect.duration == effect.duration


@pytest.mark.asyncio
async def test_audio_channel_auto_consume(audio_callback):
    """Test if the AudioChannel can auto-consume and switch between audio sinks."""
    # Initialize AudioSinks
    audio_1 = rpaudio.AudioSink(callback=audio_callback)
    audio_1.load_audio(r"tests/test_audio_files/test_md_wav.wav", force=True)
    audio_2 = rpaudio.AudioSink(callback=audio_callback)
    audio_2.load_audio(r"tests/test_audio_files/test_md_wav.wav", force=True)
    channel_1 = rpaudio.AudioChannel()
    channel_1.push(audio_1)
    channel_1.push(audio_2)
    print(channel_1.queue_contents)
    assert len(channel_1.queue_contents) == 2
    channel_1.auto_consume = True
    await asyncio.sleep(0.2)
    print(channel_1.queue_contents)
    assert len(channel_1.queue_contents) == 1
    channel_1.current_audio.stop()
    await asyncio.sleep(0.2)
    audio_callback.assert_called_once()
    await asyncio.sleep(0.2)
    assert len(channel_1.queue_contents) == 0


@pytest.mark.asyncio
async def test_audiochannel_multiple_sink_pushes(audio_channel):
    channel, mock_callback_1, mock_callback_2 = audio_channel
    audio_2 = rpaudio.AudioSink(callback=mock_callback_2)
    audio_2.load_audio(r"tests/test_audio_files/test_md_wav.wav", force=True)

    channel.push(audio_2)
    await asyncio.sleep(0.2)

    assert len(channel.queue_contents) == 1
    channel.drop_current_audio()
    await asyncio.sleep(0.2)
    assert channel.current_audio.metadata.title == audio_2.metadata.title


@pytest.mark.asyncio
async def test_drop_current_audio(audio_channel):
    """Test dropping the current audio."""
    channel, _, _ = audio_channel
    await asyncio.sleep(0.2)
    channel.current_audio.play()
    await asyncio.sleep(0.2)
    channel.drop_current_audio()
    assert channel.current_audio is None


@pytest.mark.asyncio
async def test_current_audio(audio_channel):
    """Test that current audio is not None."""
    channel, _, _ = audio_channel
    assert channel.current_audio is not None

@pytest.mark.asyncio
async def test_autoplay_songs():
    """Test automatic playback of multiple songs in AudioChannel."""
    channel = rpaudio.AudioChannel()
    channel.auto_consume = True

    mock_callback_1 = MagicMock()
    mock_callback_2 = MagicMock()
    mock_callback_3 = MagicMock()
    mock_callback_4 = MagicMock()

    audio_1 = rpaudio.AudioSink(callback=mock_callback_1)
    audio_1.load_audio(r"tests/test_audio_files/test_md_mp3.mp3", force=True)
    channel.push(audio_1)

    audio_2 = rpaudio.AudioSink(callback=mock_callback_2)
    audio_2.load_audio(r"tests/test_audio_files/test_md_mp3.mp3", force=True)
    channel.push(audio_2)

    audio_3 = rpaudio.AudioSink(callback=mock_callback_3)
    audio_3.load_audio(r"tests/test_audio_files/test_md_mp3.mp3", force=True)
    channel.push(audio_3)

    audio_4 = rpaudio.AudioSink(callback=mock_callback_4)
    audio_4.load_audio(r"tests/test_audio_files/test_md_mp3.mp3", force=True)
    channel.push(audio_4)

    await asyncio.sleep(0.1)  

    initial_contents = len(channel.queue_contents)

    while channel.queue_contents:
        await asyncio.sleep(0.1)

        if channel.current_audio:
            assert channel.current_audio is not None

            channel.current_audio.stop() 
            await asyncio.sleep(0.1)  # 

        current_contents = len(channel.queue_contents)
        assert current_contents >= 0, "Queue should not be negative."
        
        assert initial_contents - (len(channel.queue_contents) + 1) <= initial_contents, f"Expected {initial_contents}, but got {len(channel.queue_contents)}"
        
        initial_contents -= 1 

    await asyncio.sleep(0.1)  

    mock_callback_1.assert_called_once()
    mock_callback_2.assert_called_once()
    mock_callback_3.assert_called_once()
    mock_callback_4.assert_called_once()
