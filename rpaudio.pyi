from typing import Optional, Callable, Protocol, Union


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
    def metadata(self) -> dict[str, str]:
        """
        Get metadata for the audio file.

        :return: A dictionary containing metadata for the audio file.
        :rtype: dict[str, any]
        """
        ...


class AudioManager(Protocol):
    """
    Manages multiple audio channels and provides an API to control them.

    :ivar channels: A dictionary mapping channel identifiers to their corresponding AudioChannel instances.
    :vartype channels: dict
    """
    def __init__(self) -> None:
        """
        Initializes an instance of AudioManager.
        """
        raise NotImplementedError

    def add_channel(self, channel: AudioChannel) -> None:
        """
        Adds a new audio channel to the manager.

        :param channel: The audio channel to add.
        :type channel: AudioChannel
        """
        raise NotImplementedError

    def drop_channel(self, channel_id: Union[int, str]) -> None:
        """
        Drops an audio channel from the manager.

        :param channel_id: The unique identifier of the channel to drop.
        :type channel_id: Union[int, str]
        """
        raise NotImplementedError

    def apply_effect(self, channel_id: Union[int, str], effect: dict[str, any]) -> None:
        """
        Stores effect settings for a channel to apply to each AudioSink object in the channel queue. 

        :param channel_id: The unique identifier of the channel to apply the effect to.
        :type channel_id: Union[int, str]
        :param effect: The effect to apply to the channel.
        :type effect: dict[str, any]
        """
        raise NotImplementedError

    async def channel_loop(self) -> None:
        """
        Asynchronously listens for audio events and sends them to the appropriate channel.
        """
        raise NotImplementedError

class AudioChannel(Protocol):
    """
    Manages a collection of audio sinks and controls playback.

    This protocol defines the methods and properties required for an audio channel,
    including adding, removing, and controlling audio sinks, as well as automatic
    consumption of sinks.

    :ivar queue_contents: A list of audio sinks in the channel's queue.
    :vartype queue_contents: list[AudioSink]
    :ivar auto_consume: Flag indicating whether the channel should automatically consume sinks.
    :vartype auto_consume: bool
    :ivar current_audio: The currently playing audio sink.
    :vartype current_audio: Optional[AudioSink]
    """

    def push(self, sink: AudioSink) -> None:
        """
        Adds a new audio sink to the channel's queue.

        :param sink: The audio sink to add.
        :type sink: AudioSink
        """
        ...

    def pop(self) -> Optional[AudioSink]:
        """
        Removes and returns the next audio sink from the channel's queue.

        :return: The next audio sink from the queue or None if the queue is empty.
        :rtype: Optional[AudioSink]
        """
        ...

    def consume(self) -> None:
        """
        Consumes the next audio sink from the queue and starts playback.

        This method should be used to manually start playback of the next audio sink.
        """
        ...

    @property
    def is_playing(self) -> bool:
        """
        Checks if there is currently audio playing in the channel.

        :return: True if audio is currently playing, False otherwise.
        :rtype: bool
        """
        ...

    @property
    def queue_contents(self) -> List[AudioSink]:
        """
        Gets the current contents of the audio queue.

        :return: A list of audio sinks currently in the queue.
        :rtype: list[AudioSink]
        """
        ...

    @property
    def current_audio(self) -> Optional[AudioSink]:
        """
        Gets the currently playing audio sink.

        :return: The currently playing audio sink or None if no audio is playing.
        :rtype: Optional[AudioSink]
        """
        ...

    @property
    def auto_consume(self) -> bool:
        """
        Gets the auto-consume flag status.

        :return: True if auto-consume is enabled, False otherwise.
        :rtype: bool
        """
        ...

    @auto_consume.setter
    def auto_consume(self, value: bool) -> None:
        """
        Sets the auto-consume flag status.

        :param value: True to enable auto-consume, False to disable it.
        :type value: bool
        """
        ...

    def channel_loop(self) -> None:
        """
        Starts the channel loop which manages playback and auto-consumption.
        
        This method runs in a separate thread and continuously checks the queue and
        manages playback based on the auto-consume flag.
        """
        ...

class MetaData:
    """
    A class representing metadata for an audio file.

    This class provides various attributes that store metadata such as title, artist, album information, and more.
    """

    def __init__(self, audio_sink) -> None:
        """
        Constructor method.

        Initializes an instance of MetaData with values from the audio_sink.

        :param audio_sink: The source of metadata for the audio file.
        :type audio_sink: Any
        """
        attribute_mapping = {
            "title": "title",
            "artist": "artist",
            "date": "date",
            "year": "year",
            "album_title": "album_title",
            "album_artist": "album_artist",
            "track_number": "track_number",
            "total_tracks": "total_tracks",
            "disc_number": "disc_number",
            "total_discs": "total_discs",
            "genre": "genre",
            "composer": "composer",
            "comment": "comment",
            "sample_rate": None,  
            "channels": None,     
            "duration": None      
        }

        for attribute, method_name in attribute_mapping.items():
            if method_name is not None:
                value = getattr(audio_sink, method_name)()  
                setattr(self, attribute, value)
            else:
                setattr(self, attribute, None)

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