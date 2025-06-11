# Парсер логов веб-сервера
## Описание
- Программа для анализа логов веб-сервера с целью подсчета количества уникальных IP-адресов и определения используемых операционных систем(учебный проект).

## Функциональность
- Парсинг логов:
- Извлечение IP-адресов
- Определение ОС посетителя
- Подсчет количества обращений
### Поддерживаемые ОС:
- Linux
- Windows
- Mac OS
- Другие ОС

## Требования
- Rust версии 1.60 или выше
- Cargo для управления зависимостями

## Структура 

* `log-parser/`;
* `├── src`;
* `│   └── main.rs`;
* `├── Cargo.toml`;
* `└── README.md`;


## Пример лога
```sh
128.148.46.126 - - [20/Jun/2024:12:56:06 +0300] "GET /forecast-system%20engine_high-level.css HTTP/1.1" 200 1359 "-" "Mozilla/5.0 (Windows NT 5.0; en-US; rv:1.9.2.20) Gecko/1990-13-08 Firefox/37.0"
52.56.245.28 - - [20/Jun/2024:12:56:06 +0300] "GET /archive/optimizing_zero%20defect.jpg HTTP/1.1" 200 2551 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/5321 (KHTML, like Gecko) Chrome/40.0.848.0 Mobile Safari/5321"
122.100.81.4 - - [20/Jun/2024:12:56:06 +0300] "POST /internet%20solution-Monitored%20Graphical%20User%20Interface/client-driven.png HTTP/1.1" 400 88 "-" "Mozilla/5.0 (Macintosh; PPC Mac OS X 10_9_10 rv:6.0) Gecko/1957-01-07 Firefox/37.0"
236.162.72.4 - - [20/Jun/2024:12:56:06 +0300] "GET /Managed-motivating-migration.js HTTP/1.1" 200 1253 "-" "Mozilla/5.0 (Windows NT 6.2) AppleWebKit/5361 (KHTML, like Gecko) Chrome/39.0.874.0 Mobile Safari/5361"
222.71.149.234 - - [20/Jun/2024:12:56:06 +0300] "GET /Ameliorated/multi-state-Fully-configurable.svg HTTP/1.1" 200 1848 "-" "Mozilla/5.0 (Windows CE) AppleWebKit/5362 (KHTML, like Gecko) Chrome/39.0.860.0 Mobile Safari/5362"
153.172.20.53 - - [20/Jun/2024:12:56:06 +0300] "GET /object-oriented.hmtl HTTP/1.1" 200 2511 "-" "Mozilla/5.0 (X11; Linux i686; rv:6.0) Gecko/1902-21-08 Firefox/36.0"
218.233.62.252 - - [20/Jun/2024:12:56:06 +0300] "GET /exuding/Re-contextualized.gif HTTP/1.1" 200 1399 "-" "Mozilla/5.0 (Macintosh; U; PPC Mac OS X 10_9_4 rv:5.0) Gecko/1983-23-03 Firefox/36.0"
```
## Пример вывода
```sh
3.147.68.159:Macintosh:1
217.40.255.56:Linux:1
44.199.12.43:Linux:1
176.56.250.180:Windows:1
125.33.143.225:Windows:1
121.3.150.56:Linux:1
```


