package com.jvmd.walto_backend.service;

import com.jvmd.walto_backend.model.Donate;
import com.jvmd.walto_backend.repo.DonateRepo;
import lombok.AllArgsConstructor;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

@Service
@AllArgsConstructor
public class DonateService {
    private final DonateRepo repo;

    public Mono<Donate> save(Donate donate) {
        return repo.save(donate);
    }

    public Flux<Donate> findAll(){
        return repo.findAll();
    }

    public Mono<Donate> findById(String id) {
        return repo.findById(id);
    }

    public Flux<Donate> findByMinecraftUsername(String minecraftUsername) {
        return repo.findByMinecraftUsername(minecraftUsername);
    }

    public Flux<Donate> findByStatus(com.jvmd.walto_backend.model.EStatus status) {
        return repo.findByStatus(status);
    }
}
