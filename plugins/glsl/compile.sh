glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_colored.vert.spv               -S vert -DVERT
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_colored_gradient.vert.spv      -S vert -DVERT -DGRADIENT
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_textured.vert.spv              -S vert -DVERT -DTEXTURED
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_textured_gradient.vert.spv     -S vert -DVERT -DTEXTURED -DGRADIENT
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_curves.vert.spv                -S vert -DVERT -DCURVE
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_curves_gradient.vert.spv       -S vert -DVERT -DCURVE -DGRADIENT
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_dyn_colored.vert.spv           -S vert -DVERT -DDYN
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_dyn_colored_gradient.vert.spv  -S vert -DVERT -DDYN -DGRADIENT
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_dyn_textured.vert.spv          -S vert -DVERT -DDYN -DTEXTURED
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_dyn_textured_gradient.vert.spv -S vert -DVERT -DDYN -DTEXTURED -DGRADIENT
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_colored.frag.spv               -S frag -DFRAG
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_textured.frag.spv              -S frag -DFRAG -DTEXTURED
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_sdf.frag.spv                   -S frag -DFRAG -DSDF
glslangValidator ./src-glsl/ui.glsl -V -o ./res/shader/ui_curves.frag.spv                -S frag -DFRAG -DCURVE
glslangValidator ./src-glsl/ui_glyphs.glsl -V -o ./res/shader/ui_glyphs.vert.spv         -S vert -DVERT
glslangValidator ./src-glsl/ui_glyphs.glsl -V -o ./res/shader/ui_glyphs.geom.spv         -S geom -DGEOM
glslangValidator ./src-glsl/ui_sdf_gen.comp.glsl -V -o ./res/shader/ui_sdf_gen.comp.spv
glslangValidator ./src-glsl/sdf_trace.comp.glsl -V -o ./res/shader/sdf_trace.comp.spv
glslangValidator ./src-glsl/sdf_gen.comp.glsl -V -o ./res/shader/sdf_gen.comp.spv        -DMESH
glslangValidator ./src-glsl/sdf_upd.comp.glsl -V -o ./res/shader/sdf_upd.comp.spv
