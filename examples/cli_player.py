import os
from rpaudio import AudioChannel, AudioSink
from rpaudio.effects import FadeIn


def get_mp3_files(directory):
    mp3_files = []

    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(".mp3"):
                mp3_files.append(os.path.join(root, file))

    return mp3_files


def play_audio(channel):
    print("Audio is playing...")
    channel.current_audio.play()


def pause_audio(channel):
    print("Audio is paused.")
    channel.current_audio.pause()


def skip_audio(channel):
    print("Skipping...")
    channel.current_audio.stop()


def change_speed(channel, modifier):
    audio = channel.current_audio
    audio.set_speed(modifier*audio.get_speed())
    print(f"changing audio speed to {audio.get_speed():.2f}")


def change_volume(channel, modifier):
    audio = channel.current_audio
    if modifier > 1.0:
        if modifier*audio.get_volume() > 1.0:
            audio.set_volume(1.0)
            print("volume is at max")
            return

    elif modifier < 1.0:
        if modifier*audio.get_volume() < 0.0:
            audio.set_volume(0.0)
            print("volume is at min")
            return

    audio.set_volume(modifier*audio.get_volume())
    print(f"changing audio volume to {audio.get_volume():.2f}")


def main():
    directory = r"C:\Users\16145\Desktop\nondmc\ShroomheadOne"
    mp3_files = get_mp3_files(directory)
    audio_channel = AudioChannel()

    if mp3_files:
        for mp3 in mp3_files:
            audio = AudioSink().load_audio(mp3)
            audio_channel.push(audio)
    audio_channel.set_effects_chain([FadeIn(duration=3)])
    audio_channel.auto_consume = True

    print("Press 'p' to play/pause, and 'q' to quit.")
    playing = True

    while True:
        user_input = input("Command: ")

        match user_input:
            case 'p':
                if not playing:
                    play_audio(audio_channel)
                else:
                    pause_audio(audio_channel)
                playing = not playing
            case 'n':
                skip_audio(audio_channel)
            case 'S':
                change_speed(audio_channel, 1.10)
            case 's':
                change_speed(audio_channel, 0.90)
            case 'V':
                change_volume(audio_channel, 1.10)
            case 'v':
                change_volume(audio_channel, 0.90)
            case 'q':
                print("Exiting...")
                break
            case _:
                print("Invalid input, press 'p' to play/pause or 'q' to quit.")


if __name__ == "__main__":
    main()
