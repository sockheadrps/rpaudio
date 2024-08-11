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
    def effects(self, effect: dict[str, any]) -> dict[str, any]:
        """
        Get current effect settings.
        rtype: dict[str, any]
        """
        ...
    
    @property.setter
    def effects(self, effect: dict[str, any]) -> None:
        """
        Apply an effect to the audio.

        :param effect: A dictionary containing the effect settings.
        :type effect: dict[str, any]
        """
        ...

    @property
    def metadata(self) -> dict[str, any]:
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
    Manages a queue of AudioSink objects and provides an interface to control them.

    :param channel_id: The unique identifier for the audio channel.
    :type channel_id: Union[int, str]
    :param channel_callback: An optional callback function to be invoked when the queue becomes idle.
    :type channel_callback: Optional[Callable[[], None]]
    """

    def __init__(self, channel_id: Union[int, str], channel_callback: Optional[Callable[[], None]]) -> None:
        raise NotImplementedError

    def add_audio(self, audio: AudioSink) -> None:
        """
        Adds an audio object to the channel queue.

        :param audio: The audio object to add to the queue.
        :type audio: AudioSink
        """
        raise NotImplementedError

    @property
    def auto_consume(self) -> bool:
        """
        Gets the auto-consume property.

        :return: True if the channel automatically consumes the queue, False otherwise.
        :rtype: bool
        """
        raise NotImplementedError

    @auto_consume.setter
    def auto_consume(self, value: bool) -> None:
        """
        Sets the auto-consume property.

        :param value: If True, the channel will automatically consume the queue.
        :type value: bool
        """
        raise NotImplementedError

    def drop_current_audio(self) -> None:
        """
        Drops the current audio object from the queue.
        """
        raise NotImplementedError

    @property
    def current_audio(self) -> AudioSink:
        """
        Returns the current audio object in the queue.

        :return: The current audio object in the queue.
        :rtype: AudioSink
        """
        raise NotImplementedError

    async def control_loop(self) -> None:
        """
        Listens for audio events to pass to the sink, plays the audio objects in the queue, applies channel callback if queue is exhausted,
        applies sink settings, and auto-consumes the queue if enabled.
        """
        raise NotImplementedError
