@import com.jvmd.lyceum_backend.model.News
@import com.jvmd.lyceum_backend.model.Achievement
@import com.jvmd.lyceum_backend.model.Role
@import com.jvmd.lyceum_backend.payload.SiteStatsResponse
@import java.util.List
@import java.util.Set

@param List<News> recentNews
@param List<Achievement> recentAchievements
@param SiteStatsResponse stats
@param String currentUser = null
@param Set<Role> userRoles = null

@template.layout(
    title = "Главная",
    activePage = "home",
    currentUser = currentUser,
    userRoles = userRoles,
    content = @`
        <section class="hero">
            <div class="container bellota-bold">
                <h1>Добро пожаловать в <span>Первый IT лицей</span></h1>
                <p>Хочешь быть первым, учись в первом.</p>
                <div class="hero-actions bellota-bold">
                    <a href="/admission" class="btn btn-accent btn-lg">Подать заявку</a>
                    <a href="/news" class="btn btn-outline-white btn-lg">Новости школы</a>
                </div>
            </div>
        </section>

        <section class="stats-bar bellota-bold">
            <div class="container">
                <div class="stats-grid">
                    <div class="stat-item">
                        <span class="stat-num">${stats.getNewsCount()}</span>
                        <span class="stat-label">Публикаций</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-num">${stats.getAchievementsCount()}</span>
                        <span class="stat-label">Достижений</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-num">${stats.getPortfolioCount()}</span>
                        <span class="stat-label">Портфолио</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-num">${stats.getApplicationsCount()}</span>
                        <span class="stat-label">Заявок</span>
                    </div>
                </div>
            </div>
        </section>

        <section class="section bellota-bold">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title bellota-bold">Последние новости</h2>
                    <a href="/news" class="section-link">Все новости →</a>
                </div>
                @if(recentNews.isEmpty())
                    <div class="empty-state">
                        <div class="empty-state-icon">📰</div>
                        <p>Новостей пока нет</p>
                    </div>
                @else
                    <div class="grid grid-3">
                        @for(News news : recentNews)
                            <div class="card">
                                @if(news.getImageUrl() != null)
                                    <img src="${news.getImageUrl()}" alt="${news.getTitle()}" class="card-img">
                                @else
                                    <div class="card-img-placeholder">📰</div>
                                @endif
                                <div class="card-body">
                                    @if(news.getCategory() != null)
                                        <span class="badge badge-secondary mb-1">${news.getCategory()}</span>
                                    @endif
                                    <h3 class="card-title">${news.getTitle()}</h3>
                                    <p class="card-text">${news.getContent().length() > 120 ? news.getContent().substring(0, 120) + "..." : news.getContent()}</p>
                                    <div class="card-meta">
                                        @if(news.getUser() != null)
                                            <span>✍ ${news.getUser().getUsername()}</span>
                                        @endif
                                        <span>${news.getCreatedAt().toLocalDate().toString()}</span>
                                    </div>
                                </div>
                            </div>
                        @endfor
                    </div>
                @endif
            </div>
        </section>

        <section class="section section-alt">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title bellota-bold">Достижения учеников</h2>
                    <a href="/achievements" class="section-link">Все достижения →</a>
                </div>
                @if(recentAchievements.isEmpty())
                    <div class="empty-state">
                        <div class="empty-state-icon">🏆</div>
                        <p>Достижений пока нет</p>
                    </div>
                @else
                    <div class="grid grid-3">
                        @for(Achievement a : recentAchievements)
                            <div class="card">
                                <div class="card-body achievement-card">
                                    <div class="achievement-level ${a.getLevel().toLowerCase().contains("золот") || a.getLevel().equalsIgnoreCase("gold") || a.getLevel().equals("1") ? "level-gold" : a.getLevel().toLowerCase().contains("серебр") || a.getLevel().equalsIgnoreCase("silver") || a.getLevel().equals("2") ? "level-silver" : a.getLevel().toLowerCase().contains("бронз") || a.getLevel().equalsIgnoreCase("bronze") || a.getLevel().equals("3") ? "level-bronze" : "level-other"}">
                                        ${a.getLevel()}
                                    </div>
                                    <h3 class="card-title">${a.getTitle()}</h3>
                                    <p class="card-text">${a.getStudentName()} · ${a.getYear()}</p>
                                    <p class="card-text">${a.getPlace()}</p>
                                    <div class="card-meta">
                                        <span class="badge badge-primary">${a.getCategory()}</span>
                                    </div>
                                </div>
                            </div>
                        @endfor
                    </div>
                @endif
            </div>
        </section>

        <section class="section">
            <div class="container">
                <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:1.5rem">
                    <div style="background:var(--primary);color:white;border-radius:var(--radius-lg);padding:2rem">
                        <div style="font-size:2rem;margin-bottom:0.75rem">🏛️</div>
                        <h3 class="bellota-bold" style="font-size:1.2rem;margin-bottom:0.5rem">Школьный музей</h3>
                        <p style="color:rgba(255,255,255,0.75);font-size:0.875rem;margin-bottom:1rem">Узнайте историю нашей школы через уникальные экспонаты и архивные материалы.</p>
                        <a href="/museum" class="btn btn-accent btn-sm">Посетить музей</a>
                    </div>
                    <div style="background:var(--secondary);color:white;border-radius:var(--radius-lg);padding:2rem">
                        <div style="font-size:2rem;margin-bottom:0.75rem">📚</div>
                        <h3 class="bellota-bold" style="font-size:1.2rem;margin-bottom:0.5rem">Учебные кабинеты</h3>
                        <p style="color:rgba(255,255,255,0.75);font-size:0.875rem;margin-bottom:1rem">Доступ к материалам, ресурсам и расписанию учебных кабинетов школы.</p>
                        <a href="/classrooms" class="btn btn-outline-white btn-sm">Смотреть кабинеты</a>
                    </div>
                    <div style="background:#2a9d8f;color:white;border-radius:var(--radius-lg);padding:2rem">
                        <div style="font-size:2rem;margin-bottom:0.75rem">📋</div>
                        <h3 class="bellota-bold" style="font-size:1.2rem;margin-bottom:0.5rem">Поступление</h3>
                        <p style="color:rgba(255,255,255,0.75);font-size:0.875rem;margin-bottom:1rem">Запишите ребёнка в наш лицей. Мы принимаем детей в 1 класс и дошкольную группу.</p>
                        <a href="/admission" class="btn btn-outline-white btn-sm">Подать заявку</a>
                    </div>
                </div>
            </div>
        </section>
    `
)
