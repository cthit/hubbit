/** @type {import('next').NextConfig} */
const nextConfig = {
  async rewrites() {
    return [
      {
        source: '/api/:ep*',
        destination: `${process.env.BACKEND_ADDRESS ?? ''}/api/:ep*`,
      },
    ];
  },
};

export default nextConfig;
