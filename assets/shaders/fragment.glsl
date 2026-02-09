#version 450 core

// --- Textures ---
uniform sampler2DArray u_TextureAtlas; // The block textures (grass, stone, etc.)
uniform sampler3D      u_Lightmap;     // The 3D RGB + SkyAccess texture for this chunk

// --- Scene Globals (Updated once per frame) ---
uniform vec3  u_EnvSkyColor;      // Current tint of the sunlight (Time Manager)
uniform float u_EnvSkyIntensity;  // Current brightness of the sun (Time Manager)
uniform float u_EnvAmbient;       // Base minimum light level (so nights aren't pitch black)

// --- Inputs from Vertex Shader ---
in vec2 vTexCoords;
flat in float vLayer;
in vec3 vLightmapUV;

out vec4 fragColor;

void main() {
    // 1. Sample the physical block texture
    vec4 texColor = texture(u_TextureAtlas, vec3(vTexCoords, vLayer));

    // Alpha testing for transparent/cutout blocks (leaves, glass)
    if (texColor.a < 0.1) {
        discard;
    }

    // 2. Sample the 3D Lightmap
    // .rgb = Block Light (Torches, Lava, Glowstone) - Range [0, 1]
    // .a   = Sky Accessibility (1.0 = full sky, 0.0 = completely roofed)
    vec4 lightSample = texture(u_Lightmap, vLightmapUV);

    vec3 blockLight = lightSample.rgb;
    float skyAccess = lightSample.a;

    // 3. Calculate dynamic Sunlight based on Environment Globals
    // This tints the sky-accessible areas by the current time-of-day color
    vec3 dynamicSunlight = skyAccess * u_EnvSkyColor * u_EnvSkyIntensity;

    // 4. Combine Light Sources
    // We use max() because a torch inside a dark house should provide its
    // full brightness regardless of the sun intensity outside.
    vec3 combinedLight = max(blockLight, dynamicSunlight);

    // 5. Apply Ambient Floor
    // Ensures we don't multiply the texture by [0, 0, 0]
    vec3 finalLight = max(combinedLight, vec3(u_EnvAmbient));

    // 6. Final Pixel Output
    fragColor = vec4(texColor.rgb * finalLight, texColor.a);
}