import rpaudio
import threading
import time

def on_audio_stop():
    print("Audio has stopped")

def play_audio():
    handler = rpaudio.AudioSink(callback=on_audio_stop)
    handler.load_audio("ex.wav")
    handler.play()
    count = 0
    while handler.is_playing:
        time.sleep(1)
        count += 1

        if count == 4:
            handler.pause()
            time.sleep(2)
            handler.play()
            time.sleep(1)
            handler.stop()

def sleep_loop():
    for i in range(10):
        print(f"Second {i + 1}")
        time.sleep(1)

def main():
    # Create threads
    audio_thread = threading.Thread(target=play_audio)
    print_thread = threading.Thread(target=sleep_loop)
    
    # Start threads
    audio_thread.start()
    print_thread.start()
    
    # Wait for threads to finish
    audio_thread.join()
    print_thread.join()

if __name__ == "__main__":
    main()
