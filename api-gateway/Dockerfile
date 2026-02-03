FROM gradle:8-jdk17 AS builder

WORKDIR /home/gradle/src
COPY build.gradle settings.gradle ./
COPY src ./src
RUN gradle build -x test

FROM eclipse-temurin:17-jre-jammy
WORKDIR /api-gateway
COPY --from=builder /home/gradle/src/build/libs/*[^plain].jar /api-gateway/api-gateway-0.0.1-SNAPSHOT.jar
ENV LANG C.UTF-8
ENV LC_ALL C.UTF-8
ENTRYPOINT ["java", "-jar", "api-gateway-0.0.1-SNAPSHOT.jar"]
