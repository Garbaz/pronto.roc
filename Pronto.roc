interface Pronto
    exposes [Init, 
             Draw, 
             circle,
             square]
    imports []

Init : [ Windowed U32 U32 Str, Fullscreen ]

DrawTask : [ Circle F64 F64 F64, Square F64 F64 F64 ]

Draw : List DrawTask

circle = \x, y, r -> Circle x y r
square = \x, y, r -> Circle x y r