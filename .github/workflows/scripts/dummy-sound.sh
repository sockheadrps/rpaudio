sudo tee /etc/asound.conf <<EOF
pcm.dummy {
    type plug
    slave.pcm "null"
}
ctl.dummy {
    type hw
    card 0
}
defaults.pcm.card 0
defaults.ctl.card 0
EOF

# Restart ALSA (if supported)
sudo alsa force-reload


# Check ALSA status and configuration
aplay -l
cat /etc/asound.conf