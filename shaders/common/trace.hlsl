#pragma once

float sdSphere(float3 p, float d) { return length(p) - d; } 

float sdBox( float3 p, float3 b ){
  float3 d = abs(p) - b;
  return min(max(d.x,max(d.y,d.z)),0.0) + length(max(d,0.0));
}

bool getVoxel(float3 position) {
	float3 p = position + float3(0.5, 0.5, 0.5);
	float d = min(max(-sdSphere(p, 7.5), sdBox(p, float3(6.0, 6.0, 6.0))), -sdSphere(p, 50.0));
	return d < 0.0;
}

void trace(
    in float3 origin, 
    in float3 direction, 
    in uint traversing_material,
    out float3 voxel_normal, 
    out float depth, 
    out uint material, 
    out uint complexity
    ){

    int3 mapPos = int3(floor(origin + 0.));
	float3 deltaDist = abs(1 / direction);
	int3 rayStep = int3(sign(direction));
	float3 sideDist = (sign(direction) * (float3(mapPos) - origin) + (sign(direction) * 0.5) + 0.5) * deltaDist; 

	bool3 mask;

    uint i = 0;
	for (; i < 10000; i++) {
		if (getVoxel(mapPos)) break;
        mask.x = sideDist.x <= min(sideDist.y, sideDist.z);
        mask.y = sideDist.y <= min(sideDist.z, sideDist.x);
        mask.z = sideDist.z <= min(sideDist.x, sideDist.y);
        sideDist += float3(mask) * deltaDist;
        mapPos += int3(float3(mask)) * rayStep;
	}
    voxel_normal = float3(float3(mask)*float3(rayStep));
    depth = length(float3(mask) * (sideDist - deltaDist)) / length(direction);
    material = 1;
    complexity = i;
}