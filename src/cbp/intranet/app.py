import typer 



app = typer.Typer()

GATEWAY_ADDR = "10.1.10.1"
HOST_DENALI = "10.1.10.150"
ROUTER_MY_LINK = "10.1.10.68"
ROUTER_MY_STATION = "10.1.10.98"
TETON_WIFI_CONNECTION = "10.1.10.159"
HOST_SIERRA = "10.1.10.45"
HOST_SAMSUNG = "10.1.10.239"


Commands = {
    "host_devices": "ip route show",
    "id_net_devices_and_services": "sudo nmap -O -sV 10.1.10.0/24",
    "whats_my_ip": "curl -4 ifconfig.me"
}

example_id_net_devices_and_services = """
makeroftools@denali:~$ sudo nmap -O -sV 10.1.10.0/24
Starting Nmap 7.95 ( https://nmap.org ) at 2025-11-11 16:36 MST
Nmap scan report for _gateway (10.1.10.1)
Host is up (0.0016s latency).
Not shown: 991 closed tcp ports (reset)
PORT     STATE    SERVICE     VERSION
22/tcp   filtered ssh
23/tcp   filtered telnet
53/tcp   open     domain      dnsmasq 2.83
80/tcp   open     http        Xfinity Broadband Router Server
111/tcp  filtered rpcbind
443/tcp  open     ssl/https   Xfinity Broadband Router Server
8080/tcp filtered http-proxy
8181/tcp filtered intermapper
9000/tcp filtered cslistener
2 services unrecognized despite returning data. If you know the service/version, please submit the following fingerprints at https://nmap.org/cgi-bin/submit.cgi?new-service :
==============NEXT SERVICE FINGERPRINT (SUBMIT INDIVIDUALLY)==============

... (not included)

MAC Address: C4:4F:D5:B9:BA:EB (Unknown)
Device type: general purpose|router
Running: Linux 5.X, MikroTik RouterOS 7.X
OS CPE: cpe:/o:linux:linux_kernel:5 cpe:/o:mikrotik:routeros:7 cpe:/o:linux:linux_kernel:5.6.3
OS details: Linux 5.0 - 5.14, MikroTik RouterOS 7.2 - 7.5 (Linux 5.6.3)
Network Distance: 1 hop

Nmap scan report for MyLink (10.1.10.68)
Host is up (0.0019s latency).
Not shown: 996 closed tcp ports (reset)
PORT      STATE SERVICE    VERSION
22/tcp    open  ssh        Dropbear sshd (protocol 2.0)
80/tcp    open  http       lighttpd 1.4.54
443/tcp   open  ssl/http   lighttpd 1.4.54
10001/tcp open  tcpwrapped
MAC Address: 0C:EA:14:90:54:28 (Unknown)
Device type: general purpose
Running: Linux 2.6.X
OS CPE: cpe:/o:linux:linux_kernel:2.6.32
OS details: Linux 2.6.32
Network Distance: 1 hop
Service Info: OS: Linux; CPE: cpe:/o:linux:linux_kernel

Nmap scan report for MyStation (10.1.10.98)
Host is up (0.0049s latency).
Not shown: 996 closed tcp ports (reset)
PORT      STATE SERVICE    VERSION
22/tcp    open  ssh        Dropbear sshd (protocol 2.0)
80/tcp    open  http       lighttpd 1.4.54
443/tcp   open  ssl/http   lighttpd 1.4.54
10001/tcp open  tcpwrapped
MAC Address: 0C:EA:14:90:54:46 (Unknown)
Device type: general purpose
Running: Linux 2.6.X
OS CPE: cpe:/o:linux:linux_kernel:2.6.32
OS details: Linux 2.6.32
Network Distance: 1 hop
Service Info: OS: Linux; CPE: cpe:/o:linux:linux_kernel

Nmap scan report for teton (10.1.10.159)
Host is up (0.057s latency).
Not shown: 994 filtered tcp ports (no-response)
PORT      STATE  SERVICE         VERSION
80/tcp    closed http
5432/tcp  open   postgresql      PostgreSQL DB 9.6.0 or later
8081/tcp  closed blackice-icecap
8083/tcp  open   http            Cowboy httpd
8084/tcp  open   ssl/http        Cowboy httpd
24800/tcp closed unknown
MAC Address: 28:D0:43:F6:C2:60 (AzureWave Technology)
Aggressive OS guesses: Linux 4.15 - 5.19 (94%), Linux 5.0 - 5.14 (93%), OpenWrt 22.03 (Linux 5.10) (93%), MikroTik RouterOS 7.2 - 7.5 (Linux 5.6.3) (93%), Linux 5.0 (92%), Linux 3.10 - 4.11 (91%), Android 12 (Linux 5.4) (90%), Linux 3.2 - 4.14 (90%), Android 10 - 12 (Linux 4.14 - 4.19) (90%), OpenWrt 21.02 (Linux 5.4) (90%)
No exact OS matches for host (test conditions non-ideal).
Network Distance: 1 hop

Nmap scan report for denali (10.1.10.150)
Host is up (0.000033s latency).
Not shown: 995 closed tcp ports (reset)
PORT     STATE    SERVICE    VERSION
22/tcp   open     ssh        OpenSSH 10.0p2 Ubuntu 5ubuntu5 (Ubuntu Linux; protocol 2.0)
3128/tcp open     http-proxy Squid http proxy 6.13
5051/tcp filtered ida-agent
5432/tcp filtered postgresql
8080/tcp filtered http-proxy
Device type: general purpose
Running: Linux 5.X|6.X
OS CPE: cpe:/o:linux:linux_kernel:5 cpe:/o:linux:linux_kernel:6
OS details: Linux 5.0 - 6.2
Network Distance: 0 hops
Service Info: OS: Linux; CPE: cpe:/o:linux:linux_kernel

OS and Service detection performed. Please report any incorrect results at https://nmap.org/submit/ .
Nmap done: 256 IP addresses (5 hosts up) scanned in 199.09 seconds

"""