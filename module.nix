{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib)
    mkEnableOption
    mkIf
    mkOption
    types
    concatStringsSep
    ;
  cfg = config.services.portfolio;
  args = concatStringsSep " " [
    "--port ${toString cfg.port}"
    "--address ${cfg.address}"
    "--asset-dir ${pkgs.portfolio}/assets"
    "--data-path \${STATE_DIRECTORY}/data.ron"
  ];
in
{
  options.services.portfolio = {
    enable = mkEnableOption "portfolio";
    port = mkOption {
      type = types.port;
      default = 3000;
    };
    address = mkOption {
      type = types.str;
      default = "0.0.0.0";
    };
    openFirewall = mkOption {
      type = types.bool;
      default = false;
    };
  };
  config = mkIf cfg.enable {
    networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [ cfg.port ];

    systemd.services.portfolio = {
      description = "Personal portfolio website.";
      serviceConfig = {
        ExecStart = "${pkgs.portfolio}/bin/portfolio ${args}";
        StateDirectory = "portfolio";
      };

      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];
      unitConfig = {
        StartLimitIntervalSec = 5;
        StartLimitBurst = 10;
      };
    };
  };
}
