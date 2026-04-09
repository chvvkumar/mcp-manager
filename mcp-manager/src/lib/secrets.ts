const SECRET_PATTERNS = [
  /^(sk|pk|api[_-]?key|token|secret|password|auth|bearer|access[_-]?token)/i,
  /key$/i, /token$/i, /secret$/i, /password$/i, /authorization/i,
];

const ENV_VAR_PATTERN = /^\$\{.+\}$/;

export function isSecretKey(key: string): boolean {
  return SECRET_PATTERNS.some((p) => p.test(key));
}

export function isEnvVarReference(value: string): boolean {
  return ENV_VAR_PATTERN.test(value);
}

export function maskValue(value: string): string {
  if (isEnvVarReference(value)) return value;
  if (value.length <= 8) return "****";
  return value.slice(0, 4) + "****" + value.slice(-4);
}

export function shouldMaskEntry(key: string, value: string): { masked: boolean; display: string } {
  if (isEnvVarReference(value)) return { masked: false, display: value };
  if (isSecretKey(key)) return { masked: true, display: maskValue(value) };
  return { masked: false, display: value };
}
