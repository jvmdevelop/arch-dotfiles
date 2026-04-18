@import com.jvmd.lyceum_backend.model.Classroom
@import com.jvmd.lyceum_backend.model.Role
@import java.util.List
@import java.util.Set

@param List<Classroom> classrooms
@param String currentUser = null
@param Set<Role> userRoles = null

@template.layout(
    title = "Учебные кабинеты",
    activePage = "classrooms",
    currentUser = currentUser,
    userRoles = userRoles,
    content = @`
        <div class="page-header bellota-bold">
            <div class="container">
                <h1 class="bellota-bold">Учебные кабинеты</h1>
                <p>Материалы, ресурсы и информация по каждому кабинету</p>
            </div>
        </div>

        <section class="section">
            <div class="container">
                @if(classrooms.isEmpty())
                    <div class="empty-state">
                        <div class="empty-state-icon">📚</div>
                        <p>Кабинеты пока не добавлены</p>
                    </div>
                @else
                    <div class="grid grid-3">
                        @for(Classroom c : classrooms)
                            <div class="card">
                                <div class="card-img-placeholder" style="height:130px;font-size:3rem">📚</div>
                                <div class="card-body">
                                    <h3 class="card-title bellota-bold">${c.getTitle()}</h3>
                                    <div class="card-meta">
                                        <span>📄 ${c.getMaterials().size()} материалов</span>
                                    </div>
                                </div>
                                <div class="card-footer">
                                    <a href="/classrooms/${c.getId()}" class="btn btn-primary btn-sm">Открыть</a>
                                </div>
                            </div>
                        @endfor
                    </div>
                @endif
            </div>
        </section>
    `
)
