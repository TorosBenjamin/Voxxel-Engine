#version 450 core

layout (location = 0) in uint aPacked;
layout (location = 1) in uint aLayer;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform vec3 uUVOffset;

out vec2 vTexCoords;
flat out float vLayer;
out vec3 vLocalPos;
out vec3 vLight;

void main() {
    // --- Word 1: Geometry (aPacked) ---
    // Layout: x:5, y:5, z:5, u:5, v:5, face:3, padding:4
    uint x = aPacked & 31u;
    uint y = (aPacked >> 5u) & 31u;
    uint z = (aPacked >> 10u) & 31u;
    uint u = (aPacked >> 15u) & 31u;
    uint v = (aPacked >> 20u) & 31u;
    uint face = (aPacked >> 25u) & 7u;

    // --- Word 2: Light + Texture Layer (aLayer) ---
    // Layout: r:8, g:8, b:8, texture_layer:8
    uint r = aLayer & 0xFFu;
    uint g = (aLayer >> 8u) & 0xFFu;
    uint b = (aLayer >> 16u) & 0xFFu;
    uint textureLayer = (aLayer >> 24u) & 0xFFu;

    vLayer = float(textureLayer);
    vLight = vec3(float(r), float(g), float(b)) / 255.0;

    // Chunk-local position for lightmap UV sampling
    vLocalPos = vec3(float(x), float(y), float(z));

    // --- Standard Position Logic ---
    gl_Position = projection * view * model * vec4(float(x), float(y), float(z), 1.0);

    vec2 worldUV;
    if (face == 0u || face == 1u) {
        worldUV = vec2(float(u) + uUVOffset.x, float(v) + uUVOffset.y);
    } else if (face == 2u || face == 3u) {
        worldUV = vec2(float(u) + uUVOffset.z, float(v) + uUVOffset.y);
    } else { // 4u || 5u
        worldUV = vec2(float(u) + uUVOffset.x, float(v) + uUVOffset.z);
    }

    vTexCoords = worldUV;
}
