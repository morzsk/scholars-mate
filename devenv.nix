{ config, pkgs, ... }:

{
  languages.rust.enable = true;

  packages = [ pkgs.cargo-watch ];

  services.postgres = {
    enable = true;
    package = pkgs.postgresql_18;
    initialDatabases = [
      {
        name = "scholars_mate";
        user = "scholars_mate";
      }
    ];
  };

  env.DATABASE_URL =
    "postgres://scholars_mate@localhost/scholars_mate?host=${config.env.PGHOST}";
}
