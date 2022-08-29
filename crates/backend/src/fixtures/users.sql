INSERT INTO users (username, display_name, email, password_hash, can_login, admin)
VALUES (
    'user1',
    'User One',
    'user1@email.com',
    'pass1',
    'f',
    'f'
),
(
    'user2',
    'User Two',
    'user2@email.com',
    '$argon2i$v=19$m=65536,t=3,p=1$6JGByse/9Ous9DCnkgfFnA$lrixZa334c0rLb0k8SWK67q6TtSWoYjwXje67aKK0cU',
    't',
    'f'
),
(
    'user3',
    'User Three',
    'user3@email.com',
    'pass3',
    't',
    't'
)
;
