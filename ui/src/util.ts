export const trunc = (s: string, n: number) => {
    return (s && s.length > n) ? s.slice(0, n - 1) + "…" : (s || "");
}