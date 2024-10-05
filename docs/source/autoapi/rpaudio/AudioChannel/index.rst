rpaudio.AudioChannel
====================

.. py:module:: rpaudio.AudioChannel




Module Contents
---------------

.. py:class:: AudioChannel

   .. py:method:: current_audio_data() -> Dict[str, Union[str, float, int, None]]

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



   .. py:method:: drop_current_audio() -> None

      Stops the currently playing audio, if any, and removes it from the channel.

      Example:

      .. code-block:: python

          channel = AudioChannel()
          channel.drop_current_audio()  # Stops and clears the currently playing audio



   .. py:method:: is_playing() -> bool

      Returns True if audio is currently playing, otherwise False.

      Example:

      .. code-block:: python

          channel = AudioChannel()
          if channel.is_playing():
              print("Audio is playing")
          else:
              print("No audio is playing")



   .. py:method:: push(audio: rpaudio.AudioSink) -> None

      Adds an AudioSink object to the queue.

      Example:

      .. code-block:: python

          channel = AudioChannel()
          sink = AudioSink("my_audio_file.mp3")
          channel.push(sink)



   .. py:method:: set_effects_chain(effect_list: list) -> None

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
      :type: rpaudio.AudioSink

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
      :type:  Optional[rpaudio.AudioSink]


   .. py:attribute:: effects_chain
      :type:  List[ActionType]


   .. py:attribute:: queue
      :type:  List[rpaudio.AudioSink]


   .. py:property:: queue_contents
      :type: List[rpaudio.AudioSink]

      Returns the current queue of AudioSink objects.

      Example:

      .. code-block:: python

          channel = AudioChannel()
          queue = channel.queue_contents()
          print(f"Queue has {len(queue)} items")


