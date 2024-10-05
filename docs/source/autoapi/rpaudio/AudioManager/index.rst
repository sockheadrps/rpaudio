rpaudio.AudioManager
====================

.. py:module:: rpaudio.AudioManager




Module Contents
---------------

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


   .. py:method:: add_channel(name: str, channel: rpaudio.AudioChannel) -> None

      Adds a new audio channel to the manager.
      :param name: The unique identifier for the channel.
      :type name: str
      :param channel: The audio channel to add.
      :type channel: AudioChannel



   .. py:method:: channel(name: str) -> Optional[rpaudio.AudioChannel]

      Retrieves a channel by its identifier.
      :param name: The unique identifier of the channel.
      :type name: str
      :return: The corresponding AudioChannel instance, or None if not found.
      :rtype: Optional[AudioChannel]



   .. py:method:: drop_channel(name: str) -> None

      Drops an audio channel from the manager.
      :param name: The unique identifier of the channel to drop.
      :type name: str
      :raises RuntimeError: If the channel is not found.



   .. py:method:: start_all() -> None

      Starts auto-consuming audio on all channels.



   .. py:method:: stop_all() -> None

      Stops auto-consuming audio on all channels.



   .. py:attribute:: channels
      :type:  dict[str, rpaudio.AudioChannel]


