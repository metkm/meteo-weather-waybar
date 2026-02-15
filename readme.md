```json
"custom/weather": {
  "format": "{icon} {text} °C",
  "format-icons": {
    "day": "",
    "night": "",
    "default": "🎵"
  },
  "interval": 3600,
  "return-type": "json",
  "exec": "~/.config/waybar/custom/meteo-weather-waybar \"https://api.open-meteo.com/v1/forecast?latitude=40.8982&longitude=29&current_weather=true\""
},
```
