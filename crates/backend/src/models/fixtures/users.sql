INSERT INTO users (username, display_name, email, password_hash, can_login)
VALUES (
    'user1',
    'User Juan',
    'user@email.com',
    '46a9d5bde718bf366178313019f04a753bad00685d38e3ec81c8628f35dfcb1b',
    'f'
),
(
    'userNoPass',
    'User NoPass',
    'usernopass@email.com',
    '',
    'f'
)
;
