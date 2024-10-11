import asyncio
from unittest.mock import MagicMock
import pytest
from rpaudio import ChannelManager, AudioChannel, AudioSink


@pytest.fixture
def setup_channel_manager():
    """Fixture to set up a ChannelManager with two AudioChannels."""
    manager = ChannelManager()

    channel_1 = AudioChannel()
    channel_2 = AudioChannel()

    manager.add_channel("Channel1", channel_1)
    manager.add_channel("Channel2", channel_2)

    return manager, channel_1, channel_2


def test_add_channel(setup_channel_manager):
    """Test adding a channel to ChannelManager."""
    manager, _, _ = setup_channel_manager

    new_channel = AudioChannel()
    mock_callback = MagicMock()
    audio = AudioSink(callback=mock_callback)
    audio.load_audio(r"tests\test_audio_files\test_md_mp3.mp3", force=True)
    new_channel.push(audio)

    manager.add_channel("Channel3", new_channel)

    retrieved_channel = manager.channel("Channel3")
    assert retrieved_channel.queue_contents[0].metadata.as_dict() == new_channel.queue_contents[0].metadata.as_dict()


def test_channel_retrieval(setup_channel_manager):
    """Test retrieving a channel by its identifier."""
    manager, _, _ = setup_channel_manager
    mock_callback = MagicMock()
    audio = AudioSink(callback=mock_callback)
    audio.load_audio(r"tests\test_audio_files\test_md_mp3.mp3")
    channel_1 = manager.channel("Channel1")
    channel_1.push(audio)

    retrieved_channel = manager.channel("Channel1")
    assert retrieved_channel.queue_contents[0].metadata.as_dict() == channel_1.queue_contents[0].metadata.as_dict()


def test_channel_retrieval_not_found(setup_channel_manager):
    """Test retrieving a channel that does not exist."""
    manager, _, _ = setup_channel_manager

    assert manager.channel("NonExistentChannel") is None


@pytest.mark.asyncio
async def test_start_all(setup_channel_manager):
    """Test starting auto-consume on all channels."""
    manager, channel_1, channel_2 = setup_channel_manager

    audio_1 = AudioSink()
    audio_1.load_audio(r"tests\test_audio_files\test_md_wav.wav", force=True)
    audio_2 = AudioSink()
    audio_2.load_audio(r"tests\test_audio_files\test_md_wav.wav", force=True)

    channel_1.push(audio_1)
    channel_2.push(audio_2)
    channel_1.auto_consume = True
    channel_2.auto_consume = True

    manager.start_all()
    await asyncio.sleep(0.2)

    assert channel_1.current_audio.is_playing is True
    assert channel_2.current_audio.is_playing is True


@pytest.mark.asyncio
async def test_stop_all(setup_channel_manager):
    """Test stopping auto-consume on all channels."""
    manager, channel_1, channel_2 = setup_channel_manager

    audio_1 = AudioSink()
    audio_1.load_audio(r"tests\test_audio_files\test_md_mp3.mp3", force=True)
    audio_2 = AudioSink()
    audio_2.load_audio(r"tests\test_audio_files\test_md_mp3.mp3", force=True)

    channel_1.push(audio_1)
    channel_2.push(audio_2)
    channel_1.auto_consume = True
    channel_2.auto_consume = True

    manager.stop_all()
    await asyncio.sleep(1)

    assert channel_1.current_audio is None
    assert channel_2.current_audio is None


def test_drop_channel(setup_channel_manager):
    """Test dropping a channel from ChannelManager."""
    manager, _, _ = setup_channel_manager

    assert manager.channel("Channel1") is not None
    manager.drop_channel("Channel1")
    assert manager.channel("Channel1") is None


def test_drop_channel_not_found(setup_channel_manager):
    """Test dropping a channel that does not exist."""
    manager, _, _ = setup_channel_manager

    with pytest.raises(RuntimeError, match="Channel not found"):
        manager.drop_channel("NonExistentChannel")
