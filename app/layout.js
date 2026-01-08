import "./globals.css";

export const metadata = {
  title: "Paws and Preferences",
  description: "Discover your favourite cats",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
