import requests
from bs4 import BeautifulSoup
import json
import os
import shutil
from pathlib import *

letters = ["А", "Б", "В", "Г", "Д", "Е", "Ё", "Ж", "З", "И",
            "Й", "К", "Л", "М", "Н", "О", "П", "Р", "С", "Т", "У", "Ф", "Х",
              "Ц", "Ч", "Ш", "Щ", "Ъ", "Ы", "Ь", "Э", "Ю", "Я"]

link = "https://vsu.by"

i = 0

try:
    os.mkdir("src/main/scriptspython/teachers")
except FileExistsError:
    shutil.rmtree("src/main/scriptspython/teachers")
    os.mkdir("src/main/scriptspython/teachers")

class Person:

    __firstName : str 
    __lastName : str 
    __surname : str 
    __initials : str 
    __discription : str 
    __imgLink : str
    __fullname : str
    __id: int
    __qualification: str
    def __init__(self, firstName = "", lastname = "", surname = "", 
                 initials = "", description = "", imgLink = "", fullname = "", id = 0) -> None:
        self.__firstname = firstName
        self.__lastname = lastname
        self.__surname = surname
        self.__initials = initials
        self.__description = description
        self.__imgLink = imgLink
        self.__fullname = fullname
        self.__id = id
        self.__qualification = ""


    def setFirstname(self, firstname: str) -> None:
        self.firstname = firstname

    def setLastName(self, lastname: str) -> None:
        self.__lastname = lastname

    def setSurname(self,surname: str) -> None:
        self.__surname = surname 
    
    def setInitials(self, initials: str) -> None:
        self.__initials = initials

    def setDescription(self, description: str) -> None:
        self.__description = description
    
    def setImgLink(self, imgLink : str) -> None:
        self.__imgLink = imgLink

    def setFullname(self, fullname: str) -> None:
        self.__fullname = fullname

    def setId(self, id: int) -> None:
        self.__id = id

    def getLastname(self) -> str:
        return self.__lastname
    
    def getInitials(self) -> str:
        return self.__initials
    
    def getFullname(self) -> str:
        return self.__fullname

    def getId(self) -> int:
        return self.__id
    
    def getPersonDict(self) -> dict:
        return {
            "id":self.__id,
            "firstname":self.__firstname,
            "lastname":self.__lastname,
            "surname":self.__surname,
            "initials": self.__initials,
            "imgLink":self.__imgLink,
            "description" : self.__description,
            "fullname" : self.__fullname,
            "qualification" : self.__qualification,

        }   
        

def save(person: Person) -> None:
    with open(f"src/main/scriptspython/teachers\\{person.getId()}.json","w") as file:
            json.dump(person.getPersonDict(),file)



def parse(letter: str )-> None:
    resp = requests.post("https://vsu.by/templates/vsutheme/api/Persons/alphabetPerson.php",data={
        "letter":letter
    })
    soup = BeautifulSoup(resp.content,"html.parser")
    personsList = soup.find_all(class_= "person_card")
    global i
    for personCard in personsList:
        person = Person()
        fio = personCard.find(class_ = "fio_card").text
        if fio != "":
            fioSplit = fio.split(" ")
            try:
                person.setImgLink(link + personCard.find("img")["src"])
                person.setFullname(personCard.find(class_ = "fio_card").text)
                person.setFirstname(str(fioSplit[1]))
                person.setSurname(fioSplit[2])

                person.setLastName(str(fioSplit[0]))
                person.setDescription(str(personCard.text.replace(fio,"")))

                person.setInitials(f"{fioSplit[1][0]}.{fioSplit[2][0]}.")

            except IndexError:
                person.setLastName(fioSplit[0])
                person.setFirstname(fioSplit[1])
                person.setDescription(str(personCard.text.replace(fio,"")))
                
            finally:
                person.setId(i)
                save(person)
                i += 1

if __name__ == "__main__":
    for letter in letters:
        parse(letter)
    print("1")