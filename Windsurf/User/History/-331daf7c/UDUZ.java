package com.jvmd.walto_backend.controller;

import com.jvmd.walto_backend.model.EStatus;
import com.jvmd.walto_backend.service.DonateService;
import com.jvmd.walto_backend.service.DonationEventService;
import com.jvmd.walto_backend.service.ExternalServiceIntegration;
import com.jvmd.walto_backend.service.PaymentGatewayService;
import lombok.AllArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Mono;

import java.time.LocalDateTime;
import java.util.Map;

@RestController
@RequestMapping("/api/v1/payments")
@AllArgsConstructor
@Slf4j
public class PaymentController {

    private final PaymentGatewayService paymentGatewayService;
    private final DonateService donateService;
    private final ExternalServiceIntegration externalServiceIntegration;
    private final DonationEventService donationEventService;

    @PostMapping("/callback")
    public Mono<Map<String, String>> paymentCallback(@RequestBody Map<String, Object> callbackData) {
        String paymentId = (String) callbackData.get("paymentId");
        String orderId = (String) callbackData.get("orderId");
        String status = (String) callbackData.get("status");
        
        log.info("Payment callback received: paymentId={}, orderId={}, status={}", paymentId, orderId, status);
        
        return paymentGatewayService.verifyPayment(paymentId)
                .flatMap(verifiedStatus -> {
                    if (verifiedStatus == EStatus.COMPLETED) {
                        return donateService.findById(orderId)
                                .flatMap(donate -> {
                                    donate.setStatus(EStatus.COMPLETED);
                                    donate.setProcessedAt(LocalDateTime.now());
                                    donate.setPaymentId(paymentId);
                                    return donateService.save(donate);
                                })
                                .flatMap(donate -> {
                                    String alertMessage = String.format("Платеж успешно обработан! Донат от %s (%s) на сумму $%d", 
                                        donate.getName(), donate.getMinecraftUsername(), donate.getPrice());
                                    return donationEventService.createDonationAlert(donate.getId(), alertMessage)
                                            .then(externalServiceIntegration.sendDonationToExternalService(donate));
                                })
                                .then(Mono.just(Map.of("status", "success", "message", "Payment processed successfully")));
                    } else {
                        return donateService.findById(orderId)
                                .flatMap(donate -> {
                                    donate.setStatus(verifiedStatus);
                                    return donateService.save(donate);
                                })
                                .then(Mono.just(Map.of("status", "updated", "message", "Payment status updated")));
                    }
                })
                .onErrorReturn(Map.of("status", "error", "message", "Failed to process payment"));
    }

    @GetMapping("/{paymentId}/status")
    public Mono<Map<String, Object>> getPaymentStatus(@PathVariable String paymentId) {
        return paymentGatewayService.verifyPayment(paymentId)
                .map(status -> Map.of(
                        "paymentId", paymentId,
                        "status", status.name(),
                        "timestamp", LocalDateTime.now()
                ));
    }
}
