# SoundEngine

Небольшой движок для воспроизведения звуковых эффектов.

## Конфигурация

Файл `config/audio.yaml` управляет режимом работы:

```yaml
mode: assets  # assets | generation | hybrid
```

- `assets` – воспроизводятся только файлы из каталога `assets/audio/`.
- `generation` – все эффекты запрашиваются у асинхронного генератора.
- `hybrid` – сначала ищется файл, при отсутствии используется генерация.

## Использование в игровом режиме

```python
from ui.master_screen import MasterScreen

screen = MasterScreen()
# Проиграть эффект c половинной громкостью
await screen.sound.play("dice-roll", volume=0.5, loops=0, channel="sfx")
```

Метод `play` принимает:

- `effect_id` – идентификатор эффекта (имя файла без расширения);
- `volume` – громкость от 0 до 1;
- `loops` – количество повторов;
- `channel` – произвольный канал для группировки звука.

Асинхронный генератор можно передать через параметр `generative_service`
при создании `SoundEngine`.
