export function CircularProgress() {
  const circumference = STROKE_WIDTH * Math.PI * RADIUS;

  return (
    <svg className="-rotate-90 h-3 w-3 md:h-4 md:w-4" viewBox="0 0 20 20">
      <title>Circular Progress</title>
      <circle
        cx="10"
        cy="10"
        r={RADIUS}
        stroke="currentColor"
        strokeWidth={STROKE_WIDTH}
        fill="none"
        className="text-gray-600"
      />
      <circle
        cx="10"
        cy="10"
        r={RADIUS}
        stroke="currentColor"
        strokeWidth="2"
        fill="none"
        className="animate-progress text-white"
        strokeDasharray={circumference}
        strokeLinecap="round"
        style={{ strokeDashoffset: circumference }}
      />
    </svg>
  );
}

export const RADIUS = 8;
export const STROKE_WIDTH = 2;

export const animateProgressCss = `
@keyframes progress {
  from {
    stroke-dashoffset: ${2 * Math.PI * RADIUS};
  }
  to {
    stroke-dashoffset: 0;
  }
}
.animate-progress {
  animation: progress 3s linear infinite;
}
`;
