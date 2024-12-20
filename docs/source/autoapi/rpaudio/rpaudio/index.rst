rpaudio.rpaudio
===============

.. py:module:: rpaudio.rpaudio




Module Contents
---------------

.. py:class:: AudioChannel

   .. py:method:: _control_loop()
      :async:


      Continuously monitors the queue and handles playback,
      auto-consume, and callback execution. Not meant for python access



   .. py:method:: current_audio_data()

      Retrieves metadata and current playback information.

      This method returns a dictionary containing various metadata fields such
      as album artist, album title, artist, channels, duration, and more,
      along with current playback information like volume and position.

      :returns: A dictionary with audio
                metadata and playback details, including:
                    - album_artist (str): The artist of the album.
                    - album_title (str): The title of the album.
                    - artist (str): The artist of the audio track.
                    - channels (int): The number of audio channels.
                    - comment (Optional[str]): Comments about the track.
                    - composer (Optional[str]): The composer of the audio.
                    - date (Optional[str]): The release date of the audio.
                    - disc_number (Optional[int]): The disc number in a multi-disc set.
                    - duration (float): The duration of the audio in seconds.
                    - genre (Optional[str]): The genre of the audio.
                    - sample_rate (int): The sample rate of the audio in Hz.
                    - title (str): The title of the audio track.
                    - total_discs (Optional[int]): The total number of discs in a multi-disc set.
                    - total_tracks (Optional[int]): The total number of tracks in the album.
                    - track_number (Optional[int]): The track number on the album.
                    - year (Optional[int]): The year the audio was released.
                    - speed (float): The current playback speed.
                    - position (float): The current playback position in seconds.
                    - volume (float): The current volume level.
                    - effects (List[Dict[str, Any]]): List of effects applied to the audio.
      :rtype: Dict[str, Union[str, float, int, None]]



   .. py:method:: drop_current_audio()

      Stops the currently playing audio, if any, and removes it from the channel.

      Example:

      .. code-block:: python

          channel = AudioChannel()
          channel.drop_current_audio()  # Stops and clears the currently playing audio



   .. py:method:: is_playing()

      Returns True if audio is currently playing, otherwise False.

      Example:

      .. code-block:: python

          channel = AudioChannel()
          if channel.is_playing():
              print("Audio is playing")
          else:
              print("No audio is playing")



   .. py:method:: push(audio)

      Adds an AudioSink object to the queue.

      Example:

      .. code-block:: python

          channel = AudioChannel()
          sink = AudioSink("my_audio_file.mp3")
          channel.push(sink)



   .. py:method:: set_effects_chain(effect_list)

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



   .. py:property:: auto_consume
      :type: bool

      Returns whether the channel automatically consumes the queue.

      :rtype: bool


   .. py:property:: current_audio
      :type: AudioSink

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


   .. py:attribute:: currently_playing
      :type:  Optional[AudioSink]


   .. py:attribute:: effects_chain
      :type:  List[ActionType]


   .. py:attribute:: queue
      :type:  List[AudioSink]


   .. py:property:: queue_contents
      :type: List[AudioSink]

      Returns the current queue of AudioSink objects.

      Example:

      .. code-block:: python

          channel = AudioChannel()
          queue = channel.queue_contents()
          print(f"Queue has {len(queue)} items")


.. py:class:: AudioSink(callback = None)

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


   .. py:method:: apply_effects(effect_list)

      Apply a list of audio effects such as fade-in, fade-out, or speed changes.

      :param effect_list: A list of effects to apply. Each effect must be an instance of `FadeIn`, `FadeOut`, `ChangeSpeed`, or similar.
      :type effect_list: list
      :raises TypeError: If an unknown effect type is provided.
      :raises RuntimeError: If an error occurs while applying the effects.



   .. py:method:: cancel_callback()

      Cancels the current audio callback.

      This method sets a flag to indicate that the audio callback should be canceled.
      Once called, the audio sink will stop processing the current audio callback.

      Example:

      .. code-block:: python

          audio_sink = AudioSink()
          audio_sink.cancel_callback()
          print("Audio callback has been canceled.")

      :raises RuntimeError: If there is an issue acquiring the lock on the callback.



   .. py:method:: get_pos()

      Get the current playback position in seconds.

      :return: The playback position.
      :rtype: float

      :raises RuntimeError: If playback has not started.



   .. py:method:: get_remaining_time()

      Get the remaining time of the audio playback.

      :return: The remaining time of the audio in seconds, rounded to two decimal places.
      :rtype: float
      :raises RuntimeError: If the audio duration is not available.
      :raises RuntimeError: If no sink is available or audio is not loaded.



   .. py:method:: get_speed()

      Get the current playback speed of the audio.

      :return: The playback speed.
      :rtype: float



   .. py:method:: get_volume()

      Get the current volume level.

      :return: The current volume level.
      :rtype: float



   .. py:method:: load_audio(filename)

      Load an audio file for playback.

      :param filename: The path to the audio file to load.
      :type filename: str



   .. py:method:: pause()

      Pause the currently playing audio, if any.

      :raises RuntimeError: If no audio has been loaded.

      Example:

      .. code-block:: python

          handler = AudioHandler(callback=my_callback)
          handler.load_audio("my_audio_file.mp3")
          handler.play()
          handler.pause()



   .. py:method:: play()

      Start playing the loaded audio.

      This method begins playback of the audio that was loaded using the `load_audio` method.
      If the audio is already playing, this method has no effect.

      :raises RuntimeError: If no audio has been loaded.

      Example:

      .. code-block:: python

          handler = AudioHandler(callback=my_callback)
          handler.load_audio("my_audio_file.mp3")
          handler.play()



   .. py:method:: set_duration(duration)

      Set the length of the audio file to the meta data.

      :param duration: The duration. Must be a float
      :type volume: float




   .. py:method:: set_speed(speed)

      Set the playback speed of the audio.

      :param speed: The playback speed. Must be a float.
      :type speed: float

      :raises ValueError: If the speed is not a valid float.
      :raises EffectConflictException: Raised when an attempt is made to change the volume while
      effects are actively being applied. This ensures that audio effects do not conflict during playback.



   .. py:method:: set_volume(volume)

      Set the volume level for playback.

      :param volume: The volume level. Must be between 0.0 and 1.0.
      :type volume: float

      :raises ValueError: If the volume is not between 0.0 and 1.0.
      :raises EffectConflictException: Raised when an attempt is made to change the volume while
      effects are actively being applied. This ensures that audio effects do not conflict during playback.



   .. py:method:: stop()

      Stop the currently playing audio, if any.

      :raises RuntimeError: If no audio has been loaded.

      Example:

      .. code-block:: python

          handler = AudioHandler(callback=my_callback)
          handler.load_audio("my_audio_file.mp3")
          handler.play()
          handler.stop()



   .. py:method:: try_seek(position)

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


.. py:class:: ChannelManager

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


   .. py:method:: add_channel(name, channel)

      Adds a new audio channel to the manager.
      :param name: The unique identifier for the channel.
      :type name: str
      :param channel: The audio channel to add.
      :type channel: AudioChannel



   .. py:method:: channel(name)

      Retrieves a channel by its identifier.
      :param name: The unique identifier of the channel.
      :type name: str
      :return: The corresponding AudioChannel instance, or None if not found.
      :rtype: Optional[AudioChannel]



   .. py:method:: drop_channel(name)

      Drops an audio channel from the manager.
      :param name: The unique identifier of the channel to drop.
      :type name: str
      :raises RuntimeError: If the channel is not found.



   .. py:method:: start_all()

      Starts auto-consuming audio on all channels.



   .. py:method:: stop_all()

      Stops auto-consuming audio on all channels.



   .. py:attribute:: channels
      :type:  dict[str, AudioChannel]


