import rpaudio
from time import sleep
def on_audio_stop():
    print("Audio has stopped")

handler = rpaudio.AudioHandler(callback=on_audio_stop)
handler.load_audio("ex.wav")
handler.play()
while handler.is_playing():
    sleep(1)
    print(handler.is_playing())
