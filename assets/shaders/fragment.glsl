#version 450 core

uniform sampler2DArray uTexture;

in vec2 vTexCoords;
flat in float vLayer;
in vec3 vLocalPos;
in vec3 vLight;

out vec4 fragColor;

void main() {
    vec4 texColor = texture(uTexture, vec3(vTexCoords, vLayer));

    // --- CRITICAL FOR TRANSPARENCY ---
    if (texColor.a < 0.1) {
        discard;
    }

    // Apply a minimum ambient so blocks are never completely black
    float ambient = 0.05;
    vec3 lightColor = max(vLight, vec3(ambient));

    fragColor = vec4(texColor.rgb * lightColor, texColor.a);
}
