package com.jvmd.uownserver.controller;

import com.jvmd.uownserver.config.PluginConfig;
import lombok.extern.slf4j.Slf4j;
import spark.Service;

import static spark.Spark.*;

@Slf4j
public interface Controller {

    String getResponse(String path);

    default String getName() {
        return this.getClass().getSimpleName();
    }

    default void startServer() {
        port(PluginConfig.getServerConfig().getPort());
        
        get("/api/v1/user/own", (req, res) -> {
            String username = req.queryParams("username");
            if (username == null) {
                res.status(400);
                return "Missing username param";
            }
            return getResponse("/api/v1/user/own?username=" + username);
        });

        post("/api/v1/donate", (req, res) -> {
            String nickname = req.queryParams("username");
            String userDonate = req.queryParams("donate");
            
            if (nickname == null || userDonate == null) {
                res.status(400);
                return "Missing required params: username, donate";
            }
            
            return getResponse("/api/v1/donate?username=" + nickname + "&donate=" + userDonate);
        });

        log.info("Internal server started on port: {}", PluginConfig.getServerConfig().getPort());
    }

    default void stopServer() {
        stop();
        log.info("Internal server stopped");
    }
}
