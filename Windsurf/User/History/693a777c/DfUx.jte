@param String error = null

<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Вход — Первый IT лицей</title>
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
        <h1 class="bellota-bold">Вход в систему</h1>
        <p class="subtitle">Введите ваши данные для входа</p>

        @if(error != null)
            <div class="alert alert-danger">✗ ${error}</div>
        @endif

        <form method="post" action="/login">
            <div class="form-group">
                <label for="username">Имя пользователя</label>
                <input type="text" id="username" name="username" class="form-control" required autofocus placeholder="username">
            </div>
            <div class="form-group">
                <label for="password">Пароль</label>
                <input type="password" id="password" name="password" class="form-control" required placeholder="••••••••">
            </div>
            <button type="submit" class="btn btn-primary w-100" style="margin-top:0.5rem">Войти</button>
        </form>

        <p style="text-align:center;margin-top:1.25rem;font-size:0.875rem;color:var(--text-muted)">
            Нет аккаунта? <a href="/register">Зарегистрироваться</a>
        </p>
        <p style="text-align:center;margin-top:0.5rem;font-size:0.875rem">
            <a href="/" style="color:var(--text-muted)">← На главную</a>
        </p>
    </div>
</div>

</body>
</html>
