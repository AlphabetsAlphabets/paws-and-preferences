"use client";

import { useContext } from "react";
import { CatsSeenContext } from "./page";

export default function CatView({ image }) {
  return <img src={image} />;
}
