"use client";

import { useContext, useRef } from "react";
import { LikeCats, CatsSeenContext, DislikeCats } from "./page";
import { CatImage } from "./components/cat_image";
import LikeDislike from "./components/like_dislike";

export default function CatView({ image }) {
  return (
    <div>
      <CatImage image={image} />
      <LikeDislike image={image} />
    </div>
  );
}
