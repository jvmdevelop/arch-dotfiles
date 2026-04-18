package com.jvmdevelop.strife.controller;


import org.kurento.client.*;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;
import org.springframework.web.socket.*;
import org.springframework.web.socket.handler.TextWebSocketHandler;

import java.io.IOException;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

@Component
public class WebRtcHandler extends TextWebSocketHandler {
    private static final KurentoClient kurentoClient = KurentoClient.create("ws://localhost:8888/kurento");
    public static final ConcurrentMap<String, MediaPipeline> channels = new ConcurrentHashMap<>();
    public static final ConcurrentMap<String, ConcurrentMap<String, WebRtcEndpoint>> users = new ConcurrentHashMap<>();
    private static final ScheduledExecutorService pingScheduler = Executors.newScheduledThreadPool(1);
    private static final Logger log = LoggerFactory.getLogger(WebRtcHandler.class);

    @Override
    public void afterConnectionEstablished(WebSocketSession session) throws Exception {
        String channel = (String) session.getAttributes().get("channel");
        String user = (String) session.getAttributes().get("user");

        channels.computeIfAbsent(channel, c -> kurentoClient.createMediaPipeline());
        users.computeIfAbsent(channel, c -> new ConcurrentHashMap<>());

        MediaPipeline pipeline = channels.get(channel);
        WebRtcEndpoint webRtcEndpoint = new WebRtcEndpoint.Builder(pipeline).build();

        webRtcEndpoint.addIceCandidateFoundListener(event -> {
            try {
                session.sendMessage(new TextMessage(event.getCandidate().toString()));
            } catch (Exception e) {
                e.printStackTrace();
            }
        });

        users.get(channel).put(user, webRtcEndpoint);
        log.info("User connected: {} to channel: {}", user, channel);
    }

    @Override
    public void handleTextMessage(WebSocketSession session, TextMessage message) throws Exception {
        String channel = (String) session.getAttributes().get("channel");
        String user = (String) session.getAttributes().get("user");
        String payload = message.getPayload();

        if (payload.startsWith("PING")) {
            try {
                long serverTimestamp = System.currentTimeMillis();
                session.sendMessage(new TextMessage("PONG:" + serverTimestamp));
            } catch (Exception e) {
                e.printStackTrace();
            }
            return;
        }

        WebRtcEndpoint webRtcEndpoint = users.get(channel).get(user);

        String sdpAnswer = webRtcEndpoint.processOffer(payload);
        try {
            session.sendMessage(new TextMessage(sdpAnswer));
        } catch (Exception e) {
            e.printStackTrace();
        }

        webRtcEndpoint.gatherCandidates();
    }

    @Override
    public void afterConnectionClosed(WebSocketSession session, CloseStatus status) throws Exception {
        String channel = (String) session.getAttributes().get("channel");
        String user = (String) session.getAttributes().get("user");

        WebRtcEndpoint webRtcEndpoint = users.get(channel).remove(user);
        if (webRtcEndpoint != null) {
            webRtcEndpoint.release();
        }

        if (users.get(channel).isEmpty()) {
            channels.get(channel).release();
            channels.remove(channel);
            users.remove(channel);
        }

        log.info("User disconnected: {} from channel: {}", user, channel);
    }

    public void connectPeers(String channel) {
        ConcurrentMap<String, WebRtcEndpoint> channelUsers = users.get(channel);
        if (channelUsers == null || channelUsers.size() < 2) {
            return;
        }

        channelUsers.values().forEach(endpoint1 ->
                channelUsers.values().forEach(endpoint2 -> {
                    if (endpoint1 != endpoint2) {
                        endpoint1.connect(endpoint2);
                    }
                })
        );
        log.info("All users in channel {} connected.", channel);
    }

    public String getChannelInfo(String channel) {
        if (!users.containsKey(channel)) {
            return "Channel not found.";
        }

        StringBuilder info = new StringBuilder("Users in channel " + channel + ":\n");
        users.get(channel).forEach((user, endpoint) -> info.append(user).append("\n"));
        return info.toString();
    }

    private void sendMessage(WebSocketSession session, String message) {
        try {
            session.sendMessage(new TextMessage(message));
        } catch (IOException e) {
            log.error("Error sending message to Session: {} - {}", session.getId(), e.getMessage(), e);
        }
    }

    static {
        pingScheduler.scheduleAtFixedRate(() -> {
            users.forEach((channel, channelUsers) -> channelUsers.forEach((user, endpoint) -> {
            }));
        }, 0, 30, TimeUnit.SECONDS);
    }
}