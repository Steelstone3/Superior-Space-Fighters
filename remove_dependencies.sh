# Ubuntu
sudo apt remove libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 librust-alsa-sys-dev librust-libudev-sys-dev libwayland-dev libxkbcommon-dev
doas apt remove libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 librust-alsa-sys-dev librust-libudev-sys-dev libwayland-dev libxkbcommon-dev
sudo apt remove lld clang

# Fedora
sudo dnf5 remove libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel
doas dnf5 remove libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel
sudo dnf remove lld clang