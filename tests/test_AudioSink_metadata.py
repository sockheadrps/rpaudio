import pytest
import asyncio
from unittest.mock import MagicMock
import rpaudio


test_dict = {'duration': None, 'date': None, 'total_tracks': None, 'channels': None, 'album_title': None, 'total_discs': None, 'genre': None, 'disc_number': None,
             'artist': None, 'year': None, 'title': None, 'album_artist': None, 'track_number': None, 'composer': None, 'sample_rate': None, 'comment': None}


@pytest.fixture
def audio_handler_metadata_wav():
    mock_callback = MagicMock()
    handler = rpaudio.AudioSink(callback=mock_callback)
    handler.load_audio(r"tests\test_audio_files\test_md_wav.wav")
    metadata = handler.metadata
    return metadata


@pytest.fixture
def audio_handler_metadata_flac():
    mock_callback = MagicMock()
    handler = rpaudio.AudioSink(callback=mock_callback)
    handler.load_audio(r"tests\test_audio_files\test_md_flac.flac")
    metadata = handler.metadata
    return metadata


@pytest.fixture
def audio_handler_metadata_mp3():
    mock_callback = MagicMock()
    handler = rpaudio.AudioSink(callback=mock_callback)
    handler.load_audio(
        r"tests\test_audio_files\test_md_mp3.mp3")
    metadata = handler.metadata
    return metadata


def test_metadata_wav(audio_handler_metadata_wav):
    metadata = audio_handler_metadata_wav
    assert metadata['channels'] == "2"
    assert metadata['sample_rate'] == "44100"
    assert metadata['duration'] == "1.4"
    assert metadata is not None


def test_metadata_mp3(audio_handler_metadata_mp3):
    metadata = audio_handler_metadata_mp3
    assert metadata['title'] == "rpaudio"
    assert metadata['artist'] == "rpaudio"
    assert metadata['album_title'] == "rpaudio"
    assert metadata['genre'] == "rpaudio"
    assert metadata['track_number'] == '1'
    assert metadata['year'] == '2024'
    assert metadata is not None

    for key in test_dict.keys():
        assert key in metadata


# @pytest.mark.asyncio
def test_metadata_flac(audio_handler_metadata_flac):
    metadata = audio_handler_metadata_flac
    assert metadata['title'] == "rpaudio"
    assert metadata['artist'] == "rpaudio"
    assert metadata['album_title'] == "rpaudio"
    assert metadata['genre'] == "rpaudio"
    assert metadata['track_number'] == '1'
    assert metadata['year'] == '2024'
    assert metadata is not None
    for key in test_dict.keys():
        assert key in metadata
