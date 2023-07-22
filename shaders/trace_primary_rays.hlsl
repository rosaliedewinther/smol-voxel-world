#include "common/camera.hlsl"
#include "common/random.hlsl"
#include "common/trace.hlsl"

RWTexture2D<unorm float4> primary_ray_direction;
StructuredBuffer<Camera> camera_data;
RWTexture2D<unorm float4> g_normal;
RWTexture2D<float> g_depth;
RWTexture2D<uint> g_material;
RWTexture2D<uint> g_complexity;

[numthreads(32, 32, 1)] void main(uint2 threadId
                                  : SV_DispatchThreadID)
{
    uint x = threadId.x;
    uint y = threadId.y;
    int2 pos = int2(x, y);
    Camera camera = camera_data[0];
    if (x >= camera.screen_dimensions.x || y >= camera.screen_dimensions.y) return;
    float3 origin = camera.position;
    float3 direction = primary_ray_direction[pos].xyz;
    uint traversing_material = 0;
    float3 normal;
    float depth;
    uint material;
    uint complexity;

    trace(
        origin,
        direction,
        traversing_material,
        normal,
        depth,
        material,
        complexity
    );

    g_normal[pos] = float4(normal, 0);
    g_depth[pos] = depth;
    g_material[pos] = material;
    g_complexity[pos] = complexity;
}