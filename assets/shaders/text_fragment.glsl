#version 450 core

uniform sampler2D uTexture;
uniform vec4 uColor;

in vec2 vUV;
out vec4 fragColor;

void main() {
    float alpha = texture(uTexture, vUV).r;
    if (alpha < 0.1) discard;
    fragColor = vec4(uColor.rgb, uColor.a * alpha);
}
