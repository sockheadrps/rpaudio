import rpaudio
from time import sleep

# Create an instance of AudioHandler
audio_handler = rpaudio.AudioHandler()

# Load an audio file
audio_handler.load_audio("ex.wav")
print("Audio loaded")
sleep(2)
audio_handler.pause()
print("Audio paused")
sleep(2)
audio_handler.resume()
print("Audio resumed")
sleep(2)
print("stopping audio")
audio_handler.stop()

