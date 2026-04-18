@import com.jvmd.lyceum_backend.model.Classroom
@import com.jvmd.lyceum_backend.model.Material
@import com.jvmd.lyceum_backend.model.Role
@import java.util.Set

@param Classroom classroom
@param String currentUser = null
@param Set<Role> userRoles = null

@template.layout(
    title = classroom.getTitle(),
    activePage = "classrooms",
    currentUser = currentUser,
    userRoles = userRoles,
    content = @`
        <div class="page-header bellota-bold">
            <div class="container">
                <div style="font-size:0.875rem;margin-bottom:0.5rem">
                    <a href="/classrooms" style="color:rgba(255,255,255,0.7)">← Кабинеты</a>
                </div>
                <h1>${classroom.getTitle()}</h1>
            </div>
        </div>

        <section class="section">
            <div class="container">
                <div style="display:grid;grid-template-columns:1fr 320px;gap:2rem;align-items:start">
                    <div>
                        @if(classroom.getHtmlContent() != null && !classroom.getHtmlContent().isBlank())
                            <div class="classroom-html-content" style="background:white;border:1px solid var(--border);border-radius:var(--radius-lg);padding:1.75rem;margin-bottom:1.5rem">
                                $unsafe{classroom.getHtmlContent()}
                            </div>
                        @endif

                        <div>
                            <h2 class="bellota-bold" style="font-size:1.15rem;font-weight:700;color:var(--primary);margin-bottom:1rem">
                                Учебные материалы
                                <span class="badge badge-secondary" style="margin-left:0.5rem">${classroom.getMaterials().size()}</span>
                            </h2>

                            @if(classroom.getMaterials().isEmpty())
                                <div class="empty-state" style="padding:2rem 0">
                                    <div class="empty-state-icon">📄</div>
                                    <p>Материалы пока не добавлены</p>
                                </div>
                            @else
                                @for(Material m : classroom.getMaterials())
                                    <div class="material-card">
                                        <div style="display:flex;justify-content:space-between;align-items:flex-start">
                                            <div style="flex:1">
                                                <h4>${m.getTitle()}</h4>
                                                @if(m.getDescription() != null && !m.getDescription().isBlank())
                                                    <p style="font-size:0.85rem;color:var(--text-muted);margin-top:0.25rem">${m.getDescription()}</p>
                                                @endif
                                                @if(m.getAuthor() != null)
                                                    <div style="font-size:0.75rem;color:var(--text-muted);margin-top:0.4rem">
                                                        ✍ ${m.getAuthor().getUsername()} · ${m.getCreatedAt().toLocalDate()}
                                                    </div>
                                                @endif
                                            </div>
                                        </div>
                                        @if(!m.getFileUrls().isEmpty())
                                            <div class="material-files">
                                                @for(String url : m.getFileUrls())
                                                    <a href="${url}" target="_blank" class="material-file">📎 Файл</a>
                                                @endfor
                                            </div>
                                        @endif
                                    </div>
                                @endfor
                            @endif
                        </div>
                    </div>

                    <aside>
                        <div style="background:white;border:1px solid var(--border);border-radius:var(--radius-lg);padding:1.25rem;position:sticky;top:calc(var(--header-h) + 1rem)">
                            <h3 class="bellota-bold" style="font-size:0.875rem;font-weight:700;text-transform:uppercase;letter-spacing:0.05em;color:var(--text-muted);margin-bottom:1rem">О кабинете</h3>
                            <div style="font-size:0.875rem">
                                <div style="display:flex;justify-content:space-between;padding:0.5rem 0;border-bottom:1px solid var(--border-light)">
                                    <span style="color:var(--text-muted)">Название</span>
                                    <span style="font-weight:600">${classroom.getTitle()}</span>
                                </div>
                                <div style="display:flex;justify-content:space-between;padding:0.5rem 0">
                                    <span style="color:var(--text-muted)">Материалов</span>
                                    <span style="font-weight:600">${classroom.getMaterials().size()}</span>
                                </div>
                            </div>
                        </div>
                    </aside>
                </div>
            </div>
        </section>
    `
)
