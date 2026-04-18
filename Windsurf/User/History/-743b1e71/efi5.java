package com.jvmd.walto_backend.repo;

import com.jvmd.walto_backend.model.Donate;
import com.jvmd.walto_backend.model.EStatus;
import org.springframework.data.mongodb.repository.ReactiveMongoRepository;
import reactor.core.publisher.Flux;

public interface DonateRepo extends ReactiveMongoRepository<Donate, String> {
    Flux<Donate> findByMinecraftUsername(String minecraftUsername);
    Flux<Donate> findByStatus(EStatus status);
}
