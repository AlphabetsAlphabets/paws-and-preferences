export async function fetchCatImage() {
  try {
    const response = await fetch("https://cataas.com/cat?json=true");
    if (response.ok) {
      const data = await response.json();
      return data.url;
    }
    throw new Error("Failed to fetch cat image");
  } catch (error) {
    console.error("Error fetching cat image:", error);
    return null;
  }
}
