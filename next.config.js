/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "export",
  trailingSlash: true,
  basePath: "/paws-and-preferences",
  assetPrefix: "/paws-and-preferences/",
  images: {
    unoptimized: true,
  },
};

module.exports = nextConfig;
