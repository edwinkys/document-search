export function validEmail(email: string): boolean {
  // Not a perfect email validation, but good enough for now.
  return email.length > 0 && email.includes("@") && email.includes(".")
}
