#!/usr/bin/env python3
import os
import requests
import time
import itertools

API = "https://codeforces.com/api/contest.list?gym=false"
HEADERS = {"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) cd-uptime-monitor/1.0"}

os.system('notify-send "Codeforces Monitor" "Codeforces is up!"')
os.system('aplay /usr/share/sounds/alsa/Noise.wav')

for _ in itertools.count():
    response = requests.get(API, headers=HEADERS, timeout=10)
    if response.status_code == 200:
        data = response.json()
        if data.get("status") == "OK":
            alert()
            break
    time.sleep(60)
