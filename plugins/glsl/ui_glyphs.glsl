// MIT License
//
// Copyright (c) 2019-2023 Tobias Pfeiffer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#version 460
#if defined VERT

layout (location = 0) in vec2 inPosition;
layout (location = 1) in uint inCharIndex;
layout (location = 2) in mat4 instModel;
layout (location = 6) in vec4 instColor;

layout (location = 0) out uint outCharIndex;
layout (location = 1) out mat4 outModel;
layout (location = 5) out vec4 outColor;
out gl_PerVertex { vec4 gl_Position; };

void main() {
	gl_Position  = vec4(inPosition, 0.0, 1.0);
	outCharIndex = inCharIndex;
	outModel     = instModel;
	outColor     = instColor;
}

#elif defined GEOM // --------------------------------------------------------------------------------------------------

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (points) in;
layout (location = 0) in uint inCharIndex[];
layout (location = 1) in mat4 inModel[];
layout (location = 5) in vec4 inColor[];
in gl_PerVertex {
	vec4 gl_Position;
} gl_in[];

layout (triangle_strip, max_vertices = 4) out;
layout (location = 0) out vec4 outColor;
layout (location = 1) out vec2 outUV;
out gl_PerVertex {
	vec4 gl_Position;
};

layout (binding = 1, std430) readonly buffer SSBOCodepoints {
	float codepoints[ ][8];
};

const uvec4[] indices = {
	uvec4(0, 1, 6, 7),
	uvec4(0, 3, 6, 5),
	uvec4(2, 1, 4, 7),
	uvec4(2, 3, 4, 5)
};

void main() {
	float[8] cp = codepoints[inCharIndex[0]];
	for (uint i = 0; i < 4; i++) {
		gl_Position = inModel[0] * vec4(gl_in[0].gl_Position.xy + vec2(cp[indices[i].x], cp[indices[i].y]), 0.0, 1.0);
		outColor    = inColor[0];
		outUV       = vec2(cp[indices[i].z], cp[indices[i].w]);
		EmitVertex();
	}
	EndPrimitive();
	
	// for diplaying the sdf image
	/*gl_Position = inModel[0] * vec4(-1.0, -1.0, 0.0, 1.0);
	outColor    = vec4(1.0);
	outUV       = vec2(0.0, 0.0);
	EmitVertex();
	
	gl_Position = inModel[0] * vec4(1.0, -1.0, 0.0, 1.0);
	outColor    = vec4(1.0);
	outUV       = vec2(1.0, 0.0);
	EmitVertex();
	
	gl_Position = inModel[0] * vec4(-1.0, 1.0, 0.0, 1.0);
	outColor    = vec4(1.0);
	outUV       = vec2(0.0, 1.0);
	EmitVertex();
	
	gl_Position = inModel[0] * vec4(1.0, 1.0, 0.0, 1.0);
	outColor    = vec4(1.0);
	outUV       = vec2(1.0, 1.0);
	EmitVertex();
	
	EndPrimitive();*/
}

#endif