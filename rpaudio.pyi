from typing import Dict, List, Optional, Callable, Protocol, Union


class AudioSink(Protocol):
    """
    Interface that wraps functionality for audio files.

    This class provides methods to load, play, pause, stop audio playback, manage audio effects,
    and manipulate playback speed and volume. An optional callback function can be invoked when
    the audio stops playing.

    Example:

    .. code-block:: python

        handler = AudioHandler(callback=my_callback)
        handler.load_audio("my_audio_file.mp3")
        handler.play()
        handler.pause()
        handler.stop()

    Args:
        callback (Optional[Callable[[], None]]): A function that will be called when the audio stops playing.

    Attributes:
        is_playing (bool): Flag indicating whether the audio is currently playing.
    """

    def __init__(self, callback: Optional[Callable[[], None]] = None) -> 'AudioSink':
        """
        Constructor method.

        Initializes an instance of AudioSink with an optional callback function.

        Args:
            callback (Optional[Callable[[], None]]): A function that will be called when the audio stops playing.

        Returns:
            None: This method does not return any value.

        Example:

        .. code-block:: python

            def on_audio_end():
                print("Audio has ended.")

            handler = AudioHandler(callback=on_audio_end)
        """

    @property
    def is_playing(self) -> bool:
        """
        Flag indicating whether the audio is currently playing.

        Returns:
            bool: True if the audio is playing, False otherwise.

        Example:

        .. code-block:: python

            handler = AudioHandler(callback=my_callback)
            handler.load_audio("my_audio_file.mp3")
            handler.play()
            print(handler.is_playing)  # True if audio is playing
        """

    def load_audio(self, filename: str) -> AudioSink:
        """
        Load an audio file for playback.

        :param filename: The path to the audio file to load.
        :type filename: str
        """

    def play(self) -> None:
        """
        Start playing the loaded audio.

        This method begins playback of the audio that was loaded using the `load_audio` method.
        If the audio is already playing, this method has no effect.

        Raises:
            RuntimeError: If no audio has been loaded.

        Example:

        .. code-block:: python

            handler = AudioHandler(callback=my_callback)
            handler.load_audio("my_audio_file.mp3")
            handler.play()
        """

    def pause(self) -> None:
        """
        Pause the currently playing audio, if any.

        Raises:
            RuntimeError: If no audio has been loaded.

        Example:

        .. code-block:: python

            handler = AudioHandler(callback=my_callback)
            handler.load_audio("my_audio_file.mp3")
            handler.play()
            handler.pause()
        """

    def stop(self) -> None:
        """
        Stop the currently playing audio, if any.

        Raises:
            RuntimeError: If no audio has been loaded.

        Example:

        .. code-block:: python

            handler = AudioHandler(callback=my_callback)
            handler.load_audio("my_audio_file.mp3")
            handler.play()
            handler.stop()
        """

    @property
    def metadata(self) -> dict[str, any]:
        """
        Get metadata for the audio file.

        Example:

        .. code-block:: python

            audio_1: rpaudio.AudioSink = rpaudio.AudioSink(callback=on_audio_stop)
            audio_1.load_audio("ex.wav")
            data = audio_1.metadata

        :return: A dictionary containing metadata for the audio file.
        :rtype: dict[str, any]
        """

    def get_speed(self) -> float:
        """
        Get the current playback speed of the audio.

        :return: The playback speed.
        :rtype: float
        """

    def get_pos(self) -> float:
        """
        Get the current playback position in seconds.

        :return: The playback position.
        :rtype: float

        :raises RuntimeError: If playback has not started.
        """

    def try_seek(self, position: float) -> None:
        """
        Attempt to seek to a specific position in the audio playback.

        :param position: The position in seconds to seek to.
        :type position: float

        :raises ValueError: If the position is negative or not a valid time in the audio.
        """

    def set_volume(self, volume: float) -> None:
        """
        Set the volume level for playback.

        :param volume: The volume level. Must be between 0.0 and 1.0.
        :type volume: float

        :raises ValueError: If the volume is not between 0.0 and 1.0.
        """

    def get_volume(self) -> float:
        """
        Get the current volume level.

        :return: The current volume level.
        :rtype: float
        """

    def set_duration(self, duration: float) -> None:
        """
        Set the length of the audio file to the meta data.

        :param duration: The duration. Must be a float
        :type volume: float

        """

    def get_remaining_time(self) -> float:
        """
        Get the remaining time of the audio playback.

        :return: The remaining time of the audio in seconds, rounded to two decimal places.
        :rtype: float
        :raises RuntimeError: If the audio duration is not available.
        :raises RuntimeError: If no sink is available or audio is not loaded.
        """

    def apply_effects(self, effect_list: list) -> None:
        """
        Apply a list of audio effects such as fade-in, fade-out, or speed changes.

        :param effect_list: A list of effects to apply. Each effect must be an instance of `FadeIn`, `FadeOut`, `ChangeSpeed`, or similar.
        :type effect_list: list
        :raises TypeError: If an unknown effect type is provided.
        :raises RuntimeError: If an error occurs while applying the effects.
        """

    def cancel_callback(self) -> None:
        """
        Cancels the current audio callback.

        This method sets a flag to indicate that the audio callback should be canceled.
        Once called, the audio sink will stop processing the current audio callback.

        Example:

        .. code-block:: python

            audio_sink = AudioSink()
            audio_sink.cancel_callback()
            print("Audio callback has been canceled.")

        :raises RuntimeError: If there is an issue acquiring the lock on the callback.
        """


class AudioChannel(Protocol):
    queue: List[AudioSink]
    auto_consume: bool
    currently_playing: Optional[AudioSink]
    effects_chain: List[ActionType]  # type: ignore

    def __init__(self) -> None:
        """
        Initializes a new AudioChannel instance with an empty queue, effects chain, and auto_consume set to False.

        Example:

        .. code-block:: python

            channel = AudioChannel()
            channel.auto_consume = True

        """

    def push(self, audio: AudioSink) -> None:
        """
        Adds an AudioSink object to the queue.

        Example:

        .. code-block:: python

            channel = AudioChannel()
            sink = AudioSink("my_audio_file.mp3")
            channel.push(sink)
        """

    @property
    def auto_consume(self) -> bool:
        """
        Returns whether the channel automatically consumes the queue.

        :rtype: bool
        """
        return self._auto_consume

    @auto_consume.setter
    def auto_consume(self, value: bool) -> None:
        """
        Sets the auto-consume behavior of the channel.

        Example:

        .. code-block:: python

            channel = AudioChannel()
            channel.set_auto_consume(True)

        :param value: True to enable auto-consume, False to disable.
        :type value: bool
        """

    def drop_current_audio(self) -> None:
        """
        Stops the currently playing audio, if any, and removes it from the channel.

        Example:

        .. code-block:: python

            channel = AudioChannel()
            channel.drop_current_audio()  # Stops and clears the currently playing audio
        """

    @property
    def current_audio(self) -> AudioSink:
        """
        Returns the currently playing AudioSink object.

        Example:

        .. code-block:: python

            channel = AudioChannel()
            current_sink = channel.current_audio()
            if current_sink:
                print("Currently playing:", current_sink)
            else:
                print("No audio is playing")

        :rtype: AudioSink
        """

    async def _control_loop(self) -> None:
        """
        Continuously monitors the queue and handles playback,
        auto-consume, and callback execution. Not meant for python access
        """

    @property
    def queue_contents(self) -> List[AudioSink]:
        """
        Returns the current queue of AudioSink objects.

        Example:

        .. code-block:: python

            channel = AudioChannel()
            queue = channel.queue_contents()
            print(f"Queue has {len(queue)} items")
        """

    def is_playing(self) -> bool:
        """
        Returns True if audio is currently playing, otherwise False.

        Example:

        .. code-block:: python

            channel = AudioChannel()
            if channel.is_playing():
                print("Audio is playing")
            else:
                print("No audio is playing")
        """

    def set_effects_chain(self, effect_list: list) -> None:
        """
        Sets the effects chain for the audio channel.

        This method accepts a list of effects and applies them to the audio channel. 
        The effects can include FadeIn, FadeOut, and ChangeSpeed.

        Example:

        .. code-block:: python

            channel = AudioChannel()
            fade_in_effect = FadeIn(start_val=0.0, end_val=1.0, duration=3.0)
            fade_out_effect = FadeOut(end_val=0.0, duration=10.0)
            speed_up_effect = ChangeSpeed(end_val=1.5, duration=5.0)

            channel.set_effects_chain([fade_in_effect, fade_out_effect, speed_up_effect])

        :param effect_list: A list of effects to set for the audio channel.
        :type effect_list: list
        :raises TypeError: If an unknown effect type is provided.
        """


class ChannelManager(Protocol):
    """
    Manages multiple audio channels and provides an API to control them.

        Example:

        .. code-block:: python

            # Intializing 2 audio sinks
            audio_1 = AudioSink(callback=on_audio_stop)
            audio_1.load_audio("ex.wav")
            audio_2 = AudioSink(callback=on_audio_stop)
            audio_2.load_audio("Acrylic.mp3")
            print(audio_1.metadata)

            # Intializing 1st audio channel
            channel_1 = AudioChannel()
            channel_1.push(audio_1)
            channel_1.push(audio_2)

            # Intializing 2 more audio sinks
            audio_3 = AudioSink(callback=on_audio_stop)
            audio_3.load_audio("ex.wav")
            audio_4 = AudioSink(callback=on_audio_stop)
            audio_4.load_audio("Acrylic.mp3")
            # Intializing 2nd audio channel
            channel_2 = AudioChannel()
            channel_2.push(audio_3)
            channel_2.push(audio_4)

            # Intializing ChannelManager
            manager = ChannelManager()
            manager.add_channel("Channel1", channel_1)
            manager.add_channel("Channel2", channel_2)

    :ivar channels: A dictionary mapping channel identifiers to their corresponding AudioChannel instances.
    :vartype channels: dict
    """
    channels: dict[str, AudioChannel]

    def __init__(self) -> None:
        """
        Initializes a new instance of ChannelManager.
        """

    def add_channel(self, name: str, channel: AudioChannel) -> None:
        """
        Adds a new audio channel to the manager.
        :param name: The unique identifier for the channel.
        :type name: str
        :param channel: The audio channel to add.
        :type channel: AudioChannel
        """

    def drop_channel(self, name: str) -> None:
        """
        Drops an audio channel from the manager.
        :param name: The unique identifier of the channel to drop.
        :type name: str
        :raises RuntimeError: If the channel is not found.
        """

    def channel(self, name: str) -> Optional[AudioChannel]:
        """
        Retrieves a channel by its identifier.
        :param name: The unique identifier of the channel.
        :type name: str
        :return: The corresponding AudioChannel instance, or None if not found.
        :rtype: Optional[AudioChannel]
        """

    def start_all(self) -> None:
        """
        Starts auto-consuming audio on all channels.
        """

    def stop_all(self) -> None:
        """
        Stops auto-consuming audio on all channels.
        """


class FadeIn:
    """
    Represents a fade-in effect for audio.

    :param duration: Duration of the fade-in effect in seconds. Defaults to 5.0.
    :param start_val: Starting volume value. Defaults to None.
    :param end_val: Ending volume value. Defaults to 1.0.
    :param apply_after: Time in seconds after which to apply the effect. Defaults to None.

    Example:

    .. code-block:: python

        fade_in = FadeIn(duration=3.0, start_val=0.2, end_val=1.0)
        # Applies a fade-in effect over 3 seconds, starting from 0.2 volume to full volume (1.0)
    """

    def __init__(self, duration=5.0, start_val=None, end_val=1.0, apply_after=None):
        pass


class FadeOut:
    """
    Represents a fade-out effect for audio.

    :param duration: Duration of the fade-out effect in seconds. Defaults to 5.0.
    :param start_val: Starting volume value. Defaults to 1.0.
    :param end_val: Ending volume value. Defaults to None.
    :param apply_after: Time in seconds after which to apply the effect. Defaults to None.

    Example:

    .. code-block:: python

        fade_out = FadeOut(duration=4.0, start_val=1.0, end_val=0.0)
        # Applies a fade-out effect over 4 seconds, fading from full volume (1.0) to silence (0.0)
    """

    def __init__(self, duration=5.0, start_val=1.0, end_val=None, apply_after=None):
        pass


class ChangeSpeed:
    """
    Represents a speed change effect for audio.

    :param duration: Duration of the speed change effect in seconds. Defaults to 0.0.
    :param start_val: Starting speed value. Defaults to 1.0.
    :param end_val: Ending speed value. Defaults to 1.5.
    :param apply_after: Time in seconds after which to apply the effect. Defaults to None.

    Example:

    .. code-block:: python

        change_speed = ChangeSpeed(duration=2.0, start_val=1.0, end_val=1.2)
        # Changes audio speed over 2 seconds from normal speed (1.0) to faster (1.2)
    """

    def __init__(self, duration=0.0, start_val=1.0, end_val=1.5, apply_after=None):
        pass
