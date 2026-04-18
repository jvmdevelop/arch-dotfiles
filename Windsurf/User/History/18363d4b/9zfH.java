package com.jvmd.walto_backend.model;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.springframework.data.annotation.Id;
import org.springframework.data.mongodb.core.mapping.Document;

import java.time.LocalDateTime;

@AllArgsConstructor
@NoArgsConstructor
@Builder
@Data
@Document
public class DonationEvent {
    @Id
    private String id;
    private String donationId;
    private String type;
    private String message;
    private boolean acknowledged;
    private LocalDateTime createdAt;
    private LocalDateTime acknowledgedAt;
}

