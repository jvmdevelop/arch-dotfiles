#!/bin/bash
class=$(hyprctl activewindow -j | jq -r '.class')
hyprctl dispatch togglefloating

if [ "$class" = "telegram-desktop" ]; then
    floating=$(hyprctl activewindow -j | jq -r '.floating')
    if [ "$floating" = "true" ]; then
        eval "$(hyprctl monitors -j | jq -r '.[] | select(.focused) |
            "W=\(.width / .scale | floor) H=\(.height / .scale | floor) MX=\(.x) MY=\(.y)"')"
        WIN_W=$((W / 4))
        WIN_X=$((MX + W - WIN_W))
        sleep 0.1
        hyprctl dispatch resizeactive exact "${WIN_W}" "${H}"
        hyprctl dispatch moveactive exact "${WIN_X}" "${MY}"
    fi
fi
