#include "../camera.hlsl"

RWTexture2D<unorm float4> primary_ray_direction;
RWTexture2D<unorm float4> screen_texture;

[numthreads(32, 32, 1)] void main(uint2 threadId
                                  : SV_DispatchThreadID)
{
    uint x = threadId.x;
    uint y = threadId.y;
    int2 pos = int2(x, y);
    screen_texture[pos] = (primary_ray_direction[pos]+1)/2;
}