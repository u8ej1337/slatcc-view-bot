import tls_client
from pystyle import Colors, Colorate, Center
import random
import threading
import ctypes
from fake_useragent import UserAgent
import os
import time
from datetime import datetime, UTC

session = tls_client.Session(client_identifier="chrome_104", random_tls_extension_order=True)
useragent = UserAgent()

with open("proxies.txt", "r") as file:
    proxies = [line.strip() for line in file]
    file.close()

username = ""
threads = 0

class stats:
    views_sent = 0
    views_failed = 0

def update_title():
    while True:
        ctypes.windll.kernel32.SetConsoleTitleW(f"NiggaBot Beta | Slat.cc View Bot | Views Sent: {stats.views_sent} | Views Failed: {stats.views_failed}")

def banner(expires=False):
    print(Center.XCenter(Colorate.Vertical(Colors.blue_to_red, f"""                     
███▄    █  ██▓  ▄████   ▄████  ▄▄▄       ▄▄▄▄    ▒█████  ▄▄▄█████▓
██ ▀█   █ ▓██▒ ██▒ ▀█▒ ██▒ ▀█▒▒████▄    ▓█████▄ ▒██▒  ██▒▓  ██▒ ▓▒
▓██  ▀█ ██▒▒██▒▒██░▄▄▄░▒██░▄▄▄░▒██  ▀█▄  ▒██▒ ▄██▒██░  ██▒▒ ▓██░ ▒░
▓██▒  ▐▌██▒░██░░▓█  ██▓░▓█  ██▓░██▄▄▄▄██ ▒██░█▀  ▒██   ██░░ ▓██▓ ░ 
▒██░   ▓██░░██░░▒▓███▀▒░▒▓███▀▒ ▓█   ▓██▒░▓█  ▀█▓░ ████▓▒░  ▒██▒ ░ 
░ ▒░   ▒ ▒ ░▓   ░▒   ▒  ░▒   ▒  ▒▒   ▓▒█░░▒▓███▀▒░ ▒░▒░▒░   ▒ ░░   
░ ░░   ░ ▒░ ▒ ░  ░   ░   ░   ░   ▒   ▒▒ ░▒░▒   ░   ░ ▒ ▒░     ░    
░   ░ ░  ▒ ░░ ░   ░ ░ ░   ░   ░   ▒    ░    ░ ░ ░ ░ ▒    ░      
        ░  ░        ░       ░       ░  ░ ░          ░ ░           
    Slat.cc View Bot                           ░  By @u8ej :)   
                
""")))

def send_slat_request():
    user_agent = useragent.random
    headers = {
        "accept": "*/*",
        "accept-language": "en-US,en;q=0.5",
        "content-type": "application/json",
        "priority": "u=1, i",
        "sec-ch-ua-mobile": "?0",
        "sec-ch-ua-platform": "\"Windows\"",
        "sec-gpc": "1",
        "User-Agent": f"{user_agent}",
        "Referer": f"https://slat.cc/{username}",
    }
    try:
        req = session.post(f"https://slat.cc/api/biolink/{username}/views", headers=headers, proxy={
            "http": "http://" + random.choice(proxies),
        })
    except:
        return
    
    if req.status_code == 200:
        stats.views_sent += 1
        print("[" + Colorate.Color(Colors.green, "SUCCESS", 1) + "] " + f"Sent view to {username}!")
    else:
        stats.views_failed += 1
        print("[" + Colorate.Color(Colors.red, "ERROR", 1) + "] " + f"Failed to send view to {username}!")

def slat_loop():
    while True:
        try:
            send_slat_request()
        except KeyboardInterrupt:
            exit(0)

ctypes.windll.kernel32.SetConsoleTitleW("NiggaBot Beta | Slat.cc View Bot")

banner()
username = str(input(Colorate.Horizontal(Colors.blue_to_red, "Slat.cc Username\n> ", 1)))
threads = int(input(Colorate.Horizontal(Colors.blue_to_red, "Number of threads\n> ", 1)))

os.system("cls")
banner()

threading.Thread(target=update_title).start()

threadsstarted = []
for _ in range(threads):
    thread = threading.Thread(target=slat_loop)
    threadsstarted.append(thread)
    thread.start()

for th in threadsstarted:
    th.join()