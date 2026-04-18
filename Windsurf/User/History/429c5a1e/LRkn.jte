@import com.jvmd.lyceum_backend.model.Achievement
@import com.jvmd.lyceum_backend.model.Role
@import java.util.List
@import java.util.Set

@param List<Achievement> achievements
@param String currentUser = null
@param Set<Role> userRoles = null

@template.layout(
    title = "Достижения",
    activePage = "achievements",
    currentUser = currentUser,
    userRoles = userRoles,
    content = @`
        <div class="page-header bellota-bold">
            <div class="container">
                <h1 class="bellota-bold">Достижения учеников</h1>
                <p>Гордость нашей школы — успехи наших учеников на олимпиадах и конкурсах</p>
            </div>
        </div>

        <section class="section">
            <div class="container">
                @if(achievements.isEmpty())
                    <div class="empty-state">
                        <div class="empty-state-icon">🏆</div>
                        <p>Достижений пока нет</p>
                    </div>
                @else
                    <div class="grid grid-3">
                        @for(Achievement a : achievements)
                            <div class="card">
                                <div class="card-body achievement-card">
                                    <div class="achievement-level ${a.getLevel().toLowerCase().contains("золот") || a.getLevel().equalsIgnoreCase("gold") || a.getLevel().equals("1") ? "level-gold" : a.getLevel().toLowerCase().contains("серебр") || a.getLevel().equalsIgnoreCase("silver") || a.getLevel().equals("2") ? "level-silver" : a.getLevel().toLowerCase().contains("бронз") || a.getLevel().equalsIgnoreCase("bronze") || a.getLevel().equals("3") ? "level-bronze" : "level-other"}">
                                        ${a.getLevel()}
                                    </div>
                                    <h3 class="card-title bellota-bold">${a.getTitle()}</h3>
                                    <p class="card-text" style="margin-bottom:0.4rem">${a.getDescription()}</p>
                                    <div class="card-meta">
                                        <span class="badge badge-primary">${a.getCategory()}</span>
                                        <span>${a.getYear()}</span>
                                    </div>
                                    <div style="margin-top:0.75rem;padding-top:0.75rem;border-top:1px solid var(--border-light)">
                                        <div style="font-size:0.85rem;color:var(--text-muted)">
                                            <strong style="color:var(--primary)">${a.getStudentName()}</strong>
                                            <br>${a.getPlace()}
                                        </div>
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
