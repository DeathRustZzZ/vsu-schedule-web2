import pathlib
import requests
from pathlib import Path



def getLinkAndToken():
    path = Path(pathlib.Path.cwd(),'application.yaml')
    paramlist = []
    with open(path) as file:
        for line in file:
            if 'path' in line or 'token' in line:

                paramlist.append(str(line.strip(" ").split(" ")[1]))
    print(paramlist)
    paramlist.pop(0)

    return paramlist

def regTelegramWebHook(linkFromNgrok,token):
    print(linkFromNgrok)
    link = "https://api.telegram.org/bot"+token+"/setwebhook?url="+linkFromNgrok.strip()+"/bots/index.php"
    response = requests.get(link)
    valueslist = []
    for item in response.json().values():
        valueslist.append(item)
    if valueslist[0]:
        print("all goooood.")
    elif not valueslist[0]:
        print(response.text)
        print("false")
    else:
        print(response.text)
        print('????☹️☹️')

def main():

    linkAndToken = getLinkAndToken()
    link = 0
    token = 0
    print(linkAndToken)
    for item in linkAndToken:
        if 'http' in item:
            link = item
        else:
            token = item

    regTelegramWebHook(link,token)





if __name__ == "__main__":
    main()