#version 410

layout(location=0) in vec2 TexCoord;
layout(location=0) out vec4 FragColor;

uniform sampler2D Tex0;

void main() {
    FragColor = texture(Tex0, TexCoord);
}
