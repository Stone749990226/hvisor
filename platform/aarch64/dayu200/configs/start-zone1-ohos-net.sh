# dayu200 OpenHarmony zone0 does not currently support the old bridge commands:
# no ip/brctl/dhclient in system/bin, and CONFIG_BRIDGE is disabled.
# Use TAP + NAT with static addresses instead.

# zone0/root OpenHarmony: create tap0 and NAT it through the real NIC.
# U-Boot TFTP uses board 192.168.1.20 <-> host 192.168.1.10, but OpenHarmony
# should configure the runtime NIC address again after boot.

echo "4 4 1 7" > /proc/sys/kernel/printk
cd /data/zone
mount -t proc proc /proc
mount -t sysfs sysfs /sys
ifconfig -a
ifconfig eth0 192.168.1.20 netmask 255.255.255.0 up
ping 192.168.1.10
mkdir -p /dev/net
mknod /dev/net/tun c 10 200
/data/zone/busybox tunctl -d tap0
tunctl -d tap0
tunctl -d -T tap0
/data/zone/busybox tunctl -t tap0
cat /sys/class/net/tap0/type
/data/zone/busybox ifconfig tap0 192.168.200.1 netmask 255.255.255.0 up
sysctl -w net.ipv4.ip_forward=1
netstat -rn
iptables -t nat -D POSTROUTING -s 192.168.200.0/24 -o eth0 -j MASQUERADE
iptables -t nat -A POSTROUTING -s 192.168.200.0/24 -o eth0 -j MASQUERADE
chmod 777 hvisor hvisor.ko
insmod hvisor.ko

# Create the virtio-blk backend image once.
# [ -e virtio-blk.img ] || dd if=/dev/zero of=virtio-blk.img bs=1M count=64

mkdir -p /dev/pts
mount -t devpts devpts /dev/pts

nohup ./hvisor virtio start zone1-ohos-virtio.json &
sleep 2
./hvisor zone start zone1-ohos.json

# zone1/non-root OpenHarmony: configure the virtio-net interface.
echo "4 4 1 7" > /proc/sys/kernel/printk
mount -t proc proc /proc
mount -t sysfs sysfs /sys
ifconfig eth0 192.168.200.2 netmask 255.255.255.0 up
/data/zone/busybox route add default gw 192.168.200.1 dev eth0
ping 192.168.200.1
ping 192.168.1.10

# Full Internet access from zone1 also needs a default route.
# The current OpenHarmony system/bin lacks ip/route, and the local busybox config
# also has CONFIG_IP/CONFIG_ROUTE disabled. Rebuild busybox with ip or route
# enabled, then add a default gateway via 192.168.200.1.
