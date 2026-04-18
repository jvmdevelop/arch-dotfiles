package com.jvmd.uownserver.controller.impl;

import com.jvmd.uownserver.controller.Controller;
import com.jvmd.uownserver.manager.Managers;
import com.jvmd.uownserver.model.Donate;
import com.jvmd.uownserver.model.DonateType;
import com.jvmd.uownserver.model.EOwn;
import lombok.extern.slf4j.Slf4j;

import java.net.URLDecoder;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Map;

@Slf4j
public class UserOwnController implements Controller {

    public UserOwnController() {
        openConnection();
    }

    @Override
    public String getResponse(String path) {
        if (path == null || path.isEmpty()) return "String is empty";

        int queryIndex = path.indexOf('?');
        if (queryIndex == -1) return "Params in empty";

        String queryString = path.substring(queryIndex + 1);
        Map<String, String> params = new HashMap<>();

        for (String pair : queryString.split("&")) {
            int eqIndex = pair.indexOf('=');
            if (eqIndex > 0) {
                String key = URLDecoder.decode(pair.substring(0, eqIndex), StandardCharsets.UTF_8);
                String value = URLDecoder.decode(pair.substring(eqIndex + 1), StandardCharsets.UTF_8);
                params.put(key, value);
            }
        }

        if (params.isEmpty()) return "Params in empty";

        if (path.startsWith("/api/v1/user/own")) {
            String userName = params.get("username");
            if (userName == null) return "Missing username param";

            EOwn o = Managers.getUserOwnManager().getUserOwn(userName);
            return o != null ? o.toString() : "User not found";
        }

        if (path.startsWith("/api/v1/donate")) {
            String nickname = params.get("username");
            String userDonate = params.get("donate");

            if (nickname == null || userDonate == null) {
                return "Missing required params: username, donate";
            }

            log.info("Received donation: {} {}", nickname, userDonate);

            Donate donate = new Donate();
            donate.setNickname(nickname);
            try {
                donate.setType(DonateType.valueOf(userDonate.toUpperCase()));
                Managers.getDonateManager().addObject(donate);
                return "OK";
            } catch (IllegalArgumentException e) {
                return "Invalid donate type: " + userDonate;
            }
        }

        return null;
    }
}
