package com.jvmd.uownserver.config;

import lombok.AllArgsConstructor;
import lombok.Data;

@Data
@AllArgsConstructor
public final class ServerConfig {

    private  String host;
    private  int port;
    private  String password;
    private  String trustedIp;


}
