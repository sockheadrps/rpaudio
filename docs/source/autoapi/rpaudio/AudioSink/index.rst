rpaudio.AudioSink
=================

.. py:module:: rpaudio.AudioSink




Module Contents
---------------

.. py:class:: AudioSink(callback: Optional[Callable[[], None]] = None)

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

   :param callback: A function that will be called when the audio stops playing.
   :type callback: :py:class:`Optional[Callable[[], None]]`

   :ivar is_playing: Flag indicating whether the audio is currently playing.

   :vartype is_playing: :py:class:`bool`


   .. py:method:: apply_effects(effect_list: list) -> None

      Apply a list of audio effects such as fade-in, fade-out, or speed changes.

      :param effect_list: A list of effects to apply. Each effect must be an instance of `FadeIn`, `FadeOut`, `ChangeSpeed`, or similar.
      :type effect_list: list
      :raises TypeError: If an unknown effect type is provided.
      :raises RuntimeError: If an error occurs while applying the effects.



   .. py:method:: cancel_callback() -> None

      Cancels the current audio callback.

      This method sets a flag to indicate that the audio callback should be canceled.
      Once called, the audio sink will stop processing the current audio callback.

      Example:

      .. code-block:: python

          audio_sink = AudioSink()
          audio_sink.cancel_callback()
          print("Audio callback has been canceled.")

      :raises RuntimeError: If there is an issue acquiring the lock on the callback.



   .. py:method:: get_pos() -> float

      Get the current playback position in seconds.

      :return: The playback position.
      :rtype: float

      :raises RuntimeError: If playback has not started.



   .. py:method:: get_remaining_time() -> float

      Get the remaining time of the audio playback.

      :return: The remaining time of the audio in seconds, rounded to two decimal places.
      :rtype: float
      :raises RuntimeError: If the audio duration is not available.
      :raises RuntimeError: If no sink is available or audio is not loaded.



   .. py:method:: get_speed() -> float

      Get the current playback speed of the audio.

      :return: The playback speed.
      :rtype: float



   .. py:method:: get_volume() -> float

      Get the current volume level.

      :return: The current volume level.
      :rtype: float



   .. py:method:: load_audio(filename: str) -> AudioSink

      Load an audio file for playback.

      :param filename: The path to the audio file to load.
      :type filename: str



   .. py:method:: pause() -> None

      Pause the currently playing audio, if any.

      :raises RuntimeError: If no audio has been loaded.

      Example:

      .. code-block:: python

          handler = AudioHandler(callback=my_callback)
          handler.load_audio("my_audio_file.mp3")
          handler.play()
          handler.pause()



   .. py:method:: play() -> None

      Start playing the loaded audio.

      This method begins playback of the audio that was loaded using the `load_audio` method.
      If the audio is already playing, this method has no effect.

      :raises RuntimeError: If no audio has been loaded.

      Example:

      .. code-block:: python

          handler = AudioHandler(callback=my_callback)
          handler.load_audio("my_audio_file.mp3")
          handler.play()



   .. py:method:: set_duration(duration: float) -> None

      Set the length of the audio file to the meta data.

      :param duration: The duration. Must be a float
      :type volume: float




   .. py:method:: set_speed(speed: float) -> None

      Set the playback speed of the audio.

      :param speed: The playback speed. Must be a float.
      :type speed: float

      :raises ValueError: If the speed is not a valid float.
      :raises EffectConflictException: Raised when an attempt is made to change the volume while
      effects are actively being applied. This ensures that audio effects do not conflict during playback.



   .. py:method:: set_volume(volume: float) -> None

      Set the volume level for playback.

      :param volume: The volume level. Must be between 0.0 and 1.0.
      :type volume: float

      :raises ValueError: If the volume is not between 0.0 and 1.0.
      :raises EffectConflictException: Raised when an attempt is made to change the volume while
      effects are actively being applied. This ensures that audio effects do not conflict during playback.



   .. py:method:: stop() -> None

      Stop the currently playing audio, if any.

      :raises RuntimeError: If no audio has been loaded.

      Example:

      .. code-block:: python

          handler = AudioHandler(callback=my_callback)
          handler.load_audio("my_audio_file.mp3")
          handler.play()
          handler.stop()



   .. py:method:: try_seek(position: float) -> None

      Attempt to seek to a specific position in the audio playback.

      :param position: The position in seconds to seek to.
      :type position: float

      :raises ValueError: If the position is negative or not a valid time in the audio.



   .. py:property:: is_playing
      :type: bool

      Flag indicating whether the audio is currently playing.

      :returns: True if the audio is playing, False otherwise.
      :rtype: bool

      Example:

      .. code-block:: python

          handler = AudioHandler(callback=my_callback)
          handler.load_audio("my_audio_file.mp3")
          handler.play()
          print(handler.is_playing)  # True if audio is playing


   .. py:property:: metadata
      :type: dict[str, any]

      Get metadata for the audio file.

      Example:

      .. code-block:: python

          audio_1: rpaudio.AudioSink = rpaudio.AudioSink(callback=on_audio_stop)
          audio_1.load_audio("ex.wav")
          data = audio_1.metadata

      :return: A dictionary containing metadata for the audio file.
      :rtype: dict[str, any]


