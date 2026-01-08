"use client";

import { useContext } from "react";
import { CatsSeenContext } from "./page";

export default function CatView({ image }) {
  if (!image) {
    return (
      <div>
        <img className="w-[500px] h-[500px] object-contain" />
        <span>Loading cat...</span>
      </div>
    );
  }

  return (
    <div>
      <img className="w-[500px] h-[500px] object-contain" src={image} />
    </div>
  );
}
