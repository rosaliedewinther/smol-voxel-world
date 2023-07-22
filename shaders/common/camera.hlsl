struct Camera { 
    float3 position;
    float aperture;
    float3 direction;
    float focal_length;
    float3 direction_side;
    float sensor_height;
    float3 direction_up;
    uint random_seed;
    uint2 screen_dimensions;
};


float3 invert_ray_direction(float3 ray_direction) {
    float epsilon = 0.0001;
    return float3(
        ray_direction.x == 0 ? epsilon : ray_direction.x,
        ray_direction.y == 0 ? epsilon : ray_direction.y,
        ray_direction.z == 0 ? epsilon : ray_direction.z
    );
}