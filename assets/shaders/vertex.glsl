#version 450 core

layout (location = 0) in uint aPacked;
layout (location = 1) in uint aLayer;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 uUVOffset;

out vec2 vTexCoords;
flat out float vLayer;
out vec3 vLightmapUV;

// Helper to get normals from your 3-bit face ID
vec3 getNormal(uint face) {
    vec3 normals[6] = vec3[](
    vec3( 0,  0,  1), vec3( 0,  0, -1), // North/South
    vec3( 1,  0,  0), vec3(-1,  0,  0), // East/West
    vec3( 0,  1,  0), vec3( 0, -1,  0)  // Up/Down
    );
    return normals[face];
}

void main() {
    uint x = aPacked & 31u;
    uint y = (aPacked >> 5u) & 31u;
    uint z = (aPacked >> 10u) & 31u;
    uint u = (aPacked >> 15u) & 31u;
    uint v = (aPacked >> 20u) & 31u;
    uint face = (aPacked >> 25u) & 7u;

    vLayer = float((aLayer >> 24u) & 0xFFu);

    // --- The Lightmap Nudge ---
    // Move the sampling point 0.1 units away from the face so we
    // definitely sample the light in the AIR, not inside the block.
    vec3 normal = getNormal(face);
    vLightmapUV = (vec3(float(x), float(y), float(z)) + 0.5 + (normal * 0.1)) / 32.0;

    gl_Position = projection * view * model * vec4(float(x), float(y), float(z), 1.0);

    // --- Standard Position Logic ---
    vec2 worldUV;
    if (face == 0u || face == 1u) {
        worldUV = vec2(float(u) + uUVOffset.x, float(v) + uUVOffset.y);
    } else if (face == 2u || face == 3u) {
        worldUV = vec2(float(u) + uUVOffset.z, float(v) + uUVOffset.y);
    } else {
        worldUV = vec2(float(u) + uUVOffset.x, float(v) + uUVOffset.z);
    }

    vTexCoords = worldUV;
}