package com.vsuscheduleweb.Controllers;


import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;



@Controller
@RequestMapping()
public class AdminController {


    @GetMapping(value = "/vsuAdmin")
    public String getAdminPage(){
        return "vsuAdminApp";
    }

    @GetMapping("/")
    public String getRoot(){
        return "redirect:/vsuAdmin";
    }

    @GetMapping("/login")
    public String getLoginPage(){
        return "login";
    }


}
