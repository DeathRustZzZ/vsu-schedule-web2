FROM eclipse-temurin:17-jre-jammy
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /schedule-service
 COPY scriptspython/requirements.txt .
 RUN ls -l requirements.txt
 RUN pip3 install --no-cache-dir -r requirements.txt
COPY build/libs/*[^plain].jar /schedule-service/schedule-service-0.0.1-SNAPSHOT.jar
COPY scriptspython/ /schedule-service/scriptspython/
ENV LANG C.UTF-8
ENV LC_ALL C.UTF-8
ENTRYPOINT ["java", "-jar", "schedule-service-0.0.1-SNAPSHOT.jar"]