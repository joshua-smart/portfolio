-- Add up migration script here

INSERT INTO tools VALUES
(0, 'rust', 'https://www.rust-lang.org'),
(1, 'html', 'https://developer.mozilla.org/en-US/docs/Web/HTML'),
(2, 'css', 'https://developer.mozilla.org/en-US/docs/Web/CSS#'),
(3, 'tailwindcss', 'https://tailwindcss.com/'),
(4, 'askama', 'https://github.com/djc/askama'),
(5, 'axum', 'https://github.com/tokio-rs/axum');

INSERT INTO projects VALUES 
(0, 'portfolio'),
(1, 'something-else'),
(2, 'a-third-thing');

INSERT INTO project_tools VALUES
(0, 0),
(0, 1),
(0, 2),
(0, 3),
(0, 4),
(0, 5),
(1, 0);
