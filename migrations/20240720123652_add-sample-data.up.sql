-- Add up migration script here

INSERT INTO projects VALUES 
(0, 'portfolio', 'A personal portfolio site and blog. Built in [Rust](https://www.rust-lang.org) using the [Axum](https://github.com/tokio-rs/axum) web framework, [Askama](https://github.com/djc/askama) HTML templating, and [Tailwind](https://tailwindcss.com/) CSS styles.', 'github', 'https://github.com/joshua-smart/portfolio'),
(1, 'nixos', 'Personal [NixOS](https://nixos.org) and [home-manager](https://github.com/nix-community/home-manager) configurations, a declarative model for reliable and reproducible environments.', 'github', 'https://github.com/joshua-smart/nixos'),
(2, 'nix-homelab', 'Homelab configuration built and deployed using [NixOS](https://nixos.org), hosting a variety of services such as [paperless-ngx](https://docs.paperless-ngx.com) and [wireguard](https://www.wireguard.com).', 'github', 'https://github.com/joshua-smart/nix-homelab'),
(3, 'sales-report-generator', 'Tool to extract and collate sales of a small business from a POS system.', 'github', 'https://github.com/joshua-smart/sales-report-generator');

INSERT INTO posts VALUES
(0, 'title-to-a-page', '## Heading 2' || X'0A' || 'Some sample text' || X'0A' || '- List item 1' || X'0A' || '- **List** item 2', '2024-07-24'),
(1, 'An exploration into effect systems and the function colouring problem.', '## Heading 2' || X'0A' || 'Some sample text' || X'0A' || '- List item 1' || X'0A' || '- **List** item 2', '2024-07-25');

INSERT INTO events VALUES
(0, 'December 2021', 'First commercial software project, a tool to extract and collate sales of a small business from a POS system.'),
(1, 'June 2022', 'A\*A\*A\* in Computer Science, Mathematics, and Further Maths A-Levels'),
(2, 'July 2022', 'Internship at [Beran Instruments Ltd](https://www.cmtg.com/beran) working in [TypeScript](https://www.typescriptlang.org/), [React](https://react.dev/), [C++](https://isocpp.org/), and [C#](https://learn.microsoft.com/en-us/dotnet/csharp/).'),
(3, 'September 2022', 'Began studying [Software Engineering MEng](https://www.southampton.ac.uk/courses/software-engineering-degree-meng) at the [University of Southampton](https://www.southampton.ac.uk/).'),
(4, 'March 2023', 'Created JSmart Software Development Ltd to facilitate contract work.'),
(5, 'June 2023', 'Continued work with [Beran Instruments Ltd](https://www.cmtg.com/beran) in a contractor position, working in [TypeScript](https://www.typescriptlang.org/), [React](https://react.dev/), and [Rust](https://www.rust-lang.org/).'),
(6, 'September 2024', 'Awarded the [netcraft](https://www.netcraft.com/) prize for top performance in Computer Science at the [University of Southampton](https://www.southampton.ac.uk/).');
