// Algorithm "xor" from p. 4 of Marsaglia, "Xorshift RNGs"
uint random_uint(inout uint state) {
  	uint x = state;
  	x ^= x << 13;
  	x ^= x >> 17;
  	x ^= x << 5;
  	return state = x;
}

float random_float(inout uint state) {
  	return random_uint(state) * 2.3283064365387e-10f;
}

uint wang_hash(uint seed) {
    seed = (seed ^ 61) ^ (seed >> 16);
    seed *= 9;
    seed = seed ^ (seed >> 4);
    seed *= 0x27d4eb2d;
    seed = seed ^ (seed >> 15);
    return seed;
}

//https://www.shadertoy.com/view/ssGXDd
float2 random_point_circle_edge(inout uint state) {
    float u = random_float(state);
    float phi = 6.28318530718*u;
    return float2(cos(phi),sin(phi));
}

float2 random_point_circle(inout uint state) {
    return random_point_circle_edge(state)*sqrt(random_float(state));
}