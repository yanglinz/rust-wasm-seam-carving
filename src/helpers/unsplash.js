export function getDemoImages() {
  const demoImages = {
    ["yRjLihK35Yw"]: { alt: "Balloons in the sky" },
    ["e-S-Pe2EmrE"]: { alt: "Birds in the sky" },
    ["F6XKjhMNB14"]: { alt: "Waves on a beach" },
    ["KGwK6n7rxSg"]: { alt: "Hot balloons" },
    ["C9XgrB8hqBI"]: { alt: "Top down shot of beach" },
    ["pVr6wvUneMk"]: { alt: "Desert landscape" },
    ["Pn6iimgM-wo"]: { alt: "Light house at night" },
    ["4Oi1756LtF4"]: { alt: "Castle" },
  };

  const screenWidth = Math.round(window.innerWidth * 0.8);
  const width = Math.min(1000, screenWidth);
  const height = width / 2;
  Object.keys(demoImages).map((key, index) => {
    demoImages[key] = {
      ...demoImages[key],
      url: `https://source.unsplash.com/${key}/${width}x${height}`,
      previewUrl: `https://source.unsplash.com/${key}/200x200`,
    };
  });

  return demoImages;
}
