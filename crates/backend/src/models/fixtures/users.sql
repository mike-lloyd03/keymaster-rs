INSERT INTO users (username, display_name, email, password_hash, can_login, admin)
VALUES (
    'user1',
    'User Juan',
    'user@email.com',
    '46a9d5bde718bf366178313019f04a753bad00685d38e3ec81c8628f35dfcb1b',
    'f',
    'f'
),
(
    'userNoPass',
    'User NoPass',
    'usernopass@email.com',
    '',
    'f',
    'f'
),
(
    'userCanLogin',
    'User CanLogin',
    'usercanlog@email.com',
    'abc123',
    't',
    'f'
)
;
