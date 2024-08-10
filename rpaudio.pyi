from typing import Optional, Callable, Protocol


class AudioHandler(Protocol):
    """
    AudioHandler. Wraps audio playback functionality.

    This class provides methods to load, play, pause, and stop audio playback.
    An optional callback function can be called when the audio stops playing if provided.

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

    def __init__(self, callback: Optional[Callable[[], None]] = None) -> None:
        """
        Constructor method.

        Initializes an instance of AudioHandler with an optional callback function.

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
        ...

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
        ...

    def load_audio(self, filename: str) -> None:
        """
        Load an audio file for playback.

        Args:
            filename (str): The path to the audio file to load.

        Returns:
            None: This method does not return any value.

        Example:
        
        .. code-block:: python

            handler = AudioHandler(callback=my_callback)
            handler.load_audio("my_audio_file.mp3")
        """
        ...

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
        ...

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
        ...

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
        ...
