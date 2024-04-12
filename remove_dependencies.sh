# Ubuntu
sudo apt remove mold clang
doas apt remove mold clang
sudo apt remove libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 librust-alsa-sys-dev librust-libudev-sys-dev libwayland-dev libxkbcommon-dev
doas apt remove libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 librust-alsa-sys-dev librust-libudev-sys-dev libwayland-dev libxkbcommon-dev

# Fedora

## dnf5
sudo dnf5 remove mold clang
doas dnf5 remove mold clang
sudo dnf5 remove libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel
doas dnf5 remove libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel

## dnf4
sudo dnf remove mold clang
doas dnf remove mold clang
sudo dnf remove libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel
doas dnf remove libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel