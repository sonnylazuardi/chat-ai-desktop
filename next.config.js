/** @type {import('next').NextConfig} */

const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  images: {
    unoptimized: true,
  },
};

module.exports = {
  ...nextConfig,
  async redirects() {
    return [
      {
        source: "/",
        destination: "https://chat.openai.com/chat",
        permanent: false,
        basePath: false,
      },
    ];
  },
};
