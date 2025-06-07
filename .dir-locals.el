;;; .dir-locals.el --- Directory-local variables for AoC workspace

;; This file sets up directory-local variables so that project commands
;; (like SPC + p + r) run from the year-specific directory rather than
;; the workspace root when working in year subdirectories.

((nil . ((eval . (let ((year-dir (locate-dominating-file default-directory "Cargo.toml")))
                   (when (and year-dir
                              (not (string= year-dir (projectile-project-root)))
                              (string-match "/\\([0-9]\\{4\\}\\)/$" year-dir))
                     ;; Set the project root to the year directory
                     (setq-local projectile-project-root year-dir)
                     ;; Also set compile command to run from year directory
                     (setq-local compile-command (format "cd %s && cargo run" year-dir))
                     ;; Set default directory for project commands
                     (setq-local default-directory year-dir)))))))

;; Alternative approach using directory-specific settings
;; Uncomment the sections below if you prefer explicit per-year configuration

;; ((rust-mode . ((projectile-project-root . nil)
;;                (eval . (setq-local projectile-project-root
;;                                    (locate-dominating-file default-directory "Cargo.toml"))))))

;; You can also add year-specific configurations like this:
;; (("2015/" . ((projectile-project-root . "./2015/")))
;;  ("2019/" . ((projectile-project-root . "./2019/")))
;;  ("2023/" . ((projectile-project-root . "./2023/")))
;;  ("2024/" . ((projectile-project-root . "./2024/"))))