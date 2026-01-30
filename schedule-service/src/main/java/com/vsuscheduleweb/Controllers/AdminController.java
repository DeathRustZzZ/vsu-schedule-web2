package com.vsuscheduleweb.Controllers;


import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;



@Controller
@RequestMapping("/schedule")
public class AdminController {


    @GetMapping(value = "/admin")
    public String getAdminPage(){
        return "vsuAdminApp";
    }

    @GetMapping("/")
    public String getRoot(){
        return "redirect:/schedule/admin";
    }

    @GetMapping("/login")
    public String getLoginPage(){
        return "login";
    }


}
