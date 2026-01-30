package com.vsuscheduleweb;


import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.client.discovery.EnableDiscoveryClient;


@SpringBootApplication
@EnableDiscoveryClient
public class VsuScheduleWebApplication {

	public static void main(String[] args)  {
		SpringApplication.run(VsuScheduleWebApplication.class, args);

	}

}
