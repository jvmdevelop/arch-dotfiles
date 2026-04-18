package com.jvmd.mh_backend.controller;

import com.jvmd.mh_backend.model.AiMessage;
import com.jvmd.mh_backend.service.rag.RagService;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.ResponseEntity;
import org.springframework.messaging.handler.annotation.MessageMapping;
import org.springframework.messaging.handler.annotation.SendTo;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.*;

import java.util.Map;

@Slf4j
@Controller
public class ChatController {
    private final RagService ragService;

    public ChatController(RagService ragService) {
        this.ragService = ragService;
    }

    @MessageMapping("/chat")
    @SendTo("/topic/messages")
    public AiMessage handleMessage(String message) {
        try {
            return ragService.handleMessage(message);
        } catch (Exception ex) {
            log.error("Error handling message: {}", ex.getMessage());
            return AiMessage.builder()
                    .content("Error processing your message: " + ex.getMessage())
                    .build();
        }
    }
    
    @PostMapping("/api/chat/send")
    public ResponseEntity<AiMessage> sendMessage(@RequestBody Map<String, String> request) {
        String message = request.get("message");
        if (message == null || message.trim().isEmpty()) {
            return ResponseEntity.badRequest().build();
        }
        
        try {
            AiMessage response = ragService.handleMessage(message);
            return ResponseEntity.ok(response);
        } catch (Exception ex) {
            log.error("Error sending message: {}", ex.getMessage());
            return ResponseEntity.internalServerError().build();
        }
    }
}
