#!/bin/bash

# Create or overwrite the ~/.asoundrc file with PulseAudio settings
cat <<EOL > ~/.asoundrc
pcm.!default {
    type pulse
    device default
}

ctl.!default {
    type pulse
}
EOL

# Restart PulseAudio
pulseaudio --kill

# Load PulseAudio daemon in the background
pulseaudio -D --exit-idle-time=-1

# Create virtual output device (used for audio playback)
pactl load-module module-null-sink sink_name=DummyOutput sink_properties=device.description="Virtual_Dummy_Output"

# Set the newly created virtual sink as the default sink
pactl set-default-sink DummyOutput

# Check and create symlinks for ALSA libraries if necessary
echo "Checking for ALSA libraries..."
MISSING_LIBS=0

# Create the ALSA lib64 directory if it doesn't exist
if [ ! -d /usr/lib64/alsa-lib ]; then
    sudo mkdir -p /usr/lib64/alsa-lib
fi

# Check for each required library
for LIB in libasound_module_conf_pulse.so libasound_module_pcm_jack.so libasound_module_pcm_pulse.so; do
    if [ ! -f "/usr/lib64/alsa-lib/$LIB" ]; then
        echo "Creating symlink for $LIB..."
        sudo ln -s "/usr/lib/x86_64-linux-gnu/alsa-lib/$LIB" "/usr/lib64/alsa-lib/$LIB"
        if [ $? -eq 0 ]; then
            echo "Symlink for $LIB created successfully."
        else
            echo "Failed to create symlink for $LIB."
            MISSING_LIBS=1
        fi
    else
        echo "$LIB already exists."
    fi
done

if [ $MISSING_LIBS -eq 0 ]; then
    echo "All necessary ALSA libraries are linked."
else
    echo "Some libraries could not be linked."
fi

# Verify the setup
echo "ALSA configuration complete. Verifying..."
aplay -L | grep pulse
