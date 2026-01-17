# Systemd Installation

## Setup

1. Copy the service file:
```bash
sudo cp fuzzyserve.service /etc/systemd/system/
```

2. Edit the service file to match your setup:
```bash
sudo vim /etc/systemd/system/fuzzyserve.service
```

Adjust:
- `User=` and `Group=` to match your system user
- `--media-root` path to your media directory
- `--port` if needed

3. Ensure the user has access to media directory:
```bash
sudo chown -R www-data:www-data /path/to/media
# or just read access:
sudo chmod -R o+r /path/to/media
```

4. Reload systemd:
```bash
sudo systemctl daemon-reload
```

5. Enable autostart on boot:
```bash
sudo systemctl enable fuzzyserve
```

6. Start the service:
```bash
sudo systemctl start fuzzyserve
```

## Commands
```bash
# Check status
sudo systemctl status fuzzyserve

# Stop
sudo systemctl stop fuzzyserve

# Restart
sudo systemctl restart fuzzyserve

# Disable autostart
sudo systemctl disable fuzzyserve
```

## Logs
```bash
# Follow logs
journalctl -u fuzzyserve -f

# Last 100 lines
journalctl -u fuzzyserve -n 100

# Logs since boot
journalctl -u fuzzyserve -b
```
