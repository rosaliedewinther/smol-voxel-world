#include "camera.hlsl"
#include "common.hlsl"

RWTexture2D<unorm float4> primary_ray_direction;
StructuredBuffer<Camera> camera_data;


float3 to_world_space(float2 shift, Camera camera){
    float3 position_horizontal = shift.x * camera.direction_side;
    float3 position_vertical = shift.y * camera.direction_up;
    return position_horizontal + position_vertical;
}

[numthreads(32, 32, 1)] void main(uint2 thread_id
                                  : SV_DispatchThreadID)
{
    uint x = thread_id.x;
    uint y = thread_id.y;
    uint2 pos = uint2(x, y);
    Camera camera = camera_data[0];
    uint screen_width = camera.screen_dimensions.x;
    uint screen_height = camera.screen_dimensions.y;
    uint random_state = wang_hash((1 + x + y * screen_width) * camera.random_seed);

    float3 sensor_center = camera.position - camera.direction * camera.focal_length;
    float horizontal_shift = ((float(x)/screen_width)-0.5) * camera.sensor_height * (screen_width/screen_height);
    float vertical_shift = ((float(y)/screen_height)-0.5) * camera.sensor_height;
    float3 position_on_sensor = sensor_center + to_world_space(float2(horizontal_shift, vertical_shift), camera);

    float2 pinhole_offset = random_point_circle(random_state) * (camera.focal_length/camera.aperture); 
    float3 pinhole_passthrough_position = camera.position + to_world_space(pinhole_offset, camera);

    float3 ray_direction = pinhole_passthrough_position - position_on_sensor;
    primary_ray_direction[pos] = float4(ray_direction, 0);
}