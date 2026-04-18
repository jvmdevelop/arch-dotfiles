@import com.jvmd.lyceum_backend.model.MuseumExhibit
@import com.jvmd.lyceum_backend.model.Role
@import java.util.Set

@param MuseumExhibit exhibit
@param String currentUser = null
@param Set<Role> userRoles = null

@template.layout(
    title = exhibit.getTitle(),
    activePage = "museum",
    currentUser = currentUser,
    userRoles = userRoles,
    content = @`
        <div class="page-header bellota-bold">
            <div class="container">
                <div style="font-size:0.875rem;margin-bottom:0.5rem">
                    <a href="/museum" style="color:rgba(255,255,255,0.7)">← Музей</a>
                </div>
                <h1>${exhibit.getTitle()}</h1>
            </div>
        </div>

        <section class="section">
            <div class="container">
                @if(!exhibit.getImageUrls().isEmpty())
                    <div class="exhibit-images">
                        @for(String url : exhibit.getImageUrls())
                            <img src="${url}" alt="${exhibit.getTitle()}" onclick="this.requestFullscreen && this.requestFullscreen()">
                        @endfor
                    </div>
                @endif

                @if(exhibit.getHtmlContent() != null && !exhibit.getHtmlContent().isBlank())
                    <div class="exhibit-html-content">
                        $unsafe{exhibit.getHtmlContent()}
                    </div>
                @elseif(exhibit.getDescription() != null)
                    <p style="font-size:1.05rem;line-height:1.7;color:var(--text)">${exhibit.getDescription()}</p>
                @endif

                @if(!exhibit.getRelatedExhibits().isEmpty())
                    <hr class="divider">
                    <h2 class="bellota-bold" style="font-size:1.25rem;font-weight:700;color:var(--primary);margin-bottom:1.25rem">Связанные экспонаты</h2>
                    <div class="grid grid-3">
                        @for(MuseumExhibit rel : exhibit.getRelatedExhibits())
                            <div class="card">
                                @if(!rel.getImageUrls().isEmpty())
                                    <img src="${rel.getImageUrls().get(0)}" alt="${rel.getTitle()}" class="card-img">
                                @else
                                    <div class="card-img-placeholder">🏛️</div>
                                @endif
                                <div class="card-body">
                                    <h3 class="card-title">${rel.getTitle()}</h3>
                                </div>
                                <div class="card-footer">
                                    <a href="/museum/${rel.getId()}" class="btn btn-primary btn-sm">Смотреть</a>
                                </div>
                            </div>
                        @endfor
                    </div>
                @endif
            </div>
        </section>
    `
)
