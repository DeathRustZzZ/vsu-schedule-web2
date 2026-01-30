package com.vsuscheduleweb.services;

import com.vsuscheduleweb.DTO.ListTeacherResponse;
import com.vsuscheduleweb.entity.Teacher;
import com.vsuscheduleweb.mapper.TeacherMapper;
import com.vsuscheduleweb.repositories.TeacherRepository;
import lombok.RequiredArgsConstructor;
import org.jsoup.Connection;
import org.jsoup.Jsoup;
import org.jsoup.nodes.Document;
import org.jsoup.nodes.Element;
import org.jsoup.select.Elements;
import org.springframework.stereotype.Service;
import java.util.ArrayList;
import java.util.List;

@Service
@RequiredArgsConstructor
public class TeacherParserService {

    private final String BASE_URL = "https://vsu.by";
    private final String API_URL = "https://vsu.by/templates/vsutheme/api/Persons/alphabetPerson.php";
    private final TeacherRepository teacherRepository;
    private final TeacherMapper teacherMapper;


    public ListTeacherResponse parseTeachers() {
        List<Teacher> teachersThatHasBeenSaved = new ArrayList<>();
        String[] letters = {"А", "Б", "В", "Г", "Д", "Е", "Ё", "Ж", "З", "И", "Й", "К", "Л", "М", "Н", "О", "П", "Р", "С", "Т", "У", "Ф", "Х", "Ц", "Ч", "Ш", "Щ", "Ъ", "Ы", "Ь", "Э", "Ю", "Я"};

        int currentId = 0;

        for (String letter : letters) {
            try {
                Document doc = Jsoup.connect(API_URL)
                        .data("letter", letter)
                        .method(Connection.Method.POST)
                        .timeout(15000)
                        .post();

                Elements cards = doc.getElementsByClass("person_card");

                for (Element card : cards) {
                    String fio = card.getElementsByClass("fio_card").text().trim();
                    if (fio.isEmpty()) continue;

                    Teacher teacher = new Teacher();
                    teacher.setId(currentId++);
                    teacher.setFullname(fio);
                    teacher.setQualification("");

                    Element img = card.selectFirst("img");
                    if (img != null) {
                        teacher.setImgLink(BASE_URL + img.attr("src"));
                    }
                    teacher.setDescription(card.text().replace(fio, "").trim());
                    String[] fioParts = fio.split("\\s+");
                    try {
                        if (fioParts.length >= 1) teacher.setLastname(fioParts[0]);
                        if (fioParts.length >= 2) teacher.setFirstname(fioParts[1]);
                        if (fioParts.length >= 3) {
                            teacher.setSurname(fioParts[2]);
                            String initials = fioParts[1].charAt(0) + "." + fioParts[2].charAt(0) + ".";
                            teacher.setInitials(initials);
                        }
                    } catch (Exception e) {
                        if (teacher.getInitials() == null) teacher.setInitials("");
                    }
                    if(!teacherRepository.existsById(currentId)) {
                        teacherRepository.save(teacher);
                        teachersThatHasBeenSaved.add(teacher);
                    }

                }
            } catch (Exception e) {
                System.err.println("Ошибка при парсинге буквы " + letter + ": " + e.getMessage());
            }
        }
        return new ListTeacherResponse(teachersThatHasBeenSaved.stream().map(teacherMapper::entityToResponse).toList());
    }
}

