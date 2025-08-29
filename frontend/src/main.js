/* neira:meta
id: NEI-20250829-180333-frontend-main
intent: docs
summary: |
  Пример простого PWA с формой и обработкой сообщений.
*/

/* global navigator, window, document, fetch, console */
import './style.css';

console.log('Hello PWA');

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('/service-worker.js').catch(err => {
      console.error('Service worker registration failed', err);
    });
  });
}

const app = document.getElementById('app');
const messages = document.createElement('div');
messages.id = 'messages';
app.appendChild(messages);

const form = document.createElement('form');
const input = document.createElement('input');
input.type = 'text';
input.placeholder = 'Спросите Нейру...';
input.required = true;
const button = document.createElement('button');
button.type = 'submit';
button.textContent = 'Отправить';
form.appendChild(input);
form.appendChild(button);
app.appendChild(form);

form.addEventListener('submit', async e => {
  e.preventDefault();
  const text = input.value.trim();
  if (!text) return;
  addMessage('user', text);
  input.value = '';
  try {
    const res = await fetch('/api/neira/interact', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message: text })
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();
    addMessage('bot', data.reply ?? JSON.stringify(data));
  } catch (err) {
    addMessage('error', `Ошибка: ${err.message}`);
  }
});

function addMessage(role, text) {
  const div = document.createElement('div');
  div.className = `msg ${role}`;
  div.textContent = text;
  messages.appendChild(div);
  messages.scrollTop = messages.scrollHeight;
}
