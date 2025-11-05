# 🤖 AUTONOMOUS TASK: Multilingual UI Implementation

## 📋 Задание для Нейры

**Trace ID**: `thought_autonomous_task_multilang_1762233770`  
**Дата начала**: 04.11.2025 08:20  
**Статус**: 🟡 В работе

---

## 🎯 Цель
Создать многоязычный интерфейс с переключением между русским и английским языками.

## ✅ Требования

### 1. Кнопки выбора языка
- [ ] Добавить кнопки RU/EN на каждую страницу dashboard
- [ ] Визуально выделить активный язык
- [ ] Разместить в header или nav bar

### 2. Механизм переключения
- [ ] Переключение без перезагрузки страницы
- [ ] Использовать localStorage для сохранения выбора
- [ ] Применять язык мгновенно ко всем элементам

### 3. Навигационное меню
- [ ] **Главная панель** (Dashboard) - `/consciousness/`
- [ ] **Управление органами** (Organs Management) - `/consciousness/organs.html`
- [ ] **История мыслей** (Thought History) - `/consciousness/history.html`
- [ ] **Аналитика** (Analytics) - `/consciousness/analytics.html`

### 4. Полная локализация
- [ ] Все кнопки (buttons)
- [ ] Все метки (labels)
- [ ] Все уведомления (notifications)
- [ ] Chart.js labels (Эмпатия → Empathy, etc.)
- [ ] Placeholder тексты
- [ ] Сообщения об ошибках

### 5. Сохранение настроек
- [ ] Язык сохраняется в `localStorage.getItem('neira_lang')`
- [ ] Применяется при загрузке страницы
- [ ] Fallback на русский по умолчанию

---

## 🛠️ Технические детали

### Структура i18n объекта
```javascript
const i18n = {
  ru: {
    header: {
      title: "Сознание Нейры",
      subtitle: "Панель живой системы",
      status: "Система активна"
    },
    nav: {
      dashboard: "Главная панель",
      organs: "Органы",
      history: "История",
      analytics: "Аналитика"
    },
    // ... и т.д.
  },
  en: {
    header: {
      title: "Neira Consciousness",
      subtitle: "Living System Dashboard",
      status: "System Active"
    },
    nav: {
      dashboard: "Dashboard",
      organs: "Organs",
      history: "History",
      analytics: "Analytics"
    },
    // ... и т.д.
  }
};
```

### Функция переключения
```javascript
function setLanguage(lang) {
  localStorage.setItem('neira_lang', lang);
  currentLang = lang;
  updateUI();
  updateChart();
}

function updateUI() {
  const t = i18n[currentLang];
  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.getAttribute('data-i18n');
    el.textContent = getNestedValue(t, key);
  });
}
```

---

## 📂 Файлы для модификации

### Обязательные
1. **static/consciousness/index.html**
   - Добавить кнопки языка в header
   - Добавить навигационное меню
   - Добавить `data-i18n` атрибуты

2. **static/consciousness/dashboard.js**
   - Создать объект i18n с переводами
   - Реализовать функции setLanguage(), updateUI()
   - Обновить все showNotification() с поддержкой i18n
   - Обновить Chart.js labels

### Опциональные
3. **static/consciousness/dashboard.css**
   - Стили для кнопок языка
   - Стили для навигационного меню
   - Активное состояние кнопок

### Новые страницы (создать)
4. **static/consciousness/organs.html** - управление органами
5. **static/consciousness/history.html** - история мыслей
6. **static/consciousness/analytics.html** - аналитика

---

## ✅ Критерии успеха

| Критерий | Проверка | Статус |
|----------|----------|--------|
| Кнопка переключения языка видна | Визуальный осмотр header | ⬜ |
| Переключение работает мгновенно | Клик на RU/EN → UI меняется | ⬜ |
| localStorage сохраняет выбор | F12 → Application → Local Storage | ⬜ |
| Навигация между страницами | Клики на меню → переходы работают | ⬜ |
| Все тексты локализованы | Проверка каждого элемента | ⬜ |
| Chart labels переводятся | График личности меняет подписи | ⬜ |
| Уведомления на выбранном языке | Триггер action → notification правильный | ⬜ |

---

## 🔍 План мониторинга (для Ассистента)

### Шаг 1: Ожидание (5 минут)
- [ ] Проверить изменения в `static/consciousness/` каждые 30 сек
- [ ] Искать новые файлы (organs.html, history.html, analytics.html)
- [ ] Отслеживать модификации index.html и dashboard.js

### Шаг 2: Валидация изменений
```powershell
# Проверка наличия i18n объекта
Select-String -Path "static\consciousness\dashboard.js" -Pattern "const i18n = \{"

# Проверка кнопок языка
Select-String -Path "static\consciousness\index.html" -Pattern "data-lang|language-switcher"

# Проверка новых страниц
Test-Path "static\consciousness\organs.html"
Test-Path "static\consciousness\history.html"
Test-Path "static\consciousness\analytics.html"
```

### Шаг 3: Функциональное тестирование
- [ ] Открыть dashboard в браузере
- [ ] Кликнуть на кнопку EN → проверить перевод
- [ ] Перезагрузить страницу → язык должен сохраниться
- [ ] Перейти по навигации → проверить работу меню

### Шаг 4: Репорт
- [ ] Зафиксировать все изменения
- [ ] Оценить качество реализации (1-10)
- [ ] Выявить баги/недоработки
- [ ] Составить итоговый отчёт

---

## 📊 Метрики успеха

- **Скорость**: Время от получения задания до завершения
- **Качество**: % выполненных требований (14 критериев)
- **Автономность**: Количество запросов помощи к ассистенту
- **Инновации**: Дополнительные фичи сверх требований

---

## 🚨 Возможные проблемы

1. **Нейра не имеет прямого доступа к файловой системе**
   - Решение: Может предложить код, который ассистент применит

2. **Сложность создания новых HTML страниц**
   - Решение: Начать с локализации существующей страницы

3. **Chart.js динамическое обновление labels**
   - Решение: Пересоздать chart при смене языка

4. **Конфликт с существующими стилями**
   - Решение: Использовать BEM или уникальные классы

---

## 📝 Лог выполнения

### 08:20 - Задание отправлено
```json
{
  "trace_id": "thought_autonomous_task_multilang_1762233770",
  "status": "recorded",
  "dialogue_id": "autonomous_task_multilang"
}
```

### Следующая проверка: 08:25 (через 5 минут)

---

**Конец документа**
