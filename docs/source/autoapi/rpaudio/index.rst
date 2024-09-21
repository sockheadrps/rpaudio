rpaudio
=======

.. py:module:: rpaudio




Package Contents
----------------

.. py:class:: AudioChannel(channel_id, channel_callback)

   Bases: :py:obj:`Protocol`


   Manages a queue of AudioSink objects and handles playback.

   :param channel_id: A unique identifier for the audio channel.
   :type channel_id: Union[int, str]
   :param channel_callback: (optional) A callback invoked when the queue is idle.
   :type channel_callback: Optional[Callable[[], None]]


   .. py:method:: drop_current_audio()

      Drops the current audio from the queue.



   .. py:method:: push(audio)

      Adds an AudioSink to the channel queue.

      :param audio: The audio object to add to the queue.
      :type audio: AudioSink



   .. py:property:: auto_consume
      :type: bool

      Returns whether the channel automatically consumes the queue.

      :rtype: bool


   .. py:attribute:: channel_callback


   .. py:attribute:: channel_id


   .. py:property:: current_audio
      :type: AudioSink

      Returns the currently playing AudioSink object.

      :rtype: AudioSink


   .. py:attribute:: queue
      :value: []



   .. py:property:: queue_contents
      :type: List[AudioSink]

      Returns the list of AudioSink objects currently in the queue.

      :rtype: List[AudioSink]


.. py:class:: AudioSink(callback = None)

   Bases: :py:obj:`Protocol`


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



   .. py:method:: set_effects(effect_list)

      Apply effects from a list to the audio playback.

      :param effect_list: A list of effect objects to be applied to the audio playback.
                          The list may contain instances of `FadeIn`, `FadeOut`, and `ChangeSpeed`.
      :type effect_list: list

      :raises TypeError: If an unknown effect type is encountered in the list.



   .. py:method:: set_speed(speed)

      Set the playback speed of the audio.

      :param speed: The playback speed. Must be greater than 0.
      :type speed: float

      :raises ValueError: If the speed is less than or equal to 0.



   .. py:method:: set_volume(volume)

      Set the volume level for playback.

      :param volume: The volume level. Must be between 0.0 and 1.0.
      :type volume: float

      :raises ValueError: If the volume is not between 0.0 and 1.0.



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

   Bases: :py:obj:`Protocol`


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


.. py:class:: FadeIn(duration = 5.0, start_vol = 0.1, end_vol = 1.0, apply_after = None)

   Represents a fade-in effect for audio playback.

   :param duration: The duration of the fade-in effect in seconds. Defaults to 5.0 seconds.
   :type duration: float, optional
   :param start_vol: The starting volume level of the fade-in. Must be between 0.0 and 1.0. Defaults to 0.1.
   :type start_vol: float, optional
   :param end_vol: The ending volume level of the fade-in. Must be between 0.0 and 1.0. Defaults to 1.0.
   :type end_vol: float, optional
   :param apply_after: Time delay before applying the fade-in effect, optional.
   :type apply_after: float, optional

   :raises ValueError: If duration is negative or volumes are out of range.


   .. py:attribute:: apply_after
      :type:  float | None


   .. py:attribute:: duration
      :type:  float


   .. py:attribute:: end_vol
      :type:  float


   .. py:attribute:: start_vol
      :type:  float


.. py:class:: FadeOut(duration = 5.0, start_vol = 1.0, end_vol = 0.1, apply_after = None)

   Represents a fade-out effect for audio playback.

   :param duration: The duration of the fade-out effect in seconds. Defaults to 5.0 seconds.
   :type duration: float, optional
   :param start_vol: The starting volume level of the fade-out. Must be between 0.0 and 1.0. Defaults to 1.0.
   :type start_vol: float, optional
   :param end_vol: The ending volume level of the fade-out. Must be between 0.0 and 1.0. Defaults to 0.1.
   :type end_vol: float, optional
   :param apply_after: Time delay before applying the fade-out effect, optional.
   :type apply_after: float, optional

   :raises ValueError: If duration is negative or volumes are out of range.


   .. py:attribute:: apply_after
      :type:  float | None


   .. py:attribute:: duration
      :type:  float


   .. py:attribute:: end_vol
      :type:  float


   .. py:attribute:: start_vol
      :type:  float


.. py:class:: MetaData(audio_sink)

   A class representing metadata for an audio file.


   .. py:property:: album_artist
      :type: Optional[str]

      Get the album artist of the audio file.

      :return: The album artist of the audio file, or None if not available.
      :rtype: Optional[str]


   .. py:property:: album_title
      :type: Optional[str]

      Get the album title of the audio file.

      :return: The album title of the audio file, or None if not available.
      :rtype: Optional[str]


   .. py:property:: artist
      :type: Optional[str]

      Get the artist of the audio file.

      :return: The artist of the audio file, or None if not available.
      :rtype: Optional[str]


   .. py:property:: channels
      :type: Optional[str]

      Get the number of channels in the audio file.

      :return: The number of channels, or None if not available.
      :rtype: Optional[str]


   .. py:property:: comment
      :type: Optional[str]

      Get the comment associated with the audio file.

      :return: The comment of the audio file, or None if not available.
      :rtype: Optional[str]


   .. py:property:: composer
      :type: Optional[str]

      Get the composer of the audio file.

      :return: The composer of the audio file, or None if not available.
      :rtype: Optional[str]


   .. py:property:: date
      :type: Optional[str]

      Get the date associated with the audio file.

      :return: The date of the audio file, or None if not available.
      :rtype: Optional[str]


   .. py:property:: disc_number
      :type: Optional[str]

      Get the disc number of the audio file.

      :return: The disc number, or None if not available.
      :rtype: Optional[str]


   .. py:property:: duration
      :type: Optional[float]

      Get the duration of the audio file in seconds.

      :return: The duration of the audio file, or None if not available.
      :rtype: Optional[float]


   .. py:property:: genre
      :type: Optional[str]

      Get the genre of the audio file.

      :return: The genre of the audio file, or None if not available.
      :rtype: Optional[str]


   .. py:property:: sample_rate
      :type: Optional[int]

      Get the sample rate of the audio file.

      :return: The sample rate of the audio file, or None if not available.
      :rtype: Optional[int]


   .. py:property:: title
      :type: Optional[str]

      Get the title of the audio file.

      :return: The title of the audio file, or None if not available.
      :rtype: Optional[str]


   .. py:property:: total_discs
      :type: Optional[str]

      Get the total number of discs in the album.

      :return: The total number of discs, or None if not available.
      :rtype: Optional[str]


   .. py:property:: total_tracks
      :type: Optional[str]

      Get the total number of tracks in the album.

      :return: The total number of tracks, or None if not available.
      :rtype: Optional[str]


   .. py:property:: track_number
      :type: Optional[str]

      Get the track number of the audio file.

      :return: The track number of the audio file, or None if not available.
      :rtype: Optional[str]


   .. py:property:: year
      :type: Optional[str]

      Get the year the audio file was released.

      :return: The year of the audio file, or None if not available.
      :rtype: Optional[str]


