"use client";

import { useContext, useRef } from "react";
import { CatsSeenContext, DislikeCats, LikeCats } from "../page";

export function CatImage({ image }) {
  const [catsSeen, setCatsSeen] = useContext(CatsSeenContext);
  const [likeCats, setLikeCats] = useContext(LikeCats);
  const [dislikeCats, setDislikeCats] = useContext(DislikeCats);

  const touchStartX = useRef(null);
  const touchEndX = useRef(null);

  const handleTouchStart = (e) => {
    touchStartX.current = e.touches[0].clientX;
  };

  const handleTouchMove = (e) => {
    touchEndX.current = e.touches[0].clientX;
  };

  const handleTouchEnd = () => {
    if (!touchStartX.current || !touchEndX.current) return;

    const distance = touchStartX.current - touchEndX.current;
    const minSwipeDistance = 50; // Minimum distance for a swipe to be detected

    if (Math.abs(distance) > minSwipeDistance) {
      if (distance > 0) {
        setDislikeCats([...dislikeCats, image]);
      } else {
        setLikeCats([...likeCats, image]);
      }

      setCatsSeen(catsSeen + 1);
    }

    // Reset values
    touchStartX.current = null;
    touchEndX.current = null;
  };

  if (!image) {
    return (
      <div>
        <img className="w-[500px] h-[500px] object-contain" />
        <span>Loading cat...</span>
      </div>
    );
  }

  return (
    <div
      onTouchStart={handleTouchStart}
      onTouchMove={handleTouchMove}
      onTouchEnd={handleTouchEnd}
      className="select-none"
    >
      <img className="w-[500px] h-[500px] object-contain" src={image} />
    </div>
  );
}
