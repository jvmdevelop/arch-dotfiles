NLP PIPELINE SERVICE
Задачи:

    Токенизация текстов (разбиение на статьи, пункты, предложения)
    Лемматизация для русского и казахского языков
    NER (Named Entity Recognition): извлечение правовых терминов, дат, сумм, ссылок на НПА
    Извлечение структурных элементов (статьи, части, пункты)
    Очистка и нормализация текста

Зависимости:

    tokenizers (Hugging Face tokenizers)
    rust-stemmers / kaznlp (для казахского языка)
    regex (парсинг структур)
    chrono (работа с датами)
    serde / serde_json (сериализация)
    nom (парсинг)

2. EMBEDDING SERVICE
Задачи:

    Векторизация текстов норм и документов
    Генерация 768-мерных эмбеддингов
    Поддержка batch processing
    Кэширование результатов
    Оптимизация инференса

Зависимости:

    candle-core / candle-transformers (Hugging Face inference)
    tokenizers (токенизация)
    ndarray / ndarray-linalg (векторные операции)
    blas / openblas-src (линейная алгебра)
    ort (ONNX Runtime, альтернатива)
    half (FP16 вычисления)
    rayon (параллелизация)

3. CLASSIFICATION SERVICE
Задачи:

    Классификация типов норм (императивные, диспозитивные, декларативные)
    Классификация сфер регулирования
    Определение уровня документа
    Предсказание с confidence score

Зависимости:

    candle-core / candle-transformers
    tokenizers
    serde / serde_json
    ndarray
    rand (для dropout при training)
    burn (альтернативный ML фреймворк)

4. CONFLICT DETECTION SERVICE
Задачи:

    Парная классификация норм на наличие противоречий
    Расчет confidence score
    Извлечение признаков (временное перекрытие, иерархия, схожесть)
    Генерация объяснений (feature importance)

Зависимости:

    candle-core / candle-transformers
    tokenizers
    ndarray
    chrono (временной анализ)
    serde / serde_json
    linfa (ML алгоритмы)
    smartcore (альтернативная ML библиотека)

5. SIMILARITY SERVICE
Задачи:

    Поиск похожих норм (approximate nearest neighbors)
    Расчет cosine similarity
    Jaccard similarity для текстового перекрытия
    HNSW индекс для быстрого поиска

Зависимости:

    hnswlib / usearch (HNSW алгоритм)
    ndarray
    rayon (параллельные вычисления)
    serde / serde_json
    faiss-rs (если нужен Facebook AI Similarity Search)

ОБЩИЕ ЗАВИСИМОСТИ (ВСЕ СЕРВИСЫ)
gRPC коммуникация:

    tonic (gRPC framework)
    prost (Protocol Buffers)
    tokio (async runtime)

Логирование и мониторинг:

    tracing / tracing-subscriber
    metrics / metrics-exporter-prometheus

Конфигурация:

    config
    dotenvy

Тестирование:

    cargo-test (встроенный)
    mockall (mocking)
    criterion (бенчмарки)

Утилиты:

    thiserror / anyhow (обработка ошибок)
    lazy_static / once_cell
    uuid
    clap (CLI, если нужен)

ИНФРАСТРУКТУРНЫЕ ЗАВИСИМОСТИ
Docker:

    Base image: rust:1.75-slim или debian:bookworm-slim + rust toolchain

Build:

    cargo package manager
    cross (cross-compilation, если нужно)

CI/CD:

    cargo fmt (formatting)
    cargo clippy (linting)
    cargo audit (security)

МОДЕЛИ ML (ДЛЯ ЗАГРУЗКИ)

    LegalBERT / RuBERT / KazBERT (ONNX формат для candle/ort)
    Tokenizer vocab files
    Model config files

