import pytest
import asyncio
from unittest.mock import MagicMock
from rpaudio.effects import FadeIn, FadeOut, ChangeSpeed
import rpaudio
import rpaudio.exceptions


@pytest.fixture
def audio_handler():
    mock_callback = MagicMock()

    handler = rpaudio.AudioSink(callback=mock_callback)
    handler.load_audio(r"examples/ex.wav", force=True)

    return handler, mock_callback


@pytest.mark.asyncio
async def test_fade_in(audio_handler):
    handler, _ = audio_handler
    fade_in_effect = FadeIn(
        apply_after=handler.get_pos(), start_val=0.0, end_val=1.0, duration=1.0)
    handler.apply_effects([fade_in_effect])
    handler.play()
    initial_volume = handler.get_volume()
    await asyncio.sleep(1)
    new_volume = handler.get_volume()
    assert new_volume > initial_volume
    handler.stop()


@pytest.mark.asyncio
async def test_fade_out(audio_handler):
    handler, _ = audio_handler
    fade_out_effect = FadeOut(
        duration=1.0, apply_after=handler.get_pos())
    handler.apply_effects([fade_out_effect])
    handler.play()
    handler.set_volume(1.0)
    initial_volume = handler.get_volume()
    await asyncio.sleep(1)
    new_volume = handler.get_volume()
    assert new_volume < initial_volume
    handler.stop()


@pytest.mark.asyncio
async def test_change_speed(audio_handler):
    handler, _ = audio_handler
    change_speed_effect = ChangeSpeed(end_val=1.5)
    handler.apply_effects([change_speed_effect])
    handler.play()
    await asyncio.sleep(0.5)
    new_speed = handler.get_speed()
    assert new_speed == 1.5
    handler.stop()


@pytest.mark.asyncio
async def test_effects_applied_over_time(audio_handler):
    handler, _ = audio_handler
    fade_in_effect = FadeIn(
        start_val=0.0, end_val=1.0, duration=1.0)
    handler.apply_effects([fade_in_effect])
    handler.play()
    await asyncio.sleep(1)
    halfway_volume = handler.get_volume()
    assert halfway_volume >= 0.5 and halfway_volume <= (
        1.0)
    handler.stop()


@pytest.mark.asyncio
async def test_effects_applied_over_time_vol_exception(audio_handler):
    handler, _ = audio_handler
    fade_in_effect = FadeIn(start_val=0.0, end_val=1.0, duration=1.0)
    handler.apply_effects([fade_in_effect])
    handler.play()
    await asyncio.sleep(0.1)

    with pytest.raises(rpaudio.exceptions.EffectConflictException) as exc_info:
        handler.set_volume(0.0)

    handler.stop()


@pytest.mark.asyncio
async def test_effects_applied_over_time_speed_exception(audio_handler):
    handler, _ = audio_handler
    speed_effect = ChangeSpeed(end_val=1.5, duration=1.0)
    handler.apply_effects([speed_effect])
    handler.play()
    await asyncio.sleep(0.1)

    with pytest.raises(rpaudio.exceptions.EffectConflictException) as exc_info:
        handler.set_speed(0.9)

    handler.stop()

@pytest.mark.asyncio
async def test_effect_completion(audio_handler):
    handler, _ = audio_handler
    fade_out_effect = FadeOut(apply_after=handler.get_pos(),
                                      duration=1.0)
    handler.apply_effects([fade_out_effect])
    handler.play()
    await asyncio.sleep(1.2)
    final_volume = handler.get_volume()
    assert final_volume == 0
    change_speed_effect = ChangeSpeed(end_val=1.0, duration=0.1)
    handler.apply_effects([change_speed_effect])
    await asyncio.sleep(1)
    final_speed = handler.get_speed()
    assert final_speed == 1.0
    handler.stop()


@pytest.mark.asyncio
async def test_set_duration(audio_handler):
    """Test that set_duration properly updates metadata."""
    handler, _ = audio_handler
    new_duration = 120.0
    handler.set_duration(new_duration)
    assert handler.metadata['duration'] == str(new_duration)


def test_set_volume_valid_input(audio_handler):
    handler, _ = audio_handler
    for volume in [0.0, 0.5, 1.0]:
        handler.set_volume(volume)
        assert handler.get_volume() == volume


def test_set_volume_out_of_range(audio_handler):
    handler, _ = audio_handler

    with pytest.raises(ValueError, match="Volume must be between 0.0 and 1.0."):
        handler.set_volume(-0.1)
    with pytest.raises(ValueError, match="Volume must be between 0.0 and 1.0."):
        handler.set_volume(1.1)


def test_volume_uninitialized_sink():
    uninitialized_sink = rpaudio.AudioSink()
    with pytest.raises(RuntimeError, match="No sink available to set volume. Load audio first."):
        uninitialized_sink.set_volume(0.5)

    with pytest.raises(RuntimeError, match="No sink available. Load audio first."):
        uninitialized_sink.get_volume()


def test_get_volume_returns_correct_value(audio_handler):
    handler, _ = audio_handler
    handler.set_volume(0.75)
    assert handler.get_volume() == 0.75


@pytest.mark.asyncio
async def test_cancel_callback():
    """Test that the callback is not called after cancellation."""
    mock_callback = MagicMock()
    handler = rpaudio.AudioSink(callback=mock_callback)
    handler.load_audio(r"examples/ex.wav")
    handler.play()
    mock_callback.assert_not_called()
    await asyncio.sleep(0.1)
    handler.cancel_callback()
    await asyncio.sleep(0.1)
    mock_callback.assert_not_called()
    handler.stop()
    await asyncio.sleep(0.1)
    mock_callback.assert_not_called()


@pytest.mark.asyncio
async def test_stop_uninitialized_sink():
    """Test stop error handling with uninitialized sink."""
    handler = rpaudio.AudioSink()

    with pytest.raises(RuntimeError, match="No sink available to stop. Load audio first."):
        handler.stop()


@pytest.mark.asyncio
async def test_pause_uninitialized_sink():
    """Test pause error handling with uninitialized sink."""
    handler = rpaudio.AudioSink()

    with pytest.raises(RuntimeError, match="No sink available. Load audio first."):
        handler.pause()


@pytest.mark.asyncio
async def test_no_file_provided():
    with pytest.raises(TypeError, match="missing 1 required positional argument: 'file_path'"):
        rpaudio.AudioSink().load_audio()


@pytest.mark.asyncio
async def test_audio_handler_no_callback():
    handler = rpaudio.AudioSink()
    handler.load_audio(r"examples/ex.wav")
    assert handler.callback is None


@pytest.mark.asyncio
async def test_load_audio_multiple_times(audio_handler):
    handler, _ = audio_handler
    with pytest.raises(RuntimeError, match="Audio is already loaded. Please stop the current audio before loading a new one."):
        handler.load_audio(r"examples/ex2.wav")


@pytest.mark.asyncio
async def test_audio_handler_callback():
    mock_callback = MagicMock()
    handler = rpaudio.AudioSink(callback=mock_callback)
    handler.load_audio(r"examples/ex.wav")
    assert handler.callback is not None


@pytest.mark.asyncio
async def test_default_values(audio_handler):
    handler, _ = audio_handler
    assert handler.is_playing is False
    assert handler.get_volume() == 0.0
    assert handler.get_speed() == 1.0


@pytest.mark.asyncio
async def test_play_audio(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    assert handler.is_playing is True
    handler.stop()


@pytest.mark.asyncio
async def test_pause(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.pause()
    await asyncio.sleep(0.1)
    assert handler.is_playing is False
    handler.stop()


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
    handler.stop()


@pytest.mark.asyncio
async def test_set_volume(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.set_volume(0.5)
    await asyncio.sleep(0.1)
    assert handler.get_volume() == 0.5
    handler.stop()


@pytest.mark.asyncio
async def test_try_seek(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.try_seek(4)
    await asyncio.sleep(0.1)
    assert handler.get_pos() >= 4
    handler.stop()


@pytest.mark.asyncio
async def test_get_pos(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    pos = handler.get_pos()
    assert pos >= 0
    handler.stop()


@pytest.mark.asyncio
async def test_set_speed(audio_handler):
    handler, _ = audio_handler
    handler.set_speed(1.5)
    handler.play()
    await asyncio.sleep(0.1)
    speed_up = ChangeSpeed(end_val=1.5)
    effects_list = [speed_up]
    handler.apply_effects(effects_list)
    await asyncio.sleep(0.1)
    current_speed = handler.get_speed()
    assert current_speed == 1.5
    handler.stop()


@pytest.mark.asyncio
async def test_get_speed(audio_handler):
    handler, _ = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    speed = handler.get_speed()
    assert speed >= 1.0
    handler.stop()


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
    handler.stop()


@pytest.mark.asyncio
async def test_callback_called(audio_handler):
    handler, mock_callback = audio_handler
    handler.play()
    await asyncio.sleep(0.1)
    handler.stop()
    await asyncio.sleep(0.1)
    mock_callback.assert_called_once()
