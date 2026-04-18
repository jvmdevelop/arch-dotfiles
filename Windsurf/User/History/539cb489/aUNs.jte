@import com.jvmd.lyceum_backend.model.Role
@import java.util.Set

@param String currentUser = null
@param Set<Role> userRoles = null
@param String success = null
@param String error = null

@template.layout(
    title = "Поступление",
    activePage = "admission",
    currentUser = currentUser,
    userRoles = userRoles,
    content = @`
        <div class="page-header bellota-bold">
            <div class="container">
                <h1>Поступление в лицей</h1>
                <p>Заполните заявку, и мы свяжемся с вами в течение 2–3 рабочих дней</p>
            </div>
        </div>

        <section class="section">
            <div class="container" style="max-width:700px">
                @if(success != null)
                    <div class="alert alert-success">
                        ✓ ${success}
                    </div>
                @endif
                @if(error != null)
                    <div class="alert alert-danger">
                        ✗ ${error}
                    </div>
                @endif

                <div class="admission-section">
                    <h2 class="bellota-bold" style="font-size:1.25rem;font-weight:700;color:var(--primary);margin-bottom:1.5rem">Заявка на поступление</h2>

                    <form method="post" action="/admission">
                        <h3 class="bellota-bold" style="font-size:1rem;font-weight:700;color:var(--text-muted);text-transform:uppercase;letter-spacing:0.05em;margin-bottom:1rem">Информация о родителе</h3>
                        <div class="form-group">
                            <label for="parentName">ФИО родителя / опекуна *</label>
                            <input type="text" id="parentName" name="parentName" class="form-control" required placeholder="Иванов Иван Иванович">
                        </div>

                        <hr class="divider">
                        <h3 class="bellota-bold" style="font-size:1rem;font-weight:700;color:var(--text-muted);text-transform:uppercase;letter-spacing:0.05em;margin-bottom:1rem">Информация о ребёнке</h3>

                        <div class="form-row">
                            <div class="form-group">
                                <label for="childName">ФИО ребёнка *</label>
                                <input type="text" id="childName" name="childName" class="form-control" required placeholder="Иванов Пётр Иванович">
                            </div>
                            <div class="form-group">
                                <label for="childAge">Возраст ребёнка *</label>
                                <input type="number" id="childAge" name="childAge" class="form-control" required min="4" max="18" placeholder="7">
                            </div>
                        </div>

                        <div class="form-group">
                            <label for="classType">Класс поступления *</label>
                            <select id="classType" name="classType" class="form-control" required>
                                <option value="">— Выберите —</option>
                                <option value="1_CLASS">1 класс</option>
                                <option value="PRESCHOOL">Дошкольная группа</option>
                            </select>
                        </div>

                        <hr class="divider">
                        <h3 class="bellota-bold" style="font-size:1rem;font-weight:700;color:var(--text-muted);text-transform:uppercase;letter-spacing:0.05em;margin-bottom:1rem">Контактные данные</h3>

                        <div class="form-row">
                            <div class="form-group">
                                <label for="phone">Телефон *</label>
                                <input type="tel" id="phone" name="phone" class="form-control" required placeholder="+7 (999) 000-00-00">
                            </div>
                            <div class="form-group">
                                <label for="email">Электронная почта</label>
                                <input type="email" id="email" name="email" class="form-control" placeholder="example@mail.ru">
                            </div>
                        </div>

                        <div class="form-group">
                            <label for="message">Дополнительная информация</label>
                            <textarea id="message" name="message" class="form-control" rows="4" placeholder="Укажите любую важную информацию о ребёнке, вопросы и пожелания..."></textarea>
                        </div>

                        <button type="submit" class="btn btn-primary btn-lg w-100">Отправить заявку</button>
                        <p class="form-hint text-center mt-2">Нажимая кнопку, вы соглашаетесь на обработку персональных данных</p>
                    </form>
                </div>
            </div>
        </section>
    `
)
