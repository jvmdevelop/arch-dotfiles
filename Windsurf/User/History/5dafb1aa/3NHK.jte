@param String error = null

<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Регистрация — Первый IT лицей</title>
    <link rel="stylesheet" href="/css/main.css">
</head>
<body style="min-height:100vh;display:flex;flex-direction:column">

<header class="site-header">
    <div class="header-inner">
        <a href="/" class="site-logo">Лицей <span>№1</span></a>
    </div>
</header>

<div class="auth-page">
    <div class="auth-card">
        <h1 class="bellota-bold">Регистрация</h1>
        <p class="subtitle">Создайте аккаунт для доступа к порталу</p>

        @if(error != null)
            <div class="alert alert-danger">✗ ${error}</div>
        @endif

        <form method="post" action="/register">
            <div class="form-group">
                <label for="username">Имя пользователя</label>
                <input type="text" id="username" name="username" class="form-control" required autofocus
                       minlength="3" maxlength="50" placeholder="от 3 до 50 символов">
            </div>
            <div class="form-group">
                <label for="email">Электронная почта</label>
                <input type="email" id="email" name="email" class="form-control" required placeholder="example@mail.ru">
            </div>
            <div class="form-group">
                <label for="password">Пароль</label>
                <input type="password" id="password" name="password" class="form-control" required
                       minlength="6" placeholder="минимум 6 символов">
                <div class="form-hint">Минимум 6 символов</div>
            </div>
            <button type="submit" class="btn btn-primary w-100" style="margin-top:0.5rem">Зарегистрироваться</button>
        </form>

        <p style="text-align:center;margin-top:1.25rem;font-size:0.875rem;color:var(--text-muted)">
            Уже есть аккаунт? <a href="/login">Войти</a>
        </p>
        <p style="text-align:center;margin-top:0.5rem;font-size:0.875rem">
            <a href="/" style="color:var(--text-muted)">← На главную</a>
        </p>
    </div>
</div>

</body>
</html>
