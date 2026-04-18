@import com.jvmd.lyceum_backend.model.MuseumExhibit
@import com.jvmd.lyceum_backend.model.Role
@import java.util.List
@import java.util.Set

@param List<MuseumExhibit> exhibits
@param String currentUser = null
@param Set<Role> userRoles = null

@template.layout(
    title = "Школьный музей",
    activePage = "museum",
    currentUser = currentUser,
    userRoles = userRoles,
    content = @`
        <div class="page-header bellota-bold">
            <div class="container">
                <h1>Школьный музей</h1>
                <p>Сохраняем историю. Вдохновляем будущее.</p>
            </div>
        </div>

        <section class="section">
            <div class="container">
                @if(exhibits.isEmpty())
                    <div class="empty-state">
                        <div class="empty-state-icon">🏛️</div>
                        <p>Экспонаты пока не добавлены</p>
                    </div>
                @else
                    <div class="grid grid-3">
                        @for(MuseumExhibit e : exhibits)
                            <div class="card">
                                @if(!e.getImageUrls().isEmpty())
                                    <img src="${e.getImageUrls().get(0)}" alt="${e.getTitle()}" class="card-img">
                                @else
                                    <div class="card-img-placeholder">🏛️</div>
                                @endif
                                <div class="card-body">
                                    <h3 class="card-title">${e.getTitle()}</h3>
                                    @if(e.getDescription() != null)
                                        <p class="card-text">${e.getDescription().length() > 100 ? e.getDescription().substring(0, 100) + "..." : e.getDescription()}</p>
                                    @endif
                                    @if(!e.getImageUrls().isEmpty())
                                        <div class="card-meta">
                                            <span style="font-size:0.78rem">📷 ${e.getImageUrls().size()} фото</span>
                                        </div>
                                    @endif
                                </div>
                                <div class="card-footer">
                                    <a href="/museum/${e.getId()}" class="btn btn-primary btn-sm">Подробнее</a>
                                    @if(!e.getRelatedExhibits().isEmpty())
                                        <span class="text-muted" style="font-size:0.78rem">связано: ${e.getRelatedExhibits().size()}</span>
                                    @endif
                                </div>
                            </div>
                        @endfor
                    </div>
                @endif
            </div>
        </section>
    `
)
