FROM eclipse-temurin:17-jre-jammy
WORKDIR /schedule-service
COPY build/libs/*[^plain].jar /schedule-service/schedule-service-0.0.1-SNAPSHOT.jar
ENV LANG C.UTF-8
ENV LC_ALL C.UTF-8
ENTRYPOINT ["java", "-jar", "schedule-service-0.0.1-SNAPSHOT.jar"]