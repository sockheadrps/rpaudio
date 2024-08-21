from typing import Dict, Optional, Callable, Protocol, Union


class AudioSink(Protocol):
    """
    Interface that wraps functionality for audio files.

    This class provides methods to load, play, pause, and stop audio playback.
    An optional callback function can be invoked when the audio stops playing.

    :param callback: (optional) A function that will be called when the audio stops playing.
    :type callback: Optional[Callable[[], None]]
    """
    def __init__(self, callback: Optional[Callable[[], None]] = None) -> None:
        """
        Constructor method.

        Initializes an instance of AudioHandler with an optional callback function.

        :param callback: A function that will be called when the audio stops playing.
        :type callback: Optional[Callable[[], None]]
        """
        ...

    @property
    def is_playing(self) -> bool:
        """
        Flag indicating whether the audio is currently playing.
        
        :return: True if the audio is playing, False otherwise.
        :rtype: bool
        """
        ...

    def load_audio(self, filename: str) -> None:
        """Load an audio file for playback.
        
        :param filename: The path to the audio file to load.
        :type filename: str
        """

        ...

    def play(self) -> None:
        """
        Start playing the loaded audio.

        This method begins playback of the audio that was loaded using the `load_audio` method.
        If the audio is already playing, this method has no effect.

        Raises:
            RuntimeError: If no audio has been loaded
        """
        ...

    def pause(self) -> None:
        """
        Pause the currently playing audio, if any.
        
        Raises:
            RuntimeError: If no audio has been loaded
        """
        ...

    def stop(self) -> None:
        """
        Stop the currently playing audio, if any.
        
        Raises:
            RuntimeError: If no audio has been loaded
        """
        ...

    @property
    def get_effects(self, effect: dict[str, any]) -> dict[str, any]:
        """
        Get current effect settings.
        rtype: dict[str, any]
        """
        ...
    
    
    @property.setter
    def set_effects(self, effect: dict[str, any]) -> None:
        """
        Apply an effect to the audio.

        :param effect: A dictionary containing the effect settings.
        :type effect: dict[str, any]
        """
        ...

    @property
    def metadata(self) -> Dict[str, str]:
        """
        Get metadata for the audio file.

        :return: A dictionary containing metadata for the audio file.
        :rtype: dict[str, any]
        """
        ...




class MetaData:
    """
    A class representing metadata for an audio file.
    """
    def __init__(self, audio_sink: 'AudioSink') -> None:
        """
        Initializes an instance of MetaData with values from the audio_sink.
        
        :param audio_sink: The source of metadata for the audio file.
        :type audio_sink: AudioSink
        """
        self.title = audio_sink.get_metadata("title")
        self.artist = audio_sink.get_metadata("artist")
        self.date = audio_sink.get_metadata("date")
        self.year = audio_sink.get_metadata("year")
        self.album_title = audio_sink.get_metadata("album_title")
        self.album_artist = audio_sink.get_metadata("album_artist")
        self.track_number = audio_sink.get_metadata("track_number")
        self.total_tracks = audio_sink.get_metadata("total_tracks")
        self.disc_number = audio_sink.get_metadata("disc_number")
        self.total_discs = audio_sink.get_metadata("total_discs")
        self.genre = audio_sink.get_metadata("genre")
        self.composer = audio_sink.get_metadata("composer")
        self.comment = audio_sink.get_metadata("comment")
        self.sample_rate = audio_sink.get_metadata("sample_rate")
        self.channels = audio_sink.get_metadata("channels")
        self.duration = audio_sink.get_metadata("duration")


    @property
    def title(self) -> Optional[str]:
        """
        Get the title of the audio file.

        :return: The title of the audio file, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def artist(self) -> Optional[str]:
        """
        Get the artist of the audio file.

        :return: The artist of the audio file, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def date(self) -> Optional[str]:
        """
        Get the date associated with the audio file.

        :return: The date of the audio file, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def year(self) -> Optional[str]:
        """
        Get the year the audio file was released.

        :return: The year of the audio file, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def album_title(self) -> Optional[str]:
        """
        Get the album title of the audio file.

        :return: The album title of the audio file, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def album_artist(self) -> Optional[str]:
        """
        Get the album artist of the audio file.

        :return: The album artist of the audio file, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def track_number(self) -> Optional[str]:
        """
        Get the track number of the audio file.

        :return: The track number of the audio file, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def total_tracks(self) -> Optional[str]:
        """
        Get the total number of tracks in the album.

        :return: The total number of tracks, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def disc_number(self) -> Optional[str]:
        """
        Get the disc number of the audio file.

        :return: The disc number, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def total_discs(self) -> Optional[str]:
        """
        Get the total number of discs in the album.

        :return: The total number of discs, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def genre(self) -> Optional[str]:
        """
        Get the genre of the audio file.

        :return: The genre of the audio file, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def composer(self) -> Optional[str]:
        """
        Get the composer of the audio file.

        :return: The composer of the audio file, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def comment(self) -> Optional[str]:
        """
        Get the comment associated with the audio file.

        :return: The comment of the audio file, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def sample_rate(self) -> Optional[int]:
        """
        Get the sample rate of the audio file.

        :return: The sample rate of the audio file, or None if not available.
        :rtype: Optional[int]
        """
        ...

    @property
    def channels(self) -> Optional[str]:
        """
        Get the number of channels in the audio file.

        :return: The number of channels, or None if not available.
        :rtype: Optional[str]
        """
        ...

    @property
    def duration(self) -> Optional[float]:
        """
        Get the duration of the audio file in seconds.

        :return: The duration of the audio file, or None if not available.
        :rtype: Optional[float]
        """
        ...

class AudioChannel(Protocol):
    """
    Manages a queue of AudioSink objects and handles playback.

    :param channel_id: A unique identifier for the audio channel.
    :type channel_id: Union[int, str]
    :param channel_callback: (optional) A callback invoked when the queue is idle.
    :type channel_callback: Optional[Callable[[], None]]
    """

    def __init__(self, channel_id: Union[int, str], channel_callback: Optional[Callable[[], None]]) -> None:
        ...

    def push(self, audio: AudioSink) -> None:
        """
        Adds an AudioSink to the channel queue.
        
        :param audio: The audio object to add to the queue.
        :type audio: AudioSink
        """
        ...

    @property
    def auto_consume(self) -> bool:
        """
        Returns whether the channel automatically consumes the queue.
        
        :rtype: bool
        """
        ...

    @auto_consume.setter
    def auto_consume(self, value: bool) -> None:
        """
        Sets the auto-consume behavior of the channel.
        
        :param value: True to enable auto-consume, False to disable.
        :type value: bool
        """
        ...

    def drop_current_audio(self) -> None:
        """
        Drops the current audio from the queue.
        """
        ...

    @property
    def current_audio(self) -> AudioSink:
        """
        Returns the currently playing AudioSink object.
        
        :rtype: AudioSink
        """
        ...

    async def control_loop(self) -> None:
        """
        Continuously monitors the queue and handles playback, 
        auto-consume, and callback execution.
        """
        ...

class ChannelManager(Protocol):
    """
    Manages multiple audio channels and provides an API to control them.

    :ivar channels: A dictionary mapping channel identifiers to their corresponding AudioChannel instances.
    :vartype channels: dict
    """

    channels: dict[str, AudioChannel]

    def __init__(self) -> None:
        """
        Initializes a new instance of ChannelManager.
        """
        ...

    def add_channel(self, name: str, channel: AudioChannel) -> None:
        """
        Adds a new audio channel to the manager.

        :param name: The unique identifier for the channel.
        :type name: str
        :param channel: The audio channel to add.
        :type channel: AudioChannel
        """
        ...

    def drop_channel(self, name: str) -> None:
        """
        Drops an audio channel from the manager.

        :param name: The unique identifier of the channel to drop.
        :type name: str
        :raises RuntimeError: If the channel is not found.
        """
        ...

    def channel(self, name: str) -> Optional[AudioChannel]:
        """
        Retrieves a channel by its identifier.

        :param name: The unique identifier of the channel.
        :type name: str
        :return: The corresponding AudioChannel instance, or None if not found.
        :rtype: Optional[AudioChannel]
        """
        ...

    def start_all(self) -> None:
        """
        Starts auto-consuming audio on all channels.
        """
        ...

    def stop_all(self) -> None:
        """
        Stops auto-consuming audio on all channels.
        """
        ...
