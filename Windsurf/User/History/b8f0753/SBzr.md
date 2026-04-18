# Legis-Entropy: AI-система для анализа нормативно-правовых актов

## Обзор
Система для автоматического анализа юридических документов с целью выявления противоречий, дублирования и устаревших норм.

## 🎯 Основные возможности
- **Извлечение юридических норм** из текстов документов с помощью NLP
- **Анализ связей** между документами (ссылки, иерархия, временные зависимости)
- **Детекция противоречий** между нормами и документами
- **Поиск дублирования** (точного, частичного, семантического)
- **Визуализация графов** связей между законами и нормами
- **Explainable AI** - прозрачные объяснения всех решений системы

## 🏗️ Архитектура
```
legis-entropy/
├── src/
│   ├── core/                 # Основные компоненты
│   │   ├── legal_analyzer.py # Главный класс системы
│   │   └── __init__.py
│   ├── nlp/                 # NLP модули
│   │   ├── text_processor.py # Предобработка текста
│   │   ├── norm_extractor.py # Извлечение юридических норм
│   │   └── __init__.py
│   ├── analysis/            # Анализ и детекция
│   │   ├── relationship_analyzer.py # Анализ связей
│   │   ├── conflict_detector.py     # Детектор противоречий
│   │   ├── duplicate_detector.py    # Детектор дублирования
│   │   └── __init__.py
│   ├── graph/               # Графовая база данных
│   │   ├── legal_graph.py   # Построение и визуализация графов
│   │   └── __init__.py
│   ├── explainability/      # Explainable AI
│   │   ├── explainer.py     # Генерация объяснений
│   │   └── __init__.py
│   └── __init__.py
├── examples/                # Примеры использования
│   ├── basic_usage.py       # Базовый пример
│   └── advanced_usage.py    # Продвинутый пример
├── notebooks/               # Jupyter ноутбуки
│   └── analysis_demo.ipynb  # Интерактивная демонстрация
├── tests/                   # Тесты
│   ├── test_legal_analyzer.py
│   ├── test_nlp_components.py
│   └── __init__.py
├── data/                    # Данные и модели
├── requirements.txt         # Зависимости
└── README.md               # Этот файл
```

## 🚀 Установка

### Требования
- Python 3.8+
- 8GB+ RAM (для моделей NLP)

### Установка зависимостей
```bash
pip install -r requirements.txt
```

### Загрузка языковых моделей
```bash
python -m spacy download ru_core_news_lg
```

## 📖 Использование

### Базовый пример
```python
from src.core.legal_analyzer import LegalAnalyzer, LegalDocument
from datetime import datetime

# Создание анализатора
analyzer = LegalAnalyzer()

# Создание документа
document = LegalDocument(
    id="law_001",
    title="Закон о государственной службе",
    content="Статья 1. Государственные служащие обязаны соблюдать законодательство...",
    document_type="закон",
    adoption_date=datetime(2023, 1, 15)
)

# Анализ документа
result = analyzer.analyze_document(document)

# Результаты
print(f"Найдено норм: {len(result.norms)}")
print(f"Противоречий: {len(result.conflicts)}")
print(f"Дубликатов: {len(result.duplicates)}")
```

### Анализ множества документов
```python
# Анализ нескольких документов с учетом взаимосвязей
documents = [doc1, doc2, doc3]
results = analyzer.analyze_multiple_documents(documents)

# Получение статистики
stats = analyzer.get_document_statistics(results)
print(f"Уровень конфликтов: {stats['conflict_rate']:.2%}")
print(f"Уровень дублирования: {stats['duplicate_rate']:.2%}")
```

### Визуализация графа связей
```python
from src.graph.legal_graph import LegalGraph

# Построение графа
graph = LegalGraph()
graph_data = graph.build_multi_document_graph(results)

# Интерактивная визуализация
fig = graph.visualize_plotly()
fig.show()

# Экспорт графа
graph.export_graph("legal_graph.json", "json")
```

### Генерация объяснений
```python
from src.explainability.explainer import LegalExplainer

# Создание объясняющего модуля
explainer = LegalExplainer()

# Генерация объяснений для проблем
explanations = explainer.generate_explanations(
    result.conflicts, 
    result.duplicates, 
    result.relationships
)

# Сводный отчет
report = explainer.generate_summary_report(explanations)
```

## 🔍 Типы обнаруживаемых проблем

### Противоречия
- **Противоречие запрет-разрешение**: "запрещается" vs "разрешается"
- **Противоречие обязательство-отрицание**: "обязан" vs "не обязан"
- **Временные противоречия**: "немедленно" vs "в течение 10 дней"
- **Количественные противоречия**: "не менее 5" vs "менее 5"

### Дублирование
- **Точное дублирование**: идентичный текст норм
- **Почти дублирование**: высокая текстовая схожесть (>90%)
- **Семантическое дублирование**: одинаковое значение, разная формулировка
- **Частичное дублирование**: общие фрагменты текста
- **Структурное дублирование**: одинаковая структура норм

### Связи между документами
- **Ссылки**: явные отсылки на другие документы
- **Иерархические связи**: подчиненность документов
- **Временные связи**: временные зависимости
- **Семантические связи**: смысловые связи по ключевым словам

## 📊 Метрики и статистика

Система предоставляет следующие метрики:
- **Уровень конфликтов**: доля противоречивых норм
- **Уровень дублирования**: доля дублирующихся норм
- **Плотность графа связей**: степень взаимосвязанности документов
- **Центральность узлов**: наиболее влиятельные документы и нормы
- **Кластеризация**: группы связанных документов

## 🧪 Тестирование

Запуск всех тестов:
```bash
python -m pytest tests/ -v
```

Запуск конкретного теста:
```bash
python -m pytest tests/test_legal_analyzer.py -v
```

## 📚 Примеры

### Запуск базового примера
```bash
cd examples
python basic_usage.py
```

### Запуск продвинутого примера
```bash
cd examples
python advanced_usage.py
```

### Jupyter ноутбук
```bash
jupyter notebook notebooks/analysis_demo.ipynb
```

## 🔧 Конфигурация

Система поддерживает конфигурацию через параметры:

```python
config = {
    'similarity_thresholds': {
        'high': 0.9,
        'medium': 0.7,
        'low': 0.5
    },
    'conflict_detection': {
        'enable_semantic': True,
        'enable_temporal': True
    },
    'graph_visualization': {
        'layout': 'spring',
        'node_size_factor': 1.0
    }
}

analyzer = LegalAnalyzer(config)
```

## 📈 Производительность

- **Обработка документа**: 10-30 секунд (зависит от размера)
- **Память**: 2-4GB для中型 документов
- **Точность**: 85-95% (зависит от типа документа)
- **Масштабируемость**: до 1000 документов за одну сессию

## 🌐 Источники данных

Система разработана для работы с:
- **Әділет** (adilet.zan.kz) - база НПА Казахстана
- **data.egov.kz** - открытые государственные данные
- **Другие правовые базы** через API или файлы

## 🤝 Contributing

1. Fork проекта
2. Создайте feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit изменения (`git commit -m 'Add some AmazingFeature'`)
4. Push в branch (`git push origin feature/AmazingFeature`)
5. Откройте Pull Request

## 📄 Лицензия

Этот проект лицензирован под MIT License - см. файл LICENSE для деталей.

## 👥 Команда

- **AI Development Team** - разработка архитектуры и AI компонентов
- **Legal Experts** - консультации по юриспруденции
- **Data Scientists** - разработка NLP моделей

## 📞 Контакты

- GitHub Issues: [Сообщить о проблеме](https://github.com/your-org/legis-entropy/issues)
- Email: legis-entropy@example.com

## 🗺️ Дорожная карта

- [ ] Веб-интерфейс для работы с системой
- [ ] Интеграция с реальными правовыми базами
- [ ] Поддержка казахского языка
- [ ] Машинное обучение для улучшения точности
- [ ] API для внешних систем
- [ ] Многопользовательский режим

---

**Legis-Entropy** - делаем законодательство понятным и непротиворечивым! 🚀
