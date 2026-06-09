export interface FirewallRule {
  name: string;
  applicationPath: string;
}

export interface RuleConvertOptions {
  inputTarget?: "mihomo" | "general" | "egern" | "sing-box";
  inputFormat?:
    | "yaml"
    | "mrs"
    | "text"
    | "json"
    | "srs"
    | "domainset"
    | "ruleset"
    | "ipset";
  inputBehavior?: "auto" | "domain" | "ip" | "classical";
  outputTarget?: "mihomo" | "general" | "egern" | "sing-box";
  outputFormat?:
    | "mrs"
    | "text"
    | "yaml"
    | "json"
    | "srs"
    | "domainset"
    | "ruleset"
    | "ipset";
  outputBehavior?: "auto" | "domain" | "ip" | "classical";
}

export interface RuleOutputInfo {
  behavior?: string;
  format: string;
  count: number;
}

export interface RuleSkippedItem {
  rule: string;
  reason: string;
}

export interface RuleStringResult {
  kind: "rules";
  outputs: Record<string, string>;
  info: Record<string, RuleOutputInfo>;
  skipped: RuleSkippedItem[];
}

export function fileToDataUrl(path: string): string;
export function fileToStr(
  path: string,
  options?: RuleConvertOptions | null,
): RuleStringResult;
export function getAppName(path: string): string;
export function getCurrentUserSid(): string;
export function isRunningAsAdmin(): boolean;
export function runElevated(command: string, args?: string[]): number;
export function setupFirewallRules(rules: FirewallRule[]): void;
