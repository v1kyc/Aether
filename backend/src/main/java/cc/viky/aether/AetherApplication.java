package cc.viky.aether;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
public class AetherApplication {

	public static void main(String[] args) {
		SpringApplication.run(AetherApplication.class, args);
	}

}
