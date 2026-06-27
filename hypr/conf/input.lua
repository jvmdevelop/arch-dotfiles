-- https://wiki.hypr.land/Configuring/Basics/Variables/#input
hl.config({
    input = {
        kb_layout  = "us, ru",
        kb_options = "grp:alt_shift_toggle",

        follow_mouse = 1,
        sensitivity  = 0,

        touchpad = {
            natural_scroll = false,
        },
    },
})

-- https://wiki.hypr.land/Configuring/Gestures/
hl.gesture({
    fingers   = 3,
    direction = "horizontal",
    action    = "workspace",
})

hl.device({
    name        = "epic-mouse-v1",
    sensitivity = -0.5,
})
