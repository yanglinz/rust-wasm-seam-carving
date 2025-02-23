export function getDemoImages() {
  const demoImages = {
    // https://images.unsplash.com/?q=80&w=4288&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D
    ["photo-1575252663512-b25714ec27f9"]: { alt: "Balloons in the sky" },

    // /?q=80&w=2670&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D
    ["photo-1533150613438-0b8ab7091235"]: { alt: "Birds in the sky" },

    // https://images.unsplash.com/?q=80&w=2666&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D
    ["photo-1519229642444-2c6c164c3aa5"]: { alt: "Waves on a beach" },

    // https://images.unsplash.com/?q=80&w=2670&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D
    ["photo-1569530593439-c5a3adda5204"]: { alt: "Hot balloons" },

    // https://images.unsplash.com/?q=80&w=1910&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D
    ["photo-1521245585918-35fd32bf376f"]: { alt: "Top down shot of beach" },

    // https://images.unsplash.com/?q=80&w=2670&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D
    ["photo-1542401886-65d6c61db217"]: { alt: "Desert landscape" },

    // https://images.unsplash.com/?q=80&w=2578&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D
    ["photo-1434873740857-1bc5653afda8"]: { alt: "Light house at night" },

    // https://images.unsplash.com/?q=80&w=2670&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=
    ["photo-1563996588193-435bc2b7f24f"]: { alt: "Castle by the sea" },
  };

  const screenWidth = Math.round(window.innerWidth * 0.8);
  const width = Math.min(1000, screenWidth);

  function getImageUrl(slug, width) {
    const qs = new URLSearchParams();
    qs.set('q', 80);
    qs.set('w', width);
    qs.set('auto', 'format');
    qs.set('fit', 'crop');
    qs.set('ixid', 'M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA==')
    return `https://images.unsplash.com/${slug}/?${qs.toString()}`;
  }

  Object.keys(demoImages).map((slug) => {
    demoImages[slug] = {
      ...demoImages[slug],
      url: getImageUrl(slug, width),
      previewUrl: getImageUrl(slug, 200)
    };
  });

  return demoImages;
}
