package com.vsuscheduleweb.security;

import org.assertj.core.api.Assertions;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.http.MediaType;
import org.springframework.mock.web.MockHttpSession;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.MvcResult;

import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.post;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.jsonPath;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

@SpringBootTest
@AutoConfigureMockMvc
class SecurityIntegrationTests {

    @Autowired
    private MockMvc mockMvc;

    @Test
    void loginSuccessReturnsOk() throws Exception {
        mockMvc.perform(post("/schedule/auth")
                        .contentType(MediaType.APPLICATION_JSON)
                        .content("{\"login\":\"admin\",\"password\":\"admin\"}"))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.ok").value(true));
    }

    @Test
    void adminPageIsProtectedWithoutLogin() throws Exception {
        MvcResult result = mockMvc.perform(get("/schedule/admin"))
                .andReturn();

        int status = result.getResponse().getStatus();
        Assertions.assertThat(status == 302 || status == 401).isTrue();
    }

    @Test
    void adminPageAccessibleAfterLogin() throws Exception {
        MvcResult loginResult = mockMvc.perform(post("/schedule/auth")
                        .contentType(MediaType.APPLICATION_JSON)
                        .content("{\"login\":\"admin\",\"password\":\"admin\"}"))
                .andExpect(status().isOk())
                .andReturn();

        MockHttpSession session = (MockHttpSession) loginResult.getRequest().getSession(false);

        mockMvc.perform(get("/schedule/admin").session(session))
                .andExpect(status().isOk());
    }

    @Test
    void logoutInvalidatesSession() throws Exception {
        MvcResult loginResult = mockMvc.perform(post("/schedule/auth")
                        .contentType(MediaType.APPLICATION_JSON)
                        .content("{\"login\":\"admin\",\"password\":\"admin\"}"))
                .andExpect(status().isOk())
                .andReturn();

        MockHttpSession session = (MockHttpSession) loginResult.getRequest().getSession(false);

        mockMvc.perform(post("/schedule/auth/logout").session(session))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.ok").value(true));

        MvcResult afterLogout = mockMvc.perform(get("/schedule/admin").session(session))
                .andReturn();
        int status = afterLogout.getResponse().getStatus();
        Assertions.assertThat(status == 302 || status == 401).isTrue();
    }
}
