// Neira Consciousness Dashboard JavaScript

const API_BASE = window.location.origin;
const API_CONSCIOUSNESS = `${API_BASE}/api/neira/consciousness`;
const API_ORGANS = `${API_BASE}/organs`;

// ========== INTERNATIONALIZATION (i18n) ==========
const i18n = {
    ru: {
        header: {
            title: "Сознание Нейры",
            subtitle: "Панель живой системы",
            status: "Всё в порядке"  // было: "Система активна" - калька с "System Active"
        },
        stats: {
            thoughts: "Мыслей в базе",  // было: "Записано мыслей" - неестественный порядок
            biases: "Найдено предубеждений",  // было: "Выявлено..." - слишком формально
            organs: "Органов работает",  // было: "Активных органов" - калька
            tasks: "Задач улучшения"  // было: "Задач на улучшение" - лишнее слово
        },
        sections: {
            personality: "Развитие личности",  // было: "Эволюция..." - калька с "Evolution"
            organs: "Рост органов",  // было: "Монитор роста органов" - избыточно
            thought: "Записать мысль",
            report: "Отчёт о развитии",  // было: "...о росте" - менее естественно
            actions: "Быстрые действия"
        },
        buttons: {
            createOrgan: "Создать орган",
            recordThought: "Записать мысль",
            generateReport: "Составить отчёт",  // было: "Сгенерировать..." - калька с "Generate"
            snapshot: "Снимок личности",  // было: "Создать снимок..." - короче
            refresh: "Обновить"  // было: "Обновить данные" - избыточно
        },
        placeholders: {
            thought: "О чём думаешь?.."  // было: "Опишите процесс рассуждения..." - слишком формально
        },
        chart: {
            labels: ['Эмпатия', 'Любопытство', 'Терпение', 'Творчество', 'Адаптивность', 'Настойчивость'],
            dataset: 'Личность сейчас'  // было: "Текущая личность" - калька
        },
        notifications: {
            refreshing: "Обновляю...",  // было: "Обновление данных..." - избыточно
            thoughtEmpty: "Напиши хоть что-нибудь",  // было: "Пожалуйста, введите мысль" - слишком формально
            thoughtRecorded: "Записано",  // было: "Мысль записана" - короче
            thoughtFailed: "Не вышло записать",  // было: "Не удалось..." - калька
            reportGenerating: "Готовлю отчёт...",  // было: "Генерация отчёта..." - калька
            reportSuccess: "Готово!",  // было: "Отчёт успешно сгенерирован!" - избыточно
            reportFailed: "Не получилось",  // было: "Не удалось сгенерировать отчёт" - длинно
            snapshotCreated: "Снимок сохранён",  // было: "Снимок создан" - точнее
            snapshotFailed: "Ошибка снимка",  // было: "Не удалось создать снимок" - короче
            organCreated: "Орган создан",
            organFailed: "Не удалось создать орган"
        },
        messages: {
            noOrgans: "Органов пока нет. Создай первый!",  // было: "Создайте" - менее формально
            loadingOrgans: "Загружаю органы...",  // было: "Загрузка органов..." - калька
            organsError: "Не удалось загрузить",  // было: "Ошибка загрузки органов" - короче
            reportPrompt: 'Нажми «Составить отчёт» для анализа',  // было: "Сгенерировать отчёт" + длинно
            organPrompt: "Имя нового органа:"  // было: "Введите имя органа:" - короче
        },
        periods: {
            "1": "сутки",  // было: "24 часа" - естественнее
            "7": "неделя",  // было: "7 дней" - естественнее
            "30": "месяц"  // было: "30 дней" - естественнее
        },
        footer: "Сознание Нейры v1.0 | Живая система | Обновлено:",  // было: "Панель сознания..." - избыточно
        nav: {
            home: "Главная",
            organs: "Органы",
            personality: "Личность",
            history: "История",
            settings: "Настройки"
        }
    },
    en: {
        header: {
            title: "Neira Consciousness",
            subtitle: "Living System Dashboard",
            status: "System Active"
        },
        stats: {
            thoughts: "Thoughts Recorded",
            biases: "Biases Detected",
            organs: "Active Organs",
            tasks: "Improvement Tasks"
        },
        sections: {
            personality: "Personality Evolution",
            organs: "Organ Growth Monitor",
            thought: "Record Thought",
            report: "Growth Report",
            actions: "Quick Actions"
        },
        buttons: {
            createOrgan: "Create Organ",
            recordThought: "Record Thought",
            generateReport: "Generate Report",
            snapshot: "Create Personality Snapshot",
            refresh: "Refresh Data"
        },
        placeholders: {
            thought: "Describe your reasoning process..."
        },
        chart: {
            labels: ['Empathy', 'Curiosity', 'Patience', 'Creativity', 'Adaptability', 'Assertiveness'],
            dataset: 'Current Personality'
        },
        notifications: {
            refreshing: "Refreshing data...",
            thoughtEmpty: "Please enter a thought",
            thoughtRecorded: "Thought recorded",
            thoughtFailed: "Failed to record thought",
            reportGenerating: "Generating report...",
            reportSuccess: "Report generated successfully!",
            reportFailed: "Failed to generate report",
            snapshotCreated: "Snapshot created",
            snapshotFailed: "Failed to create snapshot",
            organCreated: "Organ created",
            organFailed: "Failed to create organ"
        },
        messages: {
            noOrgans: "No organs yet. Create your first one!",
            loadingOrgans: "Loading organs...",
            organsError: "Failed to load organs",
            reportPrompt: 'Click "Generate Report" to see growth analysis',
            organPrompt: "Enter organ name:"
        },
        periods: {
            "1": "24 hours",
            "7": "7 days",
            "30": "30 days"
        },
        footer: "Neira Consciousness Dashboard v1.0 | Living System | Last update:",
        nav: {
            home: "Home",
            organs: "Organs",
            personality: "Personality",
            history: "History",
            settings: "Settings"
        }
    }
};

// Current language state
let currentLang = localStorage.getItem('neira_lang') || 'ru';

// Helper function to get translation
function t(key) {
    const keys = key.split('.');
    let value = i18n[currentLang];
    for (const k of keys) {
        value = value?.[k];
    }
    return value || key;
}

// Global state
let personalityChart = null;
let refreshInterval = null;

// Initialize dashboard
document.addEventListener('DOMContentLoaded', () => {
    console.log('🧠 Neira Consciousness Dashboard initializing...');
    
    initializeLanguage();
    initializeChart();
    setupEventListeners();
    loadAllData();
    
    // Auto-refresh every 5 seconds
    refreshInterval = setInterval(loadAllData, 5000);
});

// Initialize language
function initializeLanguage() {
    updateUI();
    // Update language buttons state
    document.querySelectorAll('[data-lang]').forEach(btn => {
        btn.classList.toggle('active', btn.getAttribute('data-lang') === currentLang);
    });
}

// Initialize personality chart
function initializeChart() {
    const ctx = document.getElementById('personalityChart').getContext('2d');
    personalityChart = new Chart(ctx, {
        type: 'radar',
        data: {
            labels: ['Эмпатия', 'Любопытство', 'Терпение', 'Творчество', 'Адаптивность', 'Настойчивость'],
            datasets: [{
                label: 'Текущая личность',
                data: [0.7, 0.8, 0.6, 0.75, 0.85, 0.5],
                backgroundColor: 'rgba(139, 92, 246, 0.2)',
                borderColor: 'rgba(139, 92, 246, 1)',
                borderWidth: 2,
                pointBackgroundColor: 'rgba(139, 92, 246, 1)',
                pointBorderColor: '#fff',
                pointHoverBackgroundColor: '#fff',
                pointHoverBorderColor: 'rgba(139, 92, 246, 1)'
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                r: {
                    beginAtZero: true,
                    max: 1,
                    ticks: {
                        stepSize: 0.2,
                        color: '#9ca3af'
                    },
                    grid: {
                        color: '#374151'
                    },
                    pointLabels: {
                        color: '#d1d5db',
                        font: {
                            size: 12
                        }
                    }
                }
            },
            plugins: {
                legend: {
                    display: false
                }
            }
        }
    });
}

// Setup event listeners
function setupEventListeners() {
    document.getElementById('recordThoughtBtn').addEventListener('click', recordThought);
    document.getElementById('generateReportBtn').addEventListener('click', generateReport);
    document.getElementById('snapshotBtn').addEventListener('click', createSnapshot);
    document.getElementById('refreshBtn').addEventListener('click', () => {
        showNotification(t('notifications.refreshing'), 'info');
        loadAllData();
    });
    document.getElementById('createOrganBtn').addEventListener('click', createOrgan);
    
    // Language switcher buttons
    document.querySelectorAll('[data-lang]').forEach(btn => {
        btn.addEventListener('click', (e) => {
            const lang = e.target.getAttribute('data-lang');
            setLanguage(lang);
        });
    });
}

// Set language
function setLanguage(lang) {
    if (!i18n[lang]) return;
    
    currentLang = lang;
    localStorage.setItem('neira_lang', lang);
    
    // Update UI
    updateUI();
    updateChart();
    
    // Update active button state
    document.querySelectorAll('[data-lang]').forEach(btn => {
        btn.classList.toggle('active', btn.getAttribute('data-lang') === lang);
    });
}

// Update all UI texts
function updateUI() {
    // Header
    document.querySelector('.header-title').textContent = t('header.title');
    document.querySelector('.header-subtitle').textContent = t('header.subtitle');
    document.querySelector('.status-text').textContent = t('header.status');
    
    // Stats labels
    document.querySelectorAll('.stat-label')[0].textContent = t('stats.thoughts');
    document.querySelectorAll('.stat-label')[1].textContent = t('stats.biases');
    document.querySelectorAll('.stat-label')[2].textContent = t('stats.organs');
    document.querySelectorAll('.stat-label')[3].textContent = t('stats.tasks');
    
    // Section titles
    document.querySelector('.section-personality').textContent = t('sections.personality');
    document.querySelector('.section-organs').textContent = t('sections.organs');
    document.querySelector('.section-thought').textContent = t('sections.thought');
    document.querySelector('.section-report').textContent = t('sections.report');
    document.querySelector('.section-actions').textContent = t('sections.actions');
    
    // Buttons
    document.getElementById('createOrganBtn').querySelector('span').textContent = t('buttons.createOrgan');
    document.getElementById('recordThoughtBtn').textContent = t('buttons.recordThought');
    document.getElementById('generateReportBtn').textContent = t('buttons.generateReport');
    document.getElementById('snapshotBtn').querySelector('span').textContent = t('buttons.snapshot');
    document.getElementById('refreshBtn').querySelector('span').textContent = t('buttons.refresh');
    
    // Placeholders
    document.getElementById('thoughtInput').placeholder = t('placeholders.thought');
    
    // Period selector
    document.querySelectorAll('#reportDays option').forEach((opt, idx) => {
        const val = opt.value;
        opt.textContent = t(`periods.${val}`);
    });
    
    // Footer
    const footerText = document.querySelector('footer p');
    const updateTime = document.getElementById('lastUpdate').textContent;
    footerText.innerHTML = `${t('footer')} <span id="lastUpdate">${updateTime}</span>`;
    
    // Navigation links
    const navLinks = {
        'nav-home': 'nav.home',
        'nav-organs': 'nav.organs',
        'nav-personality': 'nav.personality',
        'nav-history': 'nav.history',
        'nav-settings': 'nav.settings'
    };
    Object.entries(navLinks).forEach(([className, key]) => {
        const el = document.querySelector(`.${className}`);
        if (el) el.textContent = t(key);
    });
}

// Update chart with translated labels
function updateChart() {
    if (!personalityChart) return;
    
    personalityChart.data.labels = t('chart.labels');
    personalityChart.data.datasets[0].label = t('chart.dataset');
    personalityChart.update();
}

// Load all dashboard data
async function loadAllData() {
    try {
        await Promise.all([
            loadConsciousnessStats(),
            loadPersonality(),
            loadOrgans()
        ]);
        updateLastRefresh();
    } catch (error) {
        console.error('Error loading data:', error);
    }
}

// Load consciousness statistics
async function loadConsciousnessStats() {
    try {
        const response = await fetch(`${API_CONSCIOUSNESS}/stats`);
        if (!response.ok) throw new Error('Failed to fetch stats');
        
        const stats = await response.json();
        
        document.getElementById('thoughtsCount').textContent = stats.total_thoughts_recorded || 0;
        document.getElementById('biasesCount').textContent = stats.total_biases_detected || 0;
        document.getElementById('tasksCount').textContent = stats.total_improvement_tasks || 0;
    } catch (error) {
        console.error('Error loading consciousness stats:', error);
    }
}

// Load personality traits
async function loadPersonality() {
    try {
        const response = await fetch(`${API_CONSCIOUSNESS}/personality`);
        if (!response.ok) throw new Error('Failed to fetch personality');
        
        const data = await response.json();
        const traits = data.traits;
        
        // Update chart
        const traitValues = [
            traits.empathy?.value || 0.7,
            traits.curiosity?.value || 0.8,
            traits.patience?.value || 0.6,
            traits.creativity?.value || 0.75,
            traits.adaptability?.value || 0.85,
            traits.assertiveness?.value || 0.5
        ];
        
        personalityChart.data.datasets[0].data = traitValues;
        personalityChart.update('none');
    } catch (error) {
        console.error('Error loading personality:', error);
    }
}

// Load organs list
async function loadOrgans() {
    try {
        const response = await fetch(API_ORGANS);
        if (!response.ok) throw new Error('Failed to fetch organs');
        
        const organs = await response.json();
        document.getElementById('organsCount').textContent = organs.length || 0;
        
        const organsList = document.getElementById('organsList');
        
        if (organs.length === 0) {
            organsList.innerHTML = '<p class="text-gray-400 text-center py-8">Органов пока нет. Создайте первый!</p>';
            return;
        }
        
        organsList.innerHTML = organs.map(organ => {
            const stateColors = {
                'draft': 'bg-gray-500',
                'canary': 'bg-yellow-500',
                'experimental': 'bg-blue-500',
                'stable': 'bg-green-500',
                'failed': 'bg-red-500'
            };
            
            const stateColor = stateColors[organ.state] || 'bg-gray-500';
            
            return `
                <div class="bg-gray-700 rounded-lg p-4 hover:bg-gray-600 transition-colors">
                    <div class="flex items-center justify-between">
                        <div class="flex-1">
                            <p class="font-semibold text-white">${organ.id}</p>
                            <div class="flex items-center space-x-2 mt-1">
                                <span class="${stateColor} text-xs px-2 py-1 rounded-full uppercase font-medium">
                                    ${organ.state}
                                </span>
                            </div>
                        </div>
                        <div class="pulse-glow">
                            <svg class="w-6 h-6 text-green-400" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                        </div>
                    </div>
                </div>
            `;
        }).join('');
    } catch (error) {
        console.error('Error loading organs:', error);
        document.getElementById('organsList').innerHTML = '<p class="text-red-400 text-center py-8">Ошибка загрузки органов</p>';
    }
}

// Record thought
async function recordThought() {
    const thoughtInput = document.getElementById('thoughtInput');
    const thought = thoughtInput.value.trim();
    
    if (!thought) {
        showNotification(t('notifications.thoughtEmpty'), 'warning');
        return;
    }
    
    try {
        const response = await fetch(`${API_CONSCIOUSNESS}/thought`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                dialogue_id: `dashboard_${Date.now()}`,
                reasoning_steps: [thought],
                decisions: [{
                    step: 'User input',
                    rationale: 'Manual thought recording from dashboard',
                    confidence: 0.8,
                    alternatives_considered: []
                }]
            })
        });
        
        if (!response.ok) throw new Error('Failed to record thought');
        
        const result = await response.json();
        showNotification(`${t('notifications.thoughtRecorded')}: ${result.trace_id}`, 'success');
        thoughtInput.value = '';
        loadConsciousnessStats();
    } catch (error) {
        console.error('Error recording thought:', error);
        showNotification(t('notifications.thoughtFailed'), 'error');
    }
}

// Generate growth report
async function generateReport() {
    const days = document.getElementById('reportDays').value;
    const preview = document.getElementById('reportPreview');
    
    preview.innerHTML = `<p class="text-center text-gray-400">${t('notifications.reportGenerating')}</p>`;
    
    try {
        const response = await fetch(`${API_CONSCIOUSNESS}/growth-report?days=${days}`);
        if (!response.ok) throw new Error('Failed to generate report');
        
        const data = await response.json();
        const markdown = data.report_markdown;
        
        // Show preview (first 500 chars)
        const preview_text = markdown.substring(0, 500) + (markdown.length > 500 ? '...' : '');
        preview.innerHTML = `<pre class="whitespace-pre-wrap text-xs">${escapeHtml(preview_text)}</pre>`;
        
        showNotification(t('notifications.reportSuccess'), 'success');
    } catch (error) {
        console.error('Error generating report:', error);
        preview.innerHTML = `<p class="text-center text-red-400">${t('notifications.reportFailed')}</p>`;
        showNotification(t('notifications.reportFailed'), 'error');
    }
}

// Create personality snapshot
async function createSnapshot() {
    try {
        const response = await fetch(`${API_CONSCIOUSNESS}/personality/snapshot`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                context: `Dashboard snapshot at ${new Date().toISOString()}`
            })
        });
        
        if (!response.ok) throw new Error('Failed to create snapshot');
        
        const result = await response.json();
        showNotification(`${t('notifications.snapshotCreated')}: ${result.snapshot_id}`, 'success');
    } catch (error) {
        console.error('Error creating snapshot:', error);
        showNotification(t('notifications.snapshotFailed'), 'error');
    }
}

// Create organ
async function createOrgan() {
    const organName = prompt(t('messages.organPrompt'), 'test_organ_' + Date.now());
    if (!organName) return;
    
    try {
        const response = await fetch(`${API_ORGANS}/build`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                organ_template: {
                    name: organName,
                    type: 'analysis',
                    capabilities: ['test']
                },
                dryrun: false
            })
        });
        
        if (!response.ok) throw new Error('Failed to create organ');
        
        const result = await response.json();
        showNotification(`${t('notifications.organCreated')}: ${result.organ_id}`, 'success');
        
        // Reload organs after a short delay to see state transition
        setTimeout(loadOrgans, 1000);
    } catch (error) {
        console.error('Error creating organ:', error);
        showNotification(t('notifications.organFailed'), 'error');
    }
}

// Show notification
function showNotification(message, type = 'info') {
    const colors = {
        success: 'bg-green-500',
        error: 'bg-red-500',
        warning: 'bg-yellow-500',
        info: 'bg-blue-500'
    };
    
    const notification = document.createElement('div');
    notification.className = `fixed top-4 right-4 ${colors[type]} text-white px-6 py-3 rounded-lg shadow-lg z-50 transition-opacity`;
    notification.textContent = message;
    
    document.body.appendChild(notification);
    
    setTimeout(() => {
        notification.style.opacity = '0';
        setTimeout(() => notification.remove(), 300);
    }, 3000);
}

// Update last refresh timestamp
function updateLastRefresh() {
    const now = new Date();
    document.getElementById('lastUpdate').textContent = now.toLocaleTimeString('ru-RU');
}

// Escape HTML
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
    if (refreshInterval) {
        clearInterval(refreshInterval);
    }
});

console.log('✨ Neira Consciousness Dashboard ready!');
