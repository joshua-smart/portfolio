-- Add up migration script here

INSERT INTO tools VALUES
(0, 'rust', 'https://www.rust-lang.org'),
(1, 'html', 'https://developer.mozilla.org/en-US/docs/Web/HTML'),
(2, 'css', 'https://developer.mozilla.org/en-US/docs/Web/CSS'),
(3, 'tailwindcss', 'https://tailwindcss.com/'),
(4, 'askama', 'https://github.com/djc/askama'),
(5, 'axum', 'https://github.com/tokio-rs/axum');

INSERT INTO projects VALUES 
(0, 'portfolio', 0),
(1, 'something-else', 1),
(2, 'a-third-thing', 1);

INSERT INTO project_tools VALUES
(0, 0),
(0, 1),
(0, 2),
(0, 3),
(0, 4),
(0, 5),
(1, 0);

INSERT INTO sources VALUES
(0, 'github', 'https://github.com/joshua-smart/portfolio'),
(1, 'github', 'a-different-thing');

INSERT INTO posts VALUES
(0, 'title-to-a-page', '## Heading 2' || X'0A' || 'Some sample text' || X'0A' || '- List item 1' || X'0A' || '- **List** item 2', '2024-07-24 23:56:15.000');
