import type { TransitionConfig } from "svelte/transition";

const reducedMotion =
  typeof matchMedia === "function"
    ? matchMedia("(prefers-reduced-motion: reduce)")
    : null;

/**
 * Returns true if the user's OS has requested reduced motion.
 */
export function prefersReducedMotion(): boolean {
  return reducedMotion?.matches ?? false;
}

function duration(ms: number): number {
  return prefersReducedMotion() ? 0 : ms;
}

/**
 * Damped spring curve with ~3% overshoot, normalized to land on 1.
 */
export function springOut(t: number): number {
  return t >= 1 ? 1 : 1 - Math.exp(-7 * t) * Math.cos(5 * t);
}

/**
 * Exponential ease-out for quick exits.
 */
export function expoOut(t: number): number {
  return t >= 1 ? 1 : 1 - 2 ** (-10 * t);
}

export type TransitionFn = (node: Element) => TransitionConfig;

/**
 * Tactical dossier card or modal entering with a damped spring.
 */
export const popIn: TransitionFn = () => ({
  duration: duration(380),
  easing: springOut,
  css: (t, u) =>
    `opacity: ${Math.min(1, t * 2.2)}; transform: translateY(${u * 8}px) scale(${0.98 + 0.02 * t});`,
});

/**
 * Quick, smooth exit for popped elements.
 */
export const popOut: TransitionFn = () => ({
  duration: duration(150),
  easing: expoOut,
  css: (t) => `opacity: ${t}; transform: scale(${0.98 + 0.02 * t});`,
});

/**
 * Soft opacity fade for scrims and background overlays.
 */
export const fadeMotion: TransitionFn = () => ({
  duration: duration(220),
  easing: expoOut,
  css: (t) => `opacity: ${t};`,
});
