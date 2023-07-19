RWTexture2D<float> g_depth;
RWTexture2D<unorm float4> screen_texture;

[numthreads(32, 32, 1)] void main(uint2 threadId
                                  : SV_DispatchThreadID)
{
    uint x = threadId.x;
    uint y = threadId.y;
    int2 pos = int2(x, y);
    float shifted_depth = float(log10(g_depth[pos]+1))/10;
    screen_texture[pos] = float4(shifted_depth, shifted_depth, shifted_depth, 1);
}