<!-- neira:meta
id: NEI-20260101-navigation-guide
intent: docs
summary: Инструкция по скрытию навигационного бара в панели сознания.
-->

# Руководство по скрытию навигационной панели

## Обзор
Навигационная панель (navigation bar) расположена в верхней части дашборда сознания Нейры, сразу после header. Она содержит ссылки на разделы: Главная, Органы, Личность, История, Настройки.

Файлы, которые содержат навигацию:
- `static/consciousness/index.html` — HTML-разметка
- `static/consciousness/dashboard.css` — стили навигации
- `static/consciousness/dashboard.js` — переводы и обработчики
- `spinal_cord/static/consciousness/` — синхронизированные копии

## Метод 1: Скрытие через CSS (простой)

### Полное скрытие
Добавить в `dashboard.css`:

```css
/* Скрыть навигационную панель полностью */
nav[role="navigation"] {
    display: none !important;
}
```

### Скрытие на мобильных устройствах
```css
/* Скрыть навигацию только на экранах меньше 768px */
@media (max-width: 768px) {
    nav[role="navigation"] {
        display: none;
    }
}
```

### Временное скрытие (с возможностью показать)
```css
/* Скрыть по умолчанию, но можно показать классом .visible */
nav[role="navigation"] {
    display: none;
}

nav[role="navigation"].visible {
    display: block !important;
}
```

## Метод 2: Динамическое управление через JavaScript

### Простой переключатель (toggle)
Добавить в `dashboard.js` в раздел `setupEventListeners()`:

```javascript
// Добавить кнопку переключения навигации
const toggleNavBtn = document.createElement('button');
toggleNavBtn.id = 'toggleNav';
toggleNavBtn.textContent = '☰';
toggleNavBtn.className = 'fixed bottom-4 right-4 bg-purple-600 text-white p-3 rounded-full shadow-lg hover:bg-purple-700';
toggleNavBtn.setAttribute('aria-label', 'Показать/скрыть навигацию');
document.body.appendChild(toggleNavBtn);

// Обработчик клика
toggleNavBtn.addEventListener('click', () => {
    const nav = document.querySelector('nav[role="navigation"]');
    nav.classList.toggle('hidden'); // Tailwind класс
});
```

### Управление с сохранением состояния в localStorage
```javascript
// Инициализация состояния навигации
function initNavigation() {
    const navVisible = localStorage.getItem('neira_nav_visible') !== 'false'; // по умолчанию видна
    const nav = document.querySelector('nav[role="navigation"]');
    
    if (!navVisible) {
        nav.classList.add('hidden');
    }
    
    // Создать кнопку переключения
    const toggleBtn = document.createElement('button');
    toggleBtn.id = 'toggleNav';
    toggleBtn.innerHTML = navVisible ? '⬆ Скрыть навигацию' : '⬇ Показать навигацию';
    toggleBtn.className = 'fixed bottom-4 right-4 bg-purple-600 text-white px-4 py-2 rounded-lg shadow-lg hover:bg-purple-700 transition-all';
    document.body.appendChild(toggleBtn);
    
    // Обработчик переключения
    toggleBtn.addEventListener('click', () => {
        const isVisible = !nav.classList.contains('hidden');
        nav.classList.toggle('hidden');
        localStorage.setItem('neira_nav_visible', !isVisible);
        toggleBtn.innerHTML = !isVisible ? '⬆ Скрыть навигацию' : '⬇ Показать навигацию';
    });
}

// Вызвать в DOMContentLoaded
document.addEventListener('DOMContentLoaded', () => {
    initNavigation();
    // ... остальная инициализация
});
```

## Метод 3: Удаление из HTML (радикальный)

### Полное удаление навигации
Открыть `index.html` и удалить блок:

```html
<!-- Navigation Bar -->
<nav class="sticky top-0 z-50 bg-gray-800 shadow-lg border-b border-purple-500" role="navigation" aria-label="Основная навигация">
    <!-- ... весь блок навигации ... -->
</nav>
```

**Важно:** При удалении из HTML также нужно:
1. Убрать стили навигации из `dashboard.css` (секция `/* Navigation Bar Styles */`)
2. Убрать переводы `nav: {...}` из `dashboard.js` (в обоих языковых блоках)
3. Убрать обновление навигации в функции `updateUI()` (блок `// Navigation links`)
4. Синхронизировать изменения в `spinal_cord/static/consciousness/`

## Метод 4: Условное отображение по настройке пользователя

### Добавить чекбокс в настройки
В разделе настроек (если существует) или в панели quick actions:

```html
<label class="flex items-center space-x-2 cursor-pointer">
    <input type="checkbox" id="showNavigation" checked class="form-checkbox">
    <span>Показывать навигационную панель</span>
</label>
```

JavaScript обработчик:
```javascript
const navCheckbox = document.getElementById('showNavigation');
const nav = document.querySelector('nav[role="navigation"]');

// Загрузить настройку
const navEnabled = localStorage.getItem('neira_nav_enabled') !== 'false';
navCheckbox.checked = navEnabled;
if (!navEnabled) nav.classList.add('hidden');

// Сохранять изменения
navCheckbox.addEventListener('change', (e) => {
    const enabled = e.target.checked;
    localStorage.setItem('neira_nav_enabled', enabled);
    nav.classList.toggle('hidden', !enabled);
});
```

## Рекомендации по выбору метода

| Метод | Когда использовать | Сложность | Обратимость |
|-------|-------------------|-----------|-------------|
| CSS | Быстрое постоянное скрытие | ⭐ Простая | ✅ Легко |
| JavaScript toggle | Нужна возможность показать/скрыть | ⭐⭐ Средняя | ✅ Легко |
| Удаление HTML | Навигация точно не нужна | ⭐⭐⭐ Высокая | ❌ Сложно (нужен git revert) |
| Настройка пользователя | Разным пользователям нужны разные режимы | ⭐⭐⭐ Высокая | ✅ Очень легко |

## Примеры использования

### Пример 1: Скрыть навигацию в компактном режиме
```css
/* В dashboard.css */
body.compact-mode nav[role="navigation"] {
    display: none;
}
```

```javascript
// В dashboard.js, добавить переключатель
function toggleCompactMode() {
    document.body.classList.toggle('compact-mode');
    localStorage.setItem('neira_compact', document.body.classList.contains('compact-mode'));
}
```

### Пример 2: Показывать навигацию только при наведении
```css
/* В dashboard.css */
nav[role="navigation"] {
    transform: translateY(-100%);
    transition: transform 0.3s ease;
}

nav[role="navigation"]:hover,
nav[role="navigation"]:focus-within {
    transform: translateY(0);
}
```

### Пример 3: Скрыть отдельные пункты навигации
```javascript
// Скрыть только "История" и "Настройки"
document.querySelectorAll('.nav-history, .nav-settings').forEach(el => {
    el.closest('li').style.display = 'none';
});
```

## Проверка изменений

После применения любого метода:

1. Обновить страницу в браузере (`Ctrl+Shift+R` для жёсткой перезагрузки)
2. Проверить в DevTools (F12):
   - Вкладка Elements: убедиться, что `nav[role="navigation"]` имеет нужные стили
   - Вкладка Console: проверить отсутствие ошибок JavaScript
3. Проверить на разных размерах экрана (responsive design)
4. Проверить accessibility: если навигация скрыта, экранный диктор не должен её озвучивать

## Откат изменений

### Если изменения были в CSS:
Удалить добавленные правила или закомментировать их `/* ... */`

### Если изменения были в JavaScript:
1. Найти добавленный код по комментарию или ID элемента
2. Удалить или закомментировать блок `// ...`

### Если навигация была удалена из HTML:
```bash
# В корне репозитория
git restore static/consciousness/index.html
git restore spinal_cord/static/consciousness/index.html
```

## Связанные файлы

- `static/consciousness/` — основная директория фронтенда
- `spinal_cord/static/consciousness/` — копия для backend
- `AGENTS.md` — правила работы с репозиторием
- `CODING_GUIDELINES.md` — стандарты кода

## Changelog

- **2026-01-01**: Создана инструкция после добавления навигационного бара в dashboard
