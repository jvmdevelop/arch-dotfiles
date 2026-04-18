@import com.jvmd.lyceum_backend.model.StudentPortfolio
@import com.jvmd.lyceum_backend.model.Role
@import java.util.List
@import java.util.Set

@param List<StudentPortfolio> portfolios
@param String currentUser = null
@param Set<Role> userRoles = null

@template.layout(
    title = "Портфолио",
    activePage = "portfolio",
    currentUser = currentUser,
    userRoles = userRoles,
    content = @`
        <div class="page-header bellota-bold">
            <div class="container">
                <h1 class="bellota-bold">Портфолио студентов</h1>
                <p>Знакомьтесь с нашими талантливыми учениками</p>
            </div>
        </div>

        <section class="section">
            <div class="container">
                @if(portfolios.isEmpty())
                    <div class="empty-state">
                        <div class="empty-state-icon">👨‍🎓</div>
                        <p>Портфолио пока нет</p>
                    </div>
                @else
                    <div class="grid grid-2">
                        @for(StudentPortfolio p : portfolios)
                            <div class="card">
                                <div class="card-body">
                                    <div class="portfolio-card">
                                        @if(p.getImageUrl() != null)
                                            <img src="${p.getImageUrl()}" alt="${p.getStudentName()}" class="portfolio-avatar">
                                        @else
                                            <div class="portfolio-avatar-placeholder">👤</div>
                                        @endif
                                        <div style="flex:1">
                                            <h3 class="card-title" style="margin-bottom:0.2rem">${p.getStudentName()}</h3>
                                            <span class="badge badge-secondary">${p.getGrade()}</span>
                                        </div>
                                    </div>
                                    <p class="card-text" style="margin-top:1rem">${p.getBio()}</p>

                                    @if(!p.getSkills().isEmpty())
                                        <div style="margin-top:0.875rem">
                                            <div style="font-size:0.78rem;font-weight:700;color:var(--text-muted);text-transform:uppercase;letter-spacing:0.05em;margin-bottom:0.4rem">Навыки</div>
                                            <div class="tags">
                                                @for(String skill : p.getSkills())
                                                    <span class="tag">${skill}</span>
                                                @endfor
                                            </div>
                                        </div>
                                    @endif

                                    @if(!p.getAchievements().isEmpty())
                                        <div style="margin-top:0.875rem">
                                            <div style="font-size:0.78rem;font-weight:700;color:var(--text-muted);text-transform:uppercase;letter-spacing:0.05em;margin-bottom:0.4rem">Достижения</div>
                                            <ul style="padding-left:1.2rem;font-size:0.85rem;color:var(--text-muted)">
                                                @for(String ach : p.getAchievements())
                                                    <li>${ach}</li>
                                                @endfor
                                            </ul>
                                        </div>
                                    @endif

                                    <div class="card-meta" style="margin-top:1rem">
                                        <span style="font-size:0.78rem">Добавлено: ${p.getCreatedAt().toLocalDate().toString()}</span>
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
