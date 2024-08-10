from typing import Optional, Callable, Protocol


class AudioHandler(Protocol):
    """
    AudioHandler. Wraps audio playback functionality.

    This class provides methods to load, play, pause, and stop audio playback.
    An optional callback function can be called when the audio stops playing if provided.

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
