package com.jvmd.uownserver.controller;

import com.jvmd.uownserver.config.PluginConfig;

import java.io.*;
import java.net.ServerSocket;
import java.net.Socket;

public interface Controller {



    String getResponse(String path);

    default String getName(){
       return this.getClass().getSimpleName();
    }

    static ServerSocket getServerSocket() throws IOException {
        return new ServerSocket(PluginConfig.getServerConfig().getPort());
    }

    default void openConnection() {
        Thread.ofVirtual().start(() -> {
            try (Socket socket = getServerSocket().accept();
                 BufferedReader in = new BufferedReader(new InputStreamReader(socket.getInputStream()));
                 BufferedWriter out = new BufferedWriter(new OutputStreamWriter(socket.getOutputStream()))
            ) {
                StringBuilder sb = new StringBuilder();
                while (in.readLine() != null) {
                    sb.append(in.readLine());
                }
                String response = getResponse(sb.toString());
                out.write(response);

            } catch (IOException e) {
                throw new RuntimeException(e);
            }
        });
    }


}
