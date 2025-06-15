# SPDX-FileCopyrightText: 2025 Maximilian Marx
#
# SPDX-License-Identifier: EUPL-1.2

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
    ;
in
{
  options.die-koma.aksync = {
    enable = mkEnableOption "AK synchronisation from aktool to KoMapedia";

    passwordFile = mkOption {
      description = "File containing the KoMapedia bot password";
      type = types.path;
    };

    onCalendar = mkOption {
      description = "When to run aksync";
      type = types.listOf types.str;
    };
  };

  config =
    let
      cfg = config.die-koma.aksync;
    in
    mkIf cfg.enable {

      systemd = {
        services.aksync = {
          after = [
            "network.target"
            "nginx.service"
            "phpfpm-mediawiki.service"
          ];
          reloadTriggers = [ ];
          serviceConfig = {
            DynamicUser = true;
            ExecStart = "${lib.getExe pkgs.aksync}";
            Type = "oneshot";
            LoadCredential = [
              "AKSYNC_BOT_PASSWORD:${cfg.passwordFile}"
            ];
          };
        };

        timers.aksync = {
          timerConfig = {
            OnCalendar = cfg.onCalendar;
            Unit = "aksync.service";
          };
        };
      };
    };
}
