package com.jvmd.uownserver.controller.impl;

import com.jvmd.uownserver.controller.Controller;
import com.jvmd.uownserver.manager.Managers;
import com.jvmd.uownserver.model.Donate;
import com.jvmd.uownserver.model.DonateType;
import com.jvmd.uownserver.model.EOwn;
import io.javalin.Javalin;
import lombok.extern.slf4j.Slf4j;

@Slf4j
public class UserOwnController implements Controller {

    public UserOwnController() {
        startServer();
    }

    @Override
    public void setupRoutes(Javalin app) {
        app.get("/api/v1/user/own", ctx -> {
            String userName = ctx.queryParam("username");
            if (userName == null) {
                ctx.status(400).result("Missing username param");
                return;
            }

            EOwn o = Managers.getUserOwnManager().getUserOwn(userName);
            if (o != null) {
                ctx.result(o.toString());
            } else {
                ctx.status(404).result("User not found");
            }
        });

        app.post("/api/v1/donate", ctx -> {
            String nickname = ctx.formParam("username");
            String userDonate = ctx.formParam("donate");

            if (nickname == null || userDonate == null) {
                ctx.status(400).result("Missing required params: username, donate");
                return;
            }

            log.info("Received donation: {} {}", nickname, userDonate);

            try {
                Donate donate = new Donate();
                donate.setNickname(nickname);
                donate.setType(DonateType.valueOf(userDonate.toUpperCase()));
                Managers.getDonateManager().addObject(donate);
                ctx.result("OK");
            } catch (IllegalArgumentException e) {
                ctx.status(400).result("Invalid donate type: " + userDonate);
            }
        });
    }
}
