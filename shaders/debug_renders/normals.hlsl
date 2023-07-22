#include "../common/random.hlsl"

RWTexture2D<snorm float4> g_normals;
RWTexture2D<unorm float4> screen_texture;

[numthreads(32, 32, 1)] void main(uint2 threadId
                                  : SV_DispatchThreadID)
{
    uint x = threadId.x;
    uint y = threadId.y;
    int2 pos = int2(x, y);
    screen_texture[pos] = (g_normals[pos]+1)/2;
}