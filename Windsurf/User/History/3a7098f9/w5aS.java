package com.jvmd.lyceum_backend.config;

import com.jvmd.lyceum_backend.model.Role;
import com.jvmd.lyceum_backend.model.User;
import com.jvmd.lyceum_backend.repository.UserRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.CommandLineRunner;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Component;

import java.util.Set;

@Component
@RequiredArgsConstructor
@Slf4j
public class AdminInitializer implements CommandLineRunner {

    private final UserRepository userRepository;
    private final PasswordEncoder passwordEncoder;

    @Override
    public void run(String... args) {
        if (userRepository.existsByUsername("admin")) {
            log.info("Admin user already exists");
            return;
        }

        User admin = User.builder()
                .username("admin")
                .email("admin@lyceum.com")
                .password(passwordEncoder.encode("admin123"))
                .roles(Set.of(Role.ROLE_ADMIN, Role.ROLE_USER))
                .build();

        userRepository.save(admin);
        log.info("Admin user created successfully with username: admin and password: admin123");
    }
}
