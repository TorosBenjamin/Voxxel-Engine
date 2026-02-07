#version 450 core

uniform sampler2DArray uTexture;

in vec2 vTexCoords;
flat in float vLayer;
in vec3 vLocalPos;
in float vSkyLight;
in float vBlockLight;

out vec4 fragColor;

void main() {
    vec4 texColor = texture(uTexture, vec3(vTexCoords, vLayer));

    // --- CRITICAL FOR TRANSPARENCY ---
    // If the texture pixel is transparent (alpha < 0.1),
    // we throw it away so we can see what's behind it.
    if (texColor.a < 0.1) {
        discard;
    }

    // Use the brighter of sky light and block light for illumination
    float light = max(vSkyLight, vBlockLight);

    // Apply a minimum ambient so blocks are never completely black
    light = max(light, 0.05);

    fragColor = vec4(texColor.rgb * light, texColor.a);
}
