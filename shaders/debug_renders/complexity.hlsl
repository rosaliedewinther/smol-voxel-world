RWTexture2D<uint> g_complexity;
RWTexture2D<unorm float4> screen_texture;

[numthreads(32, 32, 1)] void main(uint2 threadId
                                  : SV_DispatchThreadID)
{
    uint x = threadId.x;
    uint y = threadId.y;
    int2 pos = int2(x, y);
    float complexity_depth = log10(g_complexity[pos]+1)/10;
    screen_texture[pos] = float4(complexity_depth, complexity_depth, complexity_depth, 1);;
}