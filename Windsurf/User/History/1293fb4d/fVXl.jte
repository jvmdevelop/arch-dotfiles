@import com.jvmd.lyceum_backend.model.News
@import com.jvmd.lyceum_backend.model.Role
@import java.util.List
@import java.util.Set

@param List<News> newsList
@param String currentUser = null
@param Set<Role> userRoles = null
@param String success = null

@template.layout(
    title = "Новости",
    activePage = "news",
    currentUser = currentUser,
    userRoles = userRoles,
    content = @`
        <div class="page-header bellota-bold">
            <div class="container">
                <h1 class="bellota-bold">Новости</h1>
                <p>Актуальные события и публикации нашей школы</p>
            </div>
        </div>

        <section class="section">
            <div class="container">
                @if(success != null)
                    <div class="alert alert-success">✓ ${success}</div>
                @endif

                @if(newsList.isEmpty())
                    <div class="empty-state">
                        <div class="empty-state-icon">📰</div>
                        <p>Новостей пока нет</p>
                    </div>
                @else
                    <div class="grid grid-2">
                        @for(News news : newsList)
                            <div class="card">
                                @if(news.getImageUrl() != null)
                                    <img src="${news.getImageUrl()}" alt="${news.getTitle()}" class="card-img">
                                @else
                                    <div class="card-img-placeholder">📰</div>
                                @endif
                                <div class="card-body">
                                    @if(news.getCategory() != null)
                                        <span class="badge badge-secondary">${news.getCategory()}</span>
                                    @endif
                                    <h2 class="card-title" style="margin-top:0.5rem">${news.getTitle()}</h2>
                                    <p class="card-text">${news.getContent()}</p>
                                    <div class="card-meta" style="margin-top:1rem">
                                        @if(news.getUser() != null)
                                            <span>✍ ${news.getUser().getUsername()}</span>
                                        @endif
                                        <span>🗓 ${news.getCreatedAt().toLocalDate().toString()}</span>
                                    </div>
                                </div>
                            </div>
                        @endfor
                    </div>
                @endif
            </div>
        </section>
    `
)
