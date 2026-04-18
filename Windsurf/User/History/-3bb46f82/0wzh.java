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
public class Donate {
    @Id
    private String id;
    private String name;
    private Integer price;
    private String whoDonate;
    private String minecraftUsername;
    private String email;
    private EDonateType donateType;
    private String message;
    private LocalDateTime createdAt;
    private LocalDateTime processedAt;
    private EStatus status;
    private String paymentId;
    private String transactionHash;
}
