package com.jvmd.walto_backend.controller;

import com.jvmd.walto_backend.dto.DonationRequest;
import com.jvmd.walto_backend.dto.DonationResponse;
import com.jvmd.walto_backend.model.Donate;
import com.jvmd.walto_backend.model.EDonateType;
import com.jvmd.walto_backend.model.EStatus;
import com.jvmd.walto_backend.service.DonateService;
import com.jvmd.walto_backend.service.ExternalServiceIntegration;
import com.jvmd.walto_backend.service.PaymentGatewayService;
import lombok.AllArgsConstructor;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.time.LocalDateTime;

@RestController
@RequestMapping("/api/v1/donations")
@AllArgsConstructor
public class DonationController {

    private final DonateService donateService;
    private final PaymentGatewayService paymentGatewayService;
    private final ExternalServiceIntegration externalServiceIntegration;

    @PostMapping
    public Mono<DonationResponse> create(@RequestBody DonationRequest request) {
        return Mono.defer(() -> {
            Integer price = request.getDonateType() == EDonateType.CUSTOM_AMOUNT 
                ? request.getCustomAmount() 
                : request.getDonateType().getDefaultPrice();
            
            Donate donate = Donate.builder()
                    .name(request.getName())
                    .minecraftUsername(request.getMinecraftUsername())
                    .email(request.getEmail())
                    .donateType(request.getDonateType())
                    .price(price)
                    .message(request.getMessage())
                    .status(EStatus.PENDING)
                    .createdAt(LocalDateTime.now())
                    .build();
            
            return donateService.save(donate);
        }).flatMap(saved -> 
            paymentGatewayService.createPayment(saved)
                .map(paymentUrl -> DonationResponse.builder()
                        .id(saved.getId())
                        .name(saved.getName())
                        .price(saved.getPrice())
                        .minecraftUsername(saved.getMinecraftUsername())
                        .donateType(saved.getDonateType())
                        .message(saved.getMessage())
                        .createdAt(saved.getCreatedAt())
                        .status(saved.getStatus())
                        .paymentUrl(paymentUrl)
                        .paymentId(saved.getId())
                        .build())
        );
    }

    @GetMapping
    public Flux<DonationResponse> findAll() {
        return donateService.findAll()
                .map(donate -> DonationResponse.builder()
                        .id(donate.getId())
                        .name(donate.getName())
                        .price(donate.getPrice())
                        .minecraftUsername(donate.getMinecraftUsername())
                        .donateType(donate.getDonateType())
                        .message(donate.getMessage())
                        .createdAt(donate.getCreatedAt())
                        .status(donate.getStatus())
                        .build());
    }

    @GetMapping("/types")
    public Flux<DonateTypeResponse> getTypes() {
        return Flux.fromArray(EDonateType.values())
                .map(type -> new DonateTypeResponse(type.name(), type.getDisplayName(), type.getDefaultPrice()));
    }

    @GetMapping("/{id}")
    public Mono<DonationResponse> findById(@PathVariable String id) {
        return donateService.findById(id)
                .map(donate -> DonationResponse.builder()
                        .id(donate.getId())
                        .name(donate.getName())
                        .price(donate.getPrice())
                        .minecraftUsername(donate.getMinecraftUsername())
                        .donateType(donate.getDonateType())
                        .message(donate.getMessage())
                        .createdAt(donate.getCreatedAt())
                        .status(donate.getStatus())
                        .build());
    }

    @PostMapping("/{id}/complete")
    public Mono<Void> completeDonation(@PathVariable String id) {
        return donateService.findById(id)
                .flatMap(donate -> {
                    donate.setStatus(EStatus.COMPLETED);
                    donate.setProcessedAt(LocalDateTime.now());
                    return donateService.save(donate);
                })
                .flatMap(donate -> externalServiceIntegration.sendDonationToExternalService(donate));
    }

    @lombok.Data
    @lombok.AllArgsConstructor
    public static class DonateTypeResponse {
        private String name;
        private String displayName;
        private Integer defaultPrice;
    }
}
