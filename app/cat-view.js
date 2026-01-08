"use client";

import { useContext } from "react";
import { CatsSeenContext } from "./page";

export default function CatView({ image }) {
  return (
    <div>
      <img className="w-[500px] h-[500px] object-contain" src={image} />
    </div>
  );
}
