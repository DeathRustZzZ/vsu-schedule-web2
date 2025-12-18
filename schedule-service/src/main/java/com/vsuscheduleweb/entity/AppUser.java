package com.vsuscheduleweb.entity;


import com.vsuscheduleweb.security.Token.Token;
import com.vsuscheduleweb.security.roles.Role;
import jakarta.persistence.*;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.experimental.Accessors;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.core.userdetails.UserDetails;

import java.util.*;
import java.util.stream.Collectors;


@Entity
@Data
@Table(name = "app_user")
@NoArgsConstructor
@Accessors(chain = true)
public class AppUser implements UserDetails {
    @Id
    @Column(name = "user_id")
    private UUID id;
    private String name;
    private String lastname;
    private String status;
    private String email;
    private String login;
    @OneToMany(mappedBy = "user")
    private List<Token> tokens = new ArrayList<>();

    private String password;
    @Column(name = "created_at")
    private Date createdAt;

    public AppUser addToken(Token token){
        tokens.add(token);
        return this;
    }

    @ElementCollection(targetClass = Role.class,fetch = FetchType.EAGER)
    @JoinTable(name = "tbRole", joinColumns = @JoinColumn(name = "user_id"))
    @Enumerated(EnumType.STRING)
    @Column(name = "role", nullable = false)
    Set<Role> roles;



    @Override
    public Collection< ? extends GrantedAuthority> getAuthorities() {
        return roles.stream().map(role -> new SimpleGrantedAuthority(role.name())).collect(Collectors.toSet());
    }

    @Override
    public String getPassword() {
        return this.password;
    }

    @Override
    public String getUsername() {
        return this.login;
    }

    @Override
    public boolean isAccountNonExpired() {
        return true;
    }

    @Override
    public boolean isAccountNonLocked() {
        return true;
    }

    @Override
    public boolean isCredentialsNonExpired() {
        return true;
    }

    @Override
    public boolean isEnabled() {
        return true;
    }
}
