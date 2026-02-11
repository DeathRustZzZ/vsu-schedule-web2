# Быстрая справка: Docker для VSU Schedule Web

## 🔧 Одна команда для сборки всего стека

```bash
docker compose -f compose-env.yaml up --build
```

---

## 📦 Сборка отдельного сервиса

### Docker (из корня проекта):
```bash
docker build -f eurekaserver/Dockerfile -t eurekaserver:latest .
docker build -f api-gateway/Dockerfile -t api-gateway:latest .
docker build -f schedule-service/Dockerfile -t schedule-service:latest .
```

### Gradle локально:
```bash
# eurekaserver
cd eurekaserver && ./gradlew bootJar -x test

# api-gateway
cd api-gateway && ./gradlew bootJar -x test

# schedule-service
cd schedule-service && ./gradlew bootJar -x test
```

---

## 🚀 Запуск контейнера вручную

```bash
# eurekaserver (порт 8761)
docker run -p 8761:8761 eurekaserver:latest

# api-gateway (порт 8765)
docker run -p 8765:8765 api-gateway:latest

# schedule-service (порт 9898)
docker run -p 9898:9898 schedule-service:latest
```

---

## 🐛 Отладка

### Посмотреть логи сборки Docker
```bash
docker build -f eurekaserver/Dockerfile -t eurekaserver:test . --progress=plain
```

### Посмотреть логи работающего контейнера
```bash
docker logs <container_id>
docker logs -f <container_id>  # tail mode
```

### Запустить контейнер с bash вместо jar
```bash
docker run -it eurekaserver:latest /bin/sh
```

### Проверить, что jar внутри контейнера
```bash
docker run --rm eurekaserver:latest ls -lh /app/
```

---

## 📝 Что было исправлено

✅ Build context изменён на корень проекта (`.`)  
✅ Используется `bootJar` вместо `build`  
✅ Добавлены главные классы Spring Boot  
✅ Явно указаны mainClass в build.gradle  
✅ Обновлена compose-env.yaml с правильными путями  
✅ Исправлены glob-паттерны для jar'ов  
✅ Добавлен gradle wrapper в schedule-service  

---

## ❓ FAQ

**Q: Почему build context — корень, а не модуль?**  
A: Потому что это мультимодульный проект. Gradle в каждом модуле нужен доступ к градлю из этого же модуля, но контекст сборки — всегда корень.

**Q: Почему используется `bootJar`, а не `build`?**  
A: `build` создаёт несколько артефактов (jar, war, etc). `bootJar` — специфически создаёт исполняемый Spring Boot jar с Main-Class в манифесте.

**Q: Почему ./gradlew, а не gradle?**  
A: Чтобы гарантировать версию Gradle (8-jdk17 в образе). gradlew — gradle wrapper, использует локальную версию из gradle/.

**Q: Где находятся главные классы?**  
A: Они созданы по пути:
- `eurekaserver/src/main/java/com/vsu_schedule/eurekaserver/EurekaServerApplication.java`
- `api-gateway/src/main/java/com/vsu_schedule/api_gateway/ApiGatewayApplication.java`
- `schedule-service/src/main/java/com/vsuscheduleweb/ScheduleServiceApplication.java`

**Q: Как мне скоммитить эти изменения?**  
A:
```bash
git add eurekaserver/src/main/java/com/vsu_schedule/eurekaserver/EurekaServerApplication.java
git add api-gateway/src/main/java/com/vsu_schedule/api_gateway/ApiGatewayApplication.java
git add schedule-service/src/main/java/com/vsuscheduleweb/ScheduleServiceApplication.java
git add schedule-service/gradlew schedule-service/gradlew.bat schedule-service/gradle/
git add eurekaserver/Dockerfile api-gateway/Dockerfile schedule-service/Dockerfile
git add compose-env.yaml
git add eurekaserver/build.gradle api-gateway/build.gradle schedule-service/build.gradle
git commit -m "Fix: мультимодульная Docker-сборка для Gradle Spring Boot проекта"
```
