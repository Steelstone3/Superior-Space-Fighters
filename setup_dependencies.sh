# Ubuntu
sudo apt install --fix-missing g++ pkg-config
doas apt install --fix-missing g++ pkg-config
sudo apt install --fix-missing libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 librust-alsa-sys-dev librust-libudev-sys-dev libwayland-dev libxkbcommon-dev
doas apt install --fix-missing libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 librust-alsa-sys-dev librust-libudev-sys-dev libwayland-dev libxkbcommon-dev
sudo apt-get install lld clang

# Fedora
sudo dnf install gcc-c++ 
doas dnf install gcc-c++ 
sudo dnf5 install libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel
doas dnf5 install libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel
sudo dnf install lld clang