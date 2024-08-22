import asyncio
from rpaudio import AudioSink, AudioChannel, ChannelManager



def on_audio_stop():
    print("Audio stopped.")


async def run_manager(manager: ChannelManager):
    # Autoplay Channel1
    manager.channel("Channel1").auto_consume = True
    await asyncio.sleep(3)

    # Autoplay all channels (channel2 not autoplayed)
    manager.start_all()
    await asyncio.sleep(3)

    # Get the current audio metadata of channel1
    print(f"{manager.channel('Channel1').current_audio.metadata} is playing in Channel1")

    # Pause channel1
    manager.channel("Channel1").current_audio.pause()
    await asyncio.sleep(3)

    # Stop channel1's current audio, autoplays next audio in queue if auto_consume is True
    manager.channel("Channel1").current_audio.cancel_callback()
    manager.channel("Channel1").current_audio.stop()
    await asyncio.sleep(3)

    # Stop the remaining audio in channel1 which exhausts its queue
    manager.channel("Channel1").current_audio.cancel_callback()
    manager.channel("Channel1").current_audio.stop()
    await asyncio.sleep(3)

    # Stop channel2's current audio, autoplays next audio in queue if auto_consume is True
    manager.channel("Channel2").current_audio.stop()
    await asyncio.sleep(3)

    # Stop the remaining audio in channel2 which exhaust its queue
    manager.channel("Channel2").current_audio.stop()
    await asyncio.sleep(1)


async def main():
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

    await asyncio.gather(
        run_manager(manager),
    )


if __name__ == "__main__":
    asyncio.run(main())
