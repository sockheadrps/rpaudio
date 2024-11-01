import rpaudio
import time

kill_audio = False
AUDIO_FILE = r"C:\Users\16145\Desktop\a1.mp3"

def on_audio_stop():
    global kill_audio
    kill_audio = True
    print("Audio has stopped")

def play_audio():
    # Initialize and load the audio file
    handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(AUDIO_FILE)

    # Set the volume to 50% (optional)
    handler.set_volume(0.5)
    handler.play()
    
    # Loop until the audio completes, print the time elapsed
    i = 0
    while not kill_audio:
        i += 1
        time.sleep(1)
        print(i)

if __name__ == "__main__":
    play_audio()
