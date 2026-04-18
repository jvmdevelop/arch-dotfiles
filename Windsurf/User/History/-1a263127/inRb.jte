@import com.jvmd.lyceum_backend.model.Vacancy
@import com.jvmd.lyceum_backend.model.Role
@import java.util.List
@import java.util.Set

@param List<Vacancy> vacancies
@param String currentUser = null
@param Set<Role> userRoles = null

@template.layout(
    title = "Вакансии",
    activePage = "vacancies",
    currentUser = currentUser,
    userRoles = userRoles,
    content = @`
        <div class="page-header bellota-bold">
            <div class="container">
                <h1 class="bellota-bold">Вакансии</h1>
                <p>Присоединяйтесь к нашей команде профессионалов</p>
            </div>
        </div>

        <section class="section">
            <div class="container">
                @if(vacancies.isEmpty())
                    <div class="empty-state">
                        <div class="empty-state-icon">💼</div>
                        <p>Открытых вакансий сейчас нет</p>
                    </div>
                @else
                    @for(Vacancy v : vacancies)
                        <div class="vacancy-card">
                            <div style="display:flex;align-items:flex-start;justify-content:space-between;gap:1rem">
                                <div>
                                    <h3>${v.getTitle()}</h3>
                                    @if(v.getDescription() != null && !v.getDescription().isBlank())
                                        <p style="margin-top:0.5rem">${v.getDescription()}</p>
                                    @endif
                                    <div style="margin-top:0.75rem;font-size:0.78rem;color:var(--text-muted)">
                                        Опубликовано: ${v.getCreatedAt().toLocalDate().toString()}
                                    </div>
                                </div>
                                <span class="badge badge-success" style="flex-shrink:0">Открыта</span>
                            </div>
                        </div>
                    @endfor
                @endif
            </div>
        </section>
    `
)
